"""Build Muse with Blender 4.3 or newer, without external assets.

Run from any directory:
    /usr/bin/blender --background --python assets/build_mascot.py

Coordinates are right-handed, Z up; the character faces -Y. Lengths are meters.
The OBJ contains all scene geometry, including the ground, in these coordinates.
The JSON records the primitive recipes and material settings for other renderers.
Fur is a hair particle system on each plush object. It lives in the .blend and is
described in scene.json, but the OBJ cannot carry it. Furred objects have their
primitive scale baked into the mesh (recipe "baked_scale") so strands are not stretched.
"""

import json
import math
import time
from pathlib import Path

import bpy
from mathutils import Vector


OUT = Path(__file__).resolve().parent
RECIPES = {}
MATERIALS = {}
FUR_SYSTEMS = {}

# Body silhouette (radius, height) traced from reference/muse-mascot.jpg at
# 3.57 m figure height, minus the fur pile: a broad flat crown, near-straight
# sides that widen toward the belly, and a flat base between the legs.
BODY_PROFILE = [
    (0.0, 0.42), (0.40, 0.43), (0.62, 0.47), (0.76, 0.54), (0.86, 0.64),
    (0.93, 0.75), (0.98, 0.86), (1.01, 0.97), (1.028, 1.08), (1.028, 1.19),
    (1.005, 1.39), (0.955, 1.85), (0.915, 2.28), (0.905, 2.50), (0.895, 2.61),
    (0.862, 2.83), (0.830, 2.94), (0.795, 3.05), (0.745, 3.16), (0.680, 3.27),
    (0.565, 3.38), (0.370, 3.49), (0.0, 3.545),
]
BODY_DEPTH = 0.63  # front-to-back radius as a fraction of the side radius

# Inset squircle face, as seen from the front: center height, half-width,
# half-height, and superellipse exponent.
FACE = {"z": 2.75, "half_width": 0.645, "half_height": 0.505, "exponent": 2.6}

FUR = {
    "length": 0.05, "root_radius": 0.0035, "tip_radius_factor": 0.12,
    "children_total": 120000, "children_per_parent": 60,
    "down_bias": -0.72, "random": 0.12, "clump": 0.5, "clump_shape": 0.3,
    "roughness_uniform": 0.0035, "roughness_end": 0.010, "roughness_random": 0.004,
    "kink": "NO", "kink_amplitude": 0.0, "kink_frequency": 2.0,
    "melanin": 0.08, "melanin_redness": 0.3, "tint": "#e2b39c",
    "roughness": 0.32, "radial_roughness": 0.4, "random_roughness": 0.3,
    "coat": 0.0, "ior": 1.55,
}

# Feature placement, matched to the reference in compare-and-fix passes.
RIG = {
    "eye_x": 0.31, "eye_z": 2.74, "eye_scale": (0.08, 0.042, 0.079),
    "smile_half_width": 0.092, "smile_z": 2.58, "smile_sag": 0.045,
    "cheek_x": 0.35, "cheek_z": 2.56, "cheek_scale": (0.25, 0.03, 0.17),
    "arm_x": 0.86, "arm_scale": (0.22, 0.26, 0.50),
    "hand_x": 0.64, "hand_y": -0.76, "hand_z": 1.74, "hand_scale": (0.34, 0.30, 0.25),
    "thumb_x": 0.63, "thumb_y": -0.82, "thumb_z": 1.97, "thumb_scale": (0.15, 0.16, 0.19),
    "thumb_tilt": 0.15,
    "leg_x": 0.455, "leg_radius": 0.315,
    "orb_center": (0, -0.93, 1.895), "orb_spill": 22,
}

LIGHTS = [
    ("Key softbox", (4.4, -2.4, 7.0), (0, 0, 2.0), 1300, 3.6, "#fff4e9"),
    ("Fill softbox", (-2.6, -4.6, 3.2), (0, 0, 1.8), 10, 3.4, "#e9f1ff"),
    ("Rim softbox", (1.6, 2.6, 5.4), (0, 0, 2.1), 340, 3.0, "#fff8f0"),
]

CAMERA = {"position": (0, -7.8, 2.6), "target": (0, -0.03, 1.71), "lens": 62.3}

# Only the orb's white-hot rim should bloom. A lower threshold also blooms the
# key-lit skin and the high-key cyc, which veils the black eyes in brown haze.
GLARE = {"type": "FOG_GLOW", "threshold": 3.0, "size": 7,
         "orb_bloom": {"type": "FOG_GLOW", "threshold": 0.9, "size": 8, "mask": "object index 1 (orb parts); glow added outside the orb only"}}

RENDER = {"samples": 512, "exposure": 0.3, "world_strength": 0.25, "cyc_glow": 0.92, "device": "CPU"}


def linear_channel(value):
    return value / 12.92 if value <= 0.04045 else ((value + 0.055) / 1.055) ** 2.4


def rgb(hex_value):
    value = hex_value.lstrip("#")
    return tuple(linear_channel(int(value[i:i + 2], 16) / 255) for i in (0, 2, 4))


def material(name, hex_color, roughness, *, emission=0, texture=False,
             transmission=0, ior=1.45, metallic=0, sheen=0):
    color = rgb(hex_color)
    mat = bpy.data.materials.new(name)
    mat.use_nodes = True
    mat.diffuse_color = (*color, 1)
    shader = mat.node_tree.nodes.get("Principled BSDF")
    for socket, value in {
        "Base Color": (*color, 1), "Roughness": roughness,
        "Metallic": metallic, "Transmission Weight": transmission, "IOR": ior,
        "Sheen Weight": sheen, "Emission Color": (*color, 1),
        "Emission Strength": emission,
    }.items():
        shader.inputs[socket].default_value = value
    spec = {
        "name": name, "base_color": list(color), "base_color_srgb_hex": hex_color,
        "roughness": roughness, "metallic": metallic, "transmission": transmission,
        "ior": ior, "sheen": sheen,
        "emission": {"color": list(color), "strength": emission},
    }
    if texture:
        nodes, links = mat.node_tree.nodes, mat.node_tree.links
        coord = nodes.new("ShaderNodeTexCoord")
        noise = nodes.new("ShaderNodeTexNoise")
        noise.inputs["Scale"].default_value = 125
        noise.inputs["Detail"].default_value = 2
        noise.inputs["Roughness"].default_value = 0.7
        links.new(coord.outputs["Object"], noise.inputs["Vector"])
        bump = nodes.new("ShaderNodeBump")
        bump.inputs["Strength"].default_value = 0.2
        bump.inputs["Distance"].default_value = 0.018
        links.new(noise.outputs["Fac"], bump.inputs["Height"])
        links.new(bump.outputs["Normal"], shader.inputs["Normal"])
        spec["surface_texture"] = {
            "type": "noise_bump", "coordinates": "object", "scale": 125,
            "detail": 2, "roughness": 0.7, "strength": 0.2, "distance": 0.018,
        }
    MATERIALS[name] = spec
    return mat


def finish(obj, name, mat, recipe):
    obj.name = name
    obj.data.materials.append(mat)
    for polygon in obj.data.polygons:
        polygon.use_smooth = True
    RECIPES[name] = recipe
    return obj


def sphere(name, position, scale, mat, segments=64, rings=40):
    bpy.ops.mesh.primitive_uv_sphere_add(
        segments=segments, ring_count=rings, radius=1, location=position)
    obj = bpy.context.object
    obj.scale = scale
    return finish(obj, name, mat, {"primitive": "uv_sphere", "radius": 1,
                                  "segments": segments, "rings": rings})


def tube(name, points, radius, mat, bevel_resolution=4):
    bpy.ops.object.select_all(action="DESELECT")
    curve = bpy.data.curves.new(name, "CURVE")
    curve.dimensions = "3D"
    curve.resolution_u = 1
    curve.bevel_depth = radius
    curve.bevel_resolution = bevel_resolution
    curve.use_fill_caps = True
    spline = curve.splines.new("POLY")
    spline.points.add(len(points) - 1)
    for point, position in zip(spline.points, points):
        point.co = (*position, 1)
    obj = bpy.data.objects.new(name, curve)
    bpy.context.collection.objects.link(obj)
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.convert(target="MESH")
    obj = bpy.context.object
    obj.select_set(False)
    return finish(obj, name, mat, {
        "primitive": "polyline_tube", "points": [list(p) for p in points],
        "radius": radius, "bevel_resolution": bevel_resolution,
    })


def aim(obj, target):
    obj.rotation_euler = (Vector(target) - obj.location).to_track_quat("-Z", "Y").to_euler()


def area(name, position, target, power, size, hex_color):
    data = bpy.data.lights.new(name, "AREA")
    data.energy = power
    data.shape = "DISK"
    data.size = size
    data.color = rgb(hex_color)
    obj = bpy.data.objects.new(name, data)
    bpy.context.collection.objects.link(obj)
    obj.location = position
    aim(obj, target)
    return obj


def catmull_rom(points, steps):
    out = []
    padded = [points[0]] + points + [points[-1]]
    for i in range(1, len(padded) - 2):
        p0, p1, p2, p3 = (Vector(padded[i + k]) for k in (-1, 0, 1, 2))
        for step in range(steps):
            t = step / steps
            out.append(tuple(0.5 * (2 * p1 + (p2 - p0) * t + (2 * p0 - 5 * p1 + 4 * p2 - p3) * t * t
                                    + (3 * p1 - p0 - 3 * p2 + p3) * t ** 3)))
    out.append(points[-1])
    return out


BODY_RINGS = catmull_rom(BODY_PROFILE, 6)


def body_radius(z):
    # Side radius of the body at height z, from the smoothed profile.
    for (r0, z0), (r1, z1) in zip(BODY_RINGS, BODY_RINGS[1:]):
        if z0 <= z <= z1 and z1 > z0:
            return r0 + (r1 - r0) * (z - z0) / (z1 - z0)
    return 0.0


def body_front(x, z):
    # Y of the body's front surface (the character faces -Y) at (x, z).
    r = body_radius(z)
    return -r * BODY_DEPTH * math.sqrt(max(0.0, 1 - (x / r) ** 2)) if r else 0.0


def lathe(name, profile, segments, depth=1.0):
    # Revolve a (radius, z) profile, bottom pole first, around +Z.
    verts, faces = [(0, 0, profile[0][1])], []
    for r, z in profile[1:-1]:
        for segment in range(segments):
            angle = 2 * math.pi * segment / segments
            verts.append((r * math.cos(angle), depth * r * math.sin(angle), z))
    top = len(verts)
    verts.append((0, 0, profile[-1][1]))
    rings = len(profile) - 2
    for segment in range(segments):
        faces.append((0, 1 + (segment + 1) % segments, 1 + segment))
    for ring in range(rings - 1):
        start = 1 + ring * segments
        for segment in range(segments):
            nxt = (segment + 1) % segments
            faces.append((start + segment, start + nxt,
                          start + segments + nxt, start + segments + segment))
    start = 1 + (rings - 1) * segments
    for segment in range(segments):
        faces.append((start + segment, start + (segment + 1) % segments, top))
    mesh = bpy.data.meshes.new(name)
    mesh.from_pydata(verts, [], faces)
    mesh.update()
    obj = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(obj)
    return obj


def make_body(mat):
    obj = lathe("Body", BODY_RINGS, 128, BODY_DEPTH)
    return finish(obj, "Body", mat, {
        "primitive": "profile_lathe", "segments": 128, "depth_scale_y": BODY_DEPTH,
        "profile_radius_z": [list(p) for p in BODY_PROFILE],
        "profile_smoothing": "catmull_rom, 6 samples per span",
        "source": "silhouette traced from reference/muse-mascot.jpg",
    })


def superellipse(angle, exponent):
    c, s = abs(math.cos(angle)), abs(math.sin(angle))
    return (c ** exponent + s ** exponent) ** (-1 / exponent)


def face_point(d, angle):
    # Map polar (d, angle) in the face disc to the front-view (x, z) plane.
    k = d * superellipse(angle, FACE["exponent"])
    return k * FACE["half_width"] * math.cos(angle), FACE["z"] + k * FACE["half_height"] * math.sin(angle)


def face_offset(d):
    # The face rides just above the head surface with a soft, puffy dome and
    # tucks under at its rim, so the fur hood overlaps the edge.
    lift = 0.004 + 0.03 * (1 - min(d, 1) ** 3)
    return lift - 0.05 * max(0.0, (d - 0.93) / 0.07) ** 2


def face_y(x, z, d=0.0):
    return body_front(x, z) - face_offset(d)


def face_normal(x, z, d):
    step = 0.005
    fx = (face_y(x + step, z, d) - face_y(x - step, z, d)) / (2 * step)
    fz = (face_y(x, z + step, d) - face_y(x, z - step, d)) / (2 * step)
    return Vector((fx, -1, fz)).normalized()


def make_face(mat):
    rings, segments = 40, 144
    verts, faces = [], []
    x, z = face_point(0, 0)
    verts.append((x, face_y(x, z, 0), z))
    for ring in range(1, rings + 1):
        d = ring / rings
        for segment in range(segments):
            x, z = face_point(d, 2 * math.pi * segment / segments)
            verts.append((x, face_y(x, z, d), z))
    for segment in range(segments):
        faces.append((0, 1 + segment, 1 + (segment + 1) % segments))
    for ring in range(rings - 1):
        start = 1 + ring * segments
        for segment in range(segments):
            nxt = (segment + 1) % segments
            faces.append((start + segment, start + segments + segment,
                          start + segments + nxt, start + nxt))
    mesh = bpy.data.meshes.new("Face")
    mesh.from_pydata(verts, [], faces)
    mesh.update()
    obj = bpy.data.objects.new("Face", mesh)
    bpy.context.collection.objects.link(obj)
    return finish(obj, "Face", mat, {
        "primitive": "squircle_patch_on_body", "rings": rings, "segments": segments,
        "center_z": FACE["z"], "half_width": FACE["half_width"],
        "half_height": FACE["half_height"], "exponent": FACE["exponent"],
        "offset": "lift above body = 0.004 + 0.03*(1-d^3), tucked 0.05 under the rim past d=0.93",
    })


def make_leg(name, position, mat, radius=0.29, fillet=0.11, top=0.95):
    # A short cylinder with a rounded, nearly flat sole; the top hides in the body.
    profile = [(0, 0)]
    for i in range(1, 7):
        profile.append(((radius - fillet) * i / 6, 0))
    for i in range(1, 9):
        a = (math.pi / 2) * i / 8
        profile.append((radius - fillet + fillet * math.sin(a), fillet * (1 - math.cos(a))))
    shaft_top = top - radius * 0.6
    for i in range(1, 7):
        profile.append((radius, fillet + (shaft_top - fillet) * i / 6))
    for i in range(1, 9):
        a = (math.pi / 2) * i / 8
        profile.append((radius * math.cos(a), shaft_top + radius * 0.6 * math.sin(a)))
    profile[-1] = (0, top)
    obj = lathe(name, profile, 64)
    obj.location = position
    return finish(obj, name, mat, {
        "primitive": "capsule_flat_sole", "radius": radius, "sole_fillet": fillet,
        "height": top, "visible_height_below_body": 0.42, "base_z": 0,
    })


def bake_scale(obj):
    RECIPES[obj.name]["baked_scale"] = list(obj.scale)
    bpy.ops.object.select_all(action="DESELECT")
    obj.select_set(True)
    bpy.context.view_layer.objects.active = obj
    bpy.ops.object.transform_apply(location=False, rotation=False, scale=True)
    obj.select_set(False)


def fur_material():
    mat = bpy.data.materials.new("Long pile faux fur")
    mat.use_nodes = True
    nodes, links = mat.node_tree.nodes, mat.node_tree.links
    nodes.remove(nodes.get("Principled BSDF"))
    hair = nodes.new("ShaderNodeBsdfHairPrincipled")
    hair.parametrization = "MELANIN"
    tint = rgb(FUR["tint"])
    for socket, value in {
        "Melanin": FUR["melanin"], "Melanin Redness": FUR["melanin_redness"],
        "Tint": (*tint, 1), "Roughness": FUR["roughness"],
        "Radial Roughness": FUR["radial_roughness"], "Coat": FUR["coat"],
        "IOR": FUR["ior"], "Random Roughness": FUR["random_roughness"],
    }.items():
        hair.inputs[socket].default_value = value
    links.new(hair.outputs[0], nodes.get("Material Output").inputs["Surface"])
    mat.diffuse_color = (*tint, 1)
    MATERIALS[mat.name] = {
        "name": mat.name, "type": "principled_hair_chiang", "parametrization": "melanin",
        "base_color_srgb_hex": FUR["tint"], "base_color": list(tint),
        **{k: FUR[k] for k in ("melanin", "melanin_redness", "tint", "roughness",
                               "radial_roughness", "random_roughness", "coat", "ior")},
    }
    return mat


def add_fur(obj, mat, parents, weight=None):
    obj.data.materials.append(mat)
    modifier = obj.modifiers.new("Fur", "PARTICLE_SYSTEM")
    system = modifier.particle_system
    system.seed = len(FUR_SYSTEMS) * 7 + 3
    if weight:
        group = obj.vertex_groups.new(name="Fur density")
        for vertex in obj.data.vertices:
            group.add([vertex.index], weight(obj.matrix_world @ vertex.co), "REPLACE")
        system.vertex_group_density = group.name
    part = system.settings
    part.name = obj.name + " fur"
    part.type = "HAIR"
    part.use_advanced_hair = True
    part.count = parents
    part.emit_from = "FACE"
    part.use_emit_random = True
    part.use_even_distribution = True
    # Strands lie down and fall toward the floor, like brushed long pile. With
    # advanced hair, strand length is 4 x |normal + align| emission velocity, so
    # scale the velocities by the length; a length vertex group evens out the
    # result between up-facing and down-facing surfaces.
    shortest = 1 + FUR["down_bias"]
    unit = FUR["length"] / (4 * shortest)
    part.normal_factor = unit
    part.object_align_factor = (0, 0, FUR["down_bias"] * unit)
    part.factor_random = FUR["random"] * unit
    lengths = obj.vertex_groups.new(name="Fur length")
    rotation = obj.matrix_world.to_3x3()
    for vertex in obj.data.vertices:
        normal = (rotation @ vertex.normal).normalized()
        reach = (normal + Vector((0, 0, FUR["down_bias"]))).length
        lengths.add([vertex.index], min(1.0, shortest / max(reach, 1e-3)), "REPLACE")
    system.vertex_group_length = lengths.name
    part.hair_step = 5
    part.display_step = 3
    part.render_step = 4
    part.use_hair_bspline = True
    part.child_type = "INTERPOLATED"
    part.child_percent = 4
    part.rendered_child_count = FUR["children_per_parent"]
    part.virtual_parents = 0
    part.clump_factor = FUR["clump"]
    part.clump_shape = FUR["clump_shape"]
    part.roughness_1 = FUR["roughness_uniform"]
    part.roughness_1_size = 1.0
    part.roughness_endpoint = FUR["roughness_end"]
    part.roughness_end_shape = 1.0
    part.roughness_2 = FUR["roughness_random"]
    part.roughness_2_size = 1.0
    part.roughness_2_threshold = 0.4
    part.kink = FUR["kink"]
    part.kink_amplitude = FUR["kink_amplitude"]
    part.kink_frequency = FUR["kink_frequency"]
    part.kink_shape = 0.0
    part.radius_scale = FUR["root_radius"]
    part.root_radius = 1.0
    part.tip_radius = FUR["tip_radius_factor"]
    part.shape = 0.0
    part.use_close_tip = True
    part.material_slot = mat.name
    FUR_SYSTEMS[obj.name] = {
        "parents": parents, "children_per_parent": FUR["children_per_parent"],
        "rendered_strands": parents * FUR["children_per_parent"],
        "density_vertex_group": bool(weight),
    }
    return system


def surface_area(obj):
    return sum(polygon.area for polygon in obj.data.polygons)


def soften_edges(mat):
    # Fade the material out toward its rim so blush reads as airbrushed, not a decal.
    nodes, links = mat.node_tree.nodes, mat.node_tree.links
    shader, output = nodes.get("Principled BSDF"), nodes.get("Material Output")
    coord = nodes.new("ShaderNodeTexCoord")
    # Distance in the local X/Z plane, so the fade is radial as seen from the front.
    flatten = nodes.new("ShaderNodeVectorMath")
    flatten.operation = "MULTIPLY"
    flatten.inputs[1].default_value = (1, 0, 1)
    links.new(coord.outputs["Object"], flatten.inputs[0])
    distance = nodes.new("ShaderNodeVectorMath")
    distance.operation = "LENGTH"
    links.new(flatten.outputs["Vector"], distance.inputs[0])
    ramp = nodes.new("ShaderNodeMapRange")
    ramp.inputs["From Min"].default_value = 0.0
    ramp.inputs["From Max"].default_value = 0.95
    ramp.inputs["To Min"].default_value = 1
    ramp.inputs["To Max"].default_value = 0
    links.new(distance.outputs["Value"], ramp.inputs["Value"])
    clear = nodes.new("ShaderNodeBsdfTransparent")
    mix = nodes.new("ShaderNodeMixShader")
    links.new(ramp.outputs["Result"], mix.inputs["Fac"])
    links.new(clear.outputs["BSDF"], mix.inputs[1])
    links.new(shader.outputs["BSDF"], mix.inputs[2])
    links.new(mix.outputs["Shader"], output.inputs["Surface"])
    MATERIALS[mat.name]["edge_fade"] = {
        "type": "radial_alpha_xz", "coordinates": "object",
        "opaque_within": 0.0, "clear_beyond": 0.95,
    }


def make_nebula():
    mat = material("Nebula purple blue pink", "#9568ed", 0.42, emission=1.1)
    nodes, links = mat.node_tree.nodes, mat.node_tree.links
    shader = nodes.get("Principled BSDF")
    coord = nodes.new("ShaderNodeTexCoord")
    warp = nodes.new("ShaderNodeTexNoise")
    warp.inputs["Scale"].default_value = 1.7
    warp.inputs["Detail"].default_value = 4
    warp.inputs["Roughness"].default_value = 0.7
    cloud = nodes.new("ShaderNodeTexNoise")
    cloud.inputs["Scale"].default_value = 2.3
    cloud.inputs["Detail"].default_value = 3
    cloud.inputs["Roughness"].default_value = 0.65
    links.new(warp.outputs["Color"], cloud.inputs["Vector"])
    ramp = nodes.new("ShaderNodeValToRGB")
    stops = [(0.22, "#1a0b4f"), (0.36, "#3b1fd0"), (0.44, "#e02fb8"),
             (0.51, "#4a2cf0"), (0.59, "#0b6fe8"), (0.70, "#3fd2f5")]
    ramp.color_ramp.elements.remove(ramp.color_ramp.elements[1])
    for index, (position, color) in enumerate(stops):
        element = ramp.color_ramp.elements[0] if index == 0 else ramp.color_ramp.elements.new(position)
        element.position = position
        element.color = (*rgb(color), 1)
    # Drift the ramp diagonally, as in the reference: magenta upper left,
    # violet-blue center, cyan lower right.
    axes = nodes.new("ShaderNodeSeparateXYZ")
    links.new(coord.outputs["Object"], axes.inputs["Vector"])
    diagonal = nodes.new("ShaderNodeMath")
    diagonal.operation = "SUBTRACT"
    links.new(axes.outputs["X"], diagonal.inputs[0])
    links.new(axes.outputs["Z"], diagonal.inputs[1])
    bias = nodes.new("ShaderNodeMath")
    bias.operation = "MULTIPLY_ADD"
    bias.inputs[1].default_value = 0.12
    links.new(diagonal.outputs["Value"], bias.inputs[0])
    links.new(cloud.outputs["Fac"], bias.inputs[2])
    links.new(bias.outputs["Value"], ramp.inputs["Fac"])
    # The core is a glowing volume; its surface only adds the white-hot rim.
    # The noise is stretched along the view axis (-Y) so each ray sees one
    # color region instead of averaging magenta, blue and cyan to lavender.
    to_generated = nodes.new("ShaderNodeMapping")
    to_generated.inputs["Location"].default_value = (0.5, 0.5, 0.5)
    to_generated.inputs["Scale"].default_value = (0.5, 0.1, 0.5)
    links.new(coord.outputs["Object"], to_generated.inputs["Vector"])
    links.new(to_generated.outputs["Vector"], warp.inputs["Vector"])
    volume = nodes.new("ShaderNodeVolumePrincipled")
    volume.inputs["Color"].default_value = (0, 0, 0, 1)
    volume.inputs["Density"].default_value = 8.0
    volume.inputs["Emission Strength"].default_value = 11.0
    links.new(ramp.outputs["Color"], volume.inputs["Emission Color"])
    output = nodes.get("Material Output")
    links.new(volume.outputs["Volume"], output.inputs["Volume"])
    facing = nodes.new("ShaderNodeLayerWeight")
    facing.inputs["Blend"].default_value = 0.5
    rim = nodes.new("ShaderNodeValToRGB")
    rim.color_ramp.elements[0].position = 0.72
    rim.color_ramp.elements[1].position = 0.97
    rim.color_ramp.elements[1].color = (*rgb("#f2e6ff"), 1)
    links.new(facing.outputs["Facing"], rim.inputs["Fac"])
    glow = nodes.new("ShaderNodeEmission")
    glow.inputs["Strength"].default_value = 5.0
    links.new(rim.outputs["Color"], glow.inputs["Color"])
    clear = nodes.new("ShaderNodeBsdfTransparent")
    surface = nodes.new("ShaderNodeAddShader")
    links.new(clear.outputs["BSDF"], surface.inputs[0])
    links.new(glow.outputs["Emission"], surface.inputs[1])
    links.new(surface.outputs["Shader"], output.inputs["Surface"])
    nodes.remove(shader)
    MATERIALS[mat.name]["procedural_color"] = {
        "type": "warped_noise_color_ramp", "coordinates": "object*(0.5,0.1,0.5)+0.5 (stretched along the view axis)",
        "ramp_bias": "fac + 0.12*(x - z) in object space",
        "drives": "principled_volume_emission_color",
        "volume": {"scatter_color": "black (absorb and emit only)", "density": 8.0, "emission_strength": 11.0},
        "surface": "transparent plus facing-rim emission (strength 5.0)",
        "warp": {"scale": 1.7, "detail": 4, "roughness": 0.7},
        "noise": {"scale": 2.3, "detail": 3, "roughness": 0.65},
        "stops": [{"position": p, "color": list(rgb(c)), "srgb_hex": c} for p, c in stops],
        "rim": {"type": "layer_weight_facing_emission", "blend": 0.5, "from": 0.72, "to": 0.97,
                "color_srgb_hex": "#f2e6ff", "strength": 5.0},
    }
    return mat


def write_scene(scene, camera, target):
    geometry, lights = [], []
    depsgraph = bpy.context.evaluated_depsgraph_get()
    for obj in sorted(scene.objects, key=lambda item: item.name):
        transform = {
            "position": list(obj.location), "rotation_euler_xyz": list(obj.rotation_euler),
            "scale": list(obj.scale),
            "matrix_world": [list(row) for row in obj.matrix_world],
        }
        if obj.type == "MESH":
            mats = [MATERIALS[slot.material.name] for slot in obj.material_slots if slot.material]
            evaluated = obj.evaluated_get(depsgraph)
            geometry.append({
                "name": obj.name, "type": obj.type, **transform,
                "geometry": RECIPES[obj.name], "materials": mats,
                "material": mats[0] if mats else None,
                "vertices": len(evaluated.data.vertices),
                "polygons": len(evaluated.data.polygons),
                "dimensions": list(obj.dimensions),
            })
        elif obj.type == "LIGHT":
            light = obj.data
            record = {"name": obj.name, "type": light.type, **transform,
                      "color": list(light.color), "energy": light.energy,
                      "energy_unit": "W", "cast_shadows": light.use_shadow,
                      "direction": list(obj.rotation_euler.to_quaternion() @ Vector((0, 0, -1)))}
            if light.type == "AREA":
                record.update(shape=light.shape, size=light.size)
            else:
                record["radius"] = light.shadow_soft_size
            lights.append(record)
    # Blender AUTO sensor fit uses the width for this camera. Derive both FOVs
    # from the actual projection so the portrait frame is unambiguous.
    frame = camera.data.view_frame(scene=scene)
    fov_x = 2 * math.atan(max(abs(v.x / v.z) for v in frame))
    fov_y = 2 * math.atan(max(abs(v.y / v.z) for v in frame))
    document = {
        "schema_version": 1, "generator": "Blender " + bpy.app.version_string,
        "units": "meters", "up_axis": "+Z", "front_axis": "-Y",
        "color_space": "linear sRGB; hex colors are display sRGB",
        "object_count": len(geometry), "light_count": len(lights),
        "scene_object_count": len(scene.objects),
        "camera": {"name": camera.name, "type": "perspective", "position": list(camera.location),
                   "target": list(target), "up": [0, 0, 1], "lens_mm": camera.data.lens,
                   "fov": math.degrees(fov_y), "fov_axis": "vertical", "fov_unit": "degrees",
                   "fov_horizontal_degrees": math.degrees(fov_x),
                   "fov_vertical_degrees": math.degrees(fov_y),
                   "near": camera.data.clip_start, "far": camera.data.clip_end},
        "world": {"color": list(rgb("#f4f3f7")), "strength": RENDER["world_strength"]},
        "render": {"engine": "CYCLES", "device": RENDER["device"], "samples": scene.cycles.samples,
                   "seed": 17, "denoise": False, "adaptive_threshold": scene.cycles.adaptive_threshold,
                   "width": scene.render.resolution_x, "height": scene.render.resolution_y,
                   "view_transform": scene.view_settings.view_transform,
                   "look": scene.view_settings.look, "exposure": scene.view_settings.exposure,
                   "compositor": GLARE},
        "lights": lights, "objects": geometry,
        "fur": {"type": "hair_particles_interpolated_children", "material": "Long pile faux fur",
                "settings": FUR, "systems": FUR_SYSTEMS,
                "rendered_strands": sum(item["rendered_strands"] for item in FUR_SYSTEMS.values()),
                "excluded": ["Face", "Eyes", "Catchlights", "Smile", "Cheeks", "Orb"]},
        "exports": {"geometry": "mascot.obj", "materials": "mascot.mtl",
                    "obj_axes": {"forward": "-Y", "up": "+Z"}},
        "notes": [
            "Plush parts carry hair particle fur; the noise bump and sheen surface underneath reads as the dense undercoat.",
            "Fur is not in the OBJ. Furred objects have their primitive scale baked; see each recipe's baked_scale.",
            "OBJ/MTL retains geometry and basic colors. The blend and JSON retain procedural materials and lights.",
            "The nebula core is an emissive Principled Volume with a rim-only surface, inside light filaments, a glass shell and a faint violet halo volume.",
            "The orb point light has shadows off so the emissive core does not block its light.",
            "Primitive recipes use local coordinates; apply each object's scale, rotation, and position.",
        ],
    }
    (OUT / "scene.json").write_text(json.dumps(document, indent=2) + "\n")
    return document


def use_gpu(scene):
    # Prefer OptiX, then CUDA; fall back to the CPU if this build has neither.
    prefs = bpy.context.preferences.addons["cycles"].preferences
    for kind in ("OPTIX", "CUDA"):
        try:
            prefs.compute_device_type = kind
        except TypeError:
            continue
        prefs.get_devices()
        gpus = [device for device in prefs.devices if device.type == kind]
        if gpus:
            for device in prefs.devices:
                device.use = device.type == kind
            scene.cycles.device = "GPU"
            return kind + " " + gpus[0].name
    scene.cycles.device = "CPU"
    return "CPU"


def main():
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.ops.object.select_all(action="SELECT")
    bpy.ops.object.delete(use_global=False)
    scene = bpy.context.scene
    scene.unit_settings.system = "METRIC"
    scene.render.engine = "CYCLES"
    RENDER["device"] = use_gpu(scene)
    scene.cycles.samples = RENDER["samples"]
    scene.cycles.seed = 17
    # Debian's Blender build has no OpenImageDenoise (or OptiX) support. Keep this
    # scene portable and render enough samples without a separate denoiser.
    scene.cycles.use_denoising = False
    scene.cycles.adaptive_threshold = 0.012
    scene.cycles.sample_clamp_indirect = 3
    scene.cycles.max_bounces = 10
    scene.cycles.transmission_bounces = 6
    scene.cycles.transparent_max_bounces = 16
    scene.cycles.volume_bounces = 1
    scene.cycles.volume_step_rate = 0.5
    scene.cycles_curves.shape = "THICK"
    scene.cycles_curves.subdivisions = 2
    scene.render.threads_mode = "FIXED"
    scene.render.threads = 12
    scene.render.resolution_x = 640
    scene.render.resolution_y = 768
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = "RGB"
    scene.render.film_transparent = False
    scene.view_settings.view_transform = "AgX"
    scene.view_settings.look = "AgX - Medium High Contrast"
    scene.view_settings.exposure = RENDER["exposure"]

    plush = material("Warm beige plush", "#a38878", 0.91, texture=True, sheen=1.0)
    plush.node_tree.nodes.get("Principled BSDF").inputs["Sheen Tint"].default_value = (*rgb("#e8d2b8"), 1)
    MATERIALS[plush.name]["sheen_tint_srgb_hex"] = "#e8d2b8"
    cream = material("Pale cream face", "#f7c385", 0.84, sheen=0.15)
    eye = material("Glossy black eyes", "#000000", 0.03)
    # A low specular level keeps the dome black; the catchlight bead does the sparkle.
    eye.node_tree.nodes.get("Principled BSDF").inputs["Specular IOR Level"].default_value = 0.06
    MATERIALS[eye.name]["specular_ior_level"] = 0.06
    smile = material("Soft dark smile", "#000000", 0.4)
    smile.node_tree.nodes.get("Principled BSDF").inputs["Specular IOR Level"].default_value = 0.15
    MATERIALS[smile.name]["specular_ior_level"] = 0.15
    cheek = material("Soft rose cheeks", "#ea6f6a", 0.94)
    soften_edges(cheek)
    glint = material("Eye catchlight", "#ffffff", 0.24, emission=0.15)
    ground = material("Soft white ground", "#f6f5f8", 0.94)
    # The cyc and floor glow faintly so the studio stays high-key while the
    # figure keeps a directional key and real shade on its left side.
    ground_shader = ground.node_tree.nodes.get("Principled BSDF")
    ground_shader.inputs["Emission Strength"].default_value = RENDER["cyc_glow"]
    MATERIALS[ground.name]["emission"]["strength"] = RENDER["cyc_glow"]
    shell = material("Orb clear glass", "#f7f5ff", 0.055, transmission=1, ior=1.16)
    shell.node_tree.nodes.get("Principled BSDF").inputs["Coat Weight"].default_value = 0.08
    MATERIALS[shell.name]["coat_weight"] = 0.08
    nebula = make_nebula()
    halo = bpy.data.materials.new("Orb violet halo")
    halo.use_nodes = True
    halo_nodes = halo.node_tree.nodes
    halo_nodes.remove(halo_nodes.get("Principled BSDF"))
    halo_volume = halo_nodes.new("ShaderNodeVolumePrincipled")
    halo_volume.inputs["Density"].default_value = 0.05
    halo_volume.inputs["Color"].default_value = (0, 0, 0, 1)
    halo_volume.inputs["Emission Color"].default_value = (*rgb("#9a6cff"), 1)
    halo_volume.inputs["Emission Strength"].default_value = 0.8
    halo.node_tree.links.new(halo_volume.outputs["Volume"],
                             halo_nodes.get("Material Output").inputs["Volume"])
    MATERIALS[halo.name] = {"name": halo.name, "type": "volume_only",
                            "base_color_srgb_hex": "#9a6cff", "base_color": list(rgb("#9a6cff")),
                            "density": 0.05, "emission": {"color_srgb_hex": "#9a6cff", "strength": 0.8}}
    violet = material("Pink light filament", "#ff6fd8", 0.4, emission=6)
    blue = material("Cyan light filament", "#45e0ff", 0.4, emission=6)
    fur = fur_material()

    body = make_body(plush)
    make_face(cream)
    furred = [body]
    for sign, side in [(-1, "Left"), (1, "Right")]:
        ex, ez = sign * RIG["eye_x"], RIG["eye_z"]
        ey = face_y(ex, ez, 0.35) + 0.018
        sphere("Eye " + side, (ex, ey, ez), RIG["eye_scale"], eye)
        # The catchlight sits upper right on both eyes, as in the reference.
        gx, gz = ex + 0.026, ez + 0.027
        sphere("Catchlight " + side, (gx, ey - RIG["eye_scale"][1] * 0.74, gz),
               (0.017, 0.010, 0.018), glint, 24, 16)
        cx, cz = sign * RIG["cheek_x"], RIG["cheek_z"]
        blush = sphere("Cheek " + side, (cx, face_y(cx, cz, 0.5) + 0.02, cz), RIG["cheek_scale"], cheek)
        blush.rotation_euler = face_normal(cx, cz, 0.5).to_track_quat("-Y", "Z").to_euler()
        arm = sphere("Arm " + side, (sign * RIG["arm_x"], -0.245, 1.82), RIG["arm_scale"], plush)
        # Align the long axis from the hand to the shoulder.
        axis = Vector((-sign * 0.10, 0.39, 0.67))
        arm.rotation_euler = axis.to_track_quat("Z", "Y").to_euler()
        hand = sphere("Hand " + side, (sign * RIG["hand_x"], RIG["hand_y"], RIG["hand_z"]),
                      RIG["hand_scale"], plush)
        thumb = sphere("Thumb " + side, (sign * RIG["thumb_x"], RIG["thumb_y"], RIG["thumb_z"]),
                       RIG["thumb_scale"], plush)
        thumb.rotation_euler = (0, -sign * RIG["thumb_tilt"], 0)
        foot = make_leg("Foot " + side, (sign * RIG["leg_x"], -0.05, 0), plush,
                        RIG["leg_radius"])
        furred += [arm, hand, thumb, foot]

    smile_points = []
    half = RIG["smile_half_width"]
    for i in range(49):
        x = -half + 2 * half * i / 48
        z = RIG["smile_z"] + RIG["smile_sag"] * (x / half) ** 2
        smile_points.append((x, face_y(x, z, 0.3) - 0.004, z))
    tube("Smile", smile_points, 0.013, smile)

    orb_center = Vector(RIG["orb_center"])
    sphere("Orb nebula core", orb_center, (0.225,) * 3, nebula, 96, 64)
    sphere("Orb glass shell", orb_center, (0.255,) * 3, shell, 96, 64)
    sphere("Orb halo", orb_center, (0.30,) * 3, halo, 48, 32)
    # Thin, irregular luminous wisps just outside the core, inside the glass.
    for index in range(8):
        points = []
        phase = index * 2.399
        for step in range(121):
            t = step / 120
            latitude = 1.2 * math.sin(phase * 1.7 + t * 2.3) + 0.12 * math.sin(t * 13 + 3 * index)
            longitude = phase + t * 2.6 + 0.35 * math.sin(t * 17 + index)
            radius = 0.229 + 0.002 * math.sin(t * 31 + index)
            point = orb_center + radius * Vector((
                math.cos(latitude) * math.cos(longitude),
                math.cos(latitude) * math.sin(longitude), math.sin(latitude)))
            points.append(tuple(point))
        tube("Nebula filament %02d" % (index + 1), points, 0.0018,
             violet if index % 2 == 0 else blue, 3)

    # Fur: bake primitive scales so strands are not stretched, then share the
    # rendered strand budget across the plush parts by surface area.
    bpy.context.view_layer.update()
    for obj in furred:
        if obj.type == "MESH" and tuple(obj.scale) != (1, 1, 1):
            bake_scale(obj)
    areas = {obj.name: surface_area(obj) for obj in furred}
    total_parents = FUR["children_total"] // FUR["children_per_parent"]

    def body_weight(co):
        # No fur under the face patch (the hood fur at its rim overlaps the edge),
        # and none on the flat base where the legs join.
        if co.y < 0:
            u = abs(co.x) / FACE["half_width"]
            v = abs(co.z - FACE["z"]) / FACE["half_height"]
            d = (u ** FACE["exponent"] + v ** FACE["exponent"]) ** (1 / FACE["exponent"])
            # Roots start just under the tucked face rim, so the hood fur spills
            # over the face edge as in the reference.
            if d < 0.92:
                return 0.0
        return 1.0

    def foot_weight(co):
        return 0.0 if co.z < 0.02 else 1.0

    for obj in furred:
        parents = int(total_parents * areas[obj.name] / sum(areas.values()))
        weight = body_weight if obj is body else foot_weight if obj.name.startswith("Foot") else None
        add_fur(obj, fur, parents, weight)
        FUR_SYSTEMS[obj.name]["surface_area_m2"] = round(areas[obj.name], 4)

    bpy.ops.mesh.primitive_plane_add(size=200, location=(0, 0, -0.008))
    finish(bpy.context.object, "Ground", ground, {"primitive": "plane", "size": 200})

    # A wide curved plane gives a seamless studio background above the floor.
    profile = []
    for i in range(33):
        angle = (math.pi / 2) * i / 32
        profile.append((2.5 + 3 * math.sin(angle), -0.008 + 3 * (1 - math.cos(angle))))
    profile.append((5.5, 10))
    verts = [(x, y, z) for y, z in profile for x in (-15, 15)]
    faces = [(2*i, 2*i+1, 2*i+3, 2*i+2) for i in range(len(profile)-1)]
    mesh = bpy.data.meshes.new("Curved studio plane")
    mesh.from_pydata(verts, [], faces)
    mesh.update()
    backdrop = bpy.data.objects.new("Studio backdrop", mesh)
    bpy.context.collection.objects.link(backdrop)
    finish(backdrop, "Studio backdrop", ground, {
        "primitive": "extruded_profile", "x_min": -15, "x_max": 15,
        "yz_profile": [list(p) for p in profile],
    })

    world = bpy.data.worlds.new("Neutral light gray world")
    scene.world = world
    world.use_nodes = True
    world.node_tree.nodes["Background"].inputs["Color"].default_value = (*rgb("#f4f3f7"), 1)
    world.node_tree.nodes["Background"].inputs["Strength"].default_value = RENDER["world_strength"]
    # The reference is keyed from camera right, with the figure's left side in soft shade.
    for name, position, target, power, size, color in LIGHTS:
        area(name, position, target, power, size, color)
    light_data = bpy.data.lights.new("Orb violet spill", "POINT")
    light_data.energy = RIG["orb_spill"]
    light_data.color = rgb("#b579ff")
    light_data.shadow_soft_size = 0.14
    light_data.use_shadow = False
    light = bpy.data.objects.new("Orb violet spill", light_data)
    bpy.context.collection.objects.link(light)
    light.location = orb_center
    # Blender 4 renders point lights as visible spheres; this one should only
    # spill violet light, not show up as a bulb inside the translucent core.
    light.visible_camera = False
    light.visible_glossy = False
    light.visible_transmission = False

    camera_data = bpy.data.cameras.new("Portrait camera")
    camera = bpy.data.objects.new("Portrait camera", camera_data)
    bpy.context.collection.objects.link(camera)
    camera.location = CAMERA["position"]
    target = CAMERA["target"]
    aim(camera, target)
    camera_data.lens = CAMERA["lens"]
    camera_data.clip_start = 0.05
    camera_data.clip_end = 300
    scene.camera = camera

    bpy.context.view_layer.use_pass_object_index = True
    for obj in scene.objects:
        if obj.name.startswith(("Orb ", "Nebula filament")) and obj.type == "MESH":
            obj.pass_index = 1
    scene.use_nodes = True
    nodes = scene.node_tree.nodes
    nodes.clear()
    render = nodes.new("CompositorNodeRLayers")
    glow = nodes.new("CompositorNodeGlare")
    glow.glare_type = GLARE["type"]
    glow.quality = "HIGH"
    glow.threshold = GLARE["threshold"]
    glow.size = GLARE["size"]
    links = scene.node_tree.links
    links.new(render.outputs["Image"], glow.inputs["Image"])
    # The orb gets its own low-threshold bloom, masked by object index, and
    # only the added glow (bloom minus the masked orb) goes over the frame.
    orb_mask = nodes.new("CompositorNodeIDMask")
    orb_mask.index = 1
    orb_mask.use_antialiasing = True
    links.new(render.outputs["IndexOB"], orb_mask.inputs["ID value"])
    orb_only = nodes.new("CompositorNodeMixRGB")
    orb_only.blend_type = "MULTIPLY"
    links.new(render.outputs["Image"], orb_only.inputs[1])
    links.new(orb_mask.outputs["Alpha"], orb_only.inputs[2])
    bloom = nodes.new("CompositorNodeGlare")
    bloom.glare_type = GLARE["orb_bloom"]["type"]
    bloom.quality = "HIGH"
    bloom.threshold = GLARE["orb_bloom"]["threshold"]
    bloom.size = GLARE["orb_bloom"]["size"]
    links.new(orb_only.outputs["Image"], bloom.inputs["Image"])
    glow_only = nodes.new("CompositorNodeMixRGB")
    glow_only.blend_type = "SUBTRACT"
    links.new(bloom.outputs["Image"], glow_only.inputs[1])
    links.new(orb_only.outputs["Image"], glow_only.inputs[2])
    # Keep the bloom outside the orb so it does not wash the nebula to lavender.
    outside = nodes.new("CompositorNodeInvert")
    links.new(orb_mask.outputs["Alpha"], outside.inputs["Color"])
    halo_only = nodes.new("CompositorNodeMixRGB")
    halo_only.blend_type = "MULTIPLY"
    links.new(glow_only.outputs["Image"], halo_only.inputs[1])
    links.new(outside.outputs["Color"], halo_only.inputs[2])
    combine = nodes.new("CompositorNodeMixRGB")
    combine.blend_type = "ADD"
    links.new(glow.outputs["Image"], combine.inputs[1])
    links.new(halo_only.outputs["Image"], combine.inputs[2])
    output = nodes.new("CompositorNodeComposite")
    links.new(combine.outputs["Image"], output.inputs["Image"])

    bpy.context.view_layer.update()
    bpy.ops.object.select_all(action="DESELECT")
    for obj in scene.objects:
        if obj.type == "MESH":
            obj.select_set(True)
    bpy.ops.wm.obj_export(filepath=str(OUT / "mascot.obj"),
                          export_selected_objects=True, export_materials=True,
                          forward_axis="NEGATIVE_Y", up_axis="Z",
                          apply_modifiers=True, export_normals=True)
    document = write_scene(scene, camera, target)
    scene.render.filepath = str(OUT / "preview.png")
    # No automatic backup file is needed when the script is rerun.
    bpy.context.preferences.filepaths.save_version = 0
    bpy.ops.wm.save_as_mainfile(filepath=str(OUT / "mascot.blend"))
    started = time.time()
    bpy.ops.render.render(write_still=True)
    print("MUSE_BUILD_COMPLETE " + json.dumps({
        "mesh_objects": document["object_count"], "lights": document["light_count"],
        "scene_objects": document["scene_object_count"],
        "vertices": sum(obj["vertices"] for obj in document["objects"]),
        "fur_strands": sum(system["rendered_strands"] for system in FUR_SYSTEMS.values()),
        "device": RENDER["device"], "samples": RENDER["samples"],
        "render_seconds": round(time.time() - started, 1),
        "output_directory": str(OUT),
    }))


if __name__ == "__main__":
    main()
