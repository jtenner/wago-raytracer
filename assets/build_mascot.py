"""Build Muse with Blender 4.3 or newer, without external assets.

Run from any directory:
    /usr/bin/blender --background --python assets/build_mascot.py

Coordinates are right-handed, Z up; the character faces -Y. Lengths are meters.
The OBJ contains all scene geometry, including the ground, in these coordinates.
The JSON records the primitive recipes and material settings for other renderers.
"""

import json
import math
from pathlib import Path

import bpy
from mathutils import Vector


OUT = Path(__file__).resolve().parent
RECIPES = {}
MATERIALS = {}


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
        bump.inputs["Strength"].default_value = 0.4
        bump.inputs["Distance"].default_value = 0.018
        links.new(noise.outputs["Fac"], bump.inputs["Height"])
        links.new(bump.outputs["Normal"], shader.inputs["Normal"])
        spec["surface_texture"] = {
            "type": "noise_bump", "coordinates": "object", "scale": 125,
            "detail": 2, "roughness": 0.7, "strength": 0.4, "distance": 0.018,
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


def make_body(mat):
    # A sphere lathe with round shoulders and a broad, softly rounded base.
    segments, rings = 96, 64
    power = 0.73
    verts = [(0, 0, -1)]
    for ring in range(1, rings):
        latitude = -math.pi / 2 + math.pi * ring / rings
        radius = math.cos(latitude) ** power
        z = math.copysign(abs(math.sin(latitude)) ** power, latitude)
        for segment in range(segments):
            angle = 2 * math.pi * segment / segments
            verts.append((radius * math.cos(angle), radius * math.sin(angle), z))
    top = len(verts)
    verts.append((0, 0, 1))
    faces = []
    for segment in range(segments):
        nxt = (segment + 1) % segments
        faces.append((0, 1 + nxt, 1 + segment))
    for ring in range(rings - 2):
        start = 1 + ring * segments
        for segment in range(segments):
            nxt = (segment + 1) % segments
            faces.append((start + segment, start + nxt,
                          start + segments + nxt, start + segments + segment))
    start = 1 + (rings - 2) * segments
    for segment in range(segments):
        faces.append((start + segment, start + (segment + 1) % segments, top))
    mesh = bpy.data.meshes.new("Body rounded lathe")
    mesh.from_pydata(verts, [], faces)
    mesh.update()
    obj = bpy.data.objects.new("Body", mesh)
    bpy.context.collection.objects.link(obj)
    obj.location = (0, 0, 1.91)
    obj.scale = (0.94, 0.59, 1.62)
    return finish(obj, "Body", mat, {
        "primitive": "superellipsoid_lathe", "segments": segments, "rings": rings,
        "latitude_power": power,
        "formula": "r=cos(latitude)^power; z=sign(sin(latitude))*abs(sin(latitude))^power; x=r*cos(longitude); y=r*sin(longitude)",
    })


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
    ramp.inputs["From Min"].default_value = 0.25
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
        "opaque_within": 0.25, "clear_beyond": 0.95,
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
    links.new(coord.outputs["Generated"], warp.inputs["Vector"])
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
    links.new(cloud.outputs["Fac"], ramp.inputs["Fac"])
    # Keep the surface itself dark so studio light cannot wash the nebula to pastel.
    shader.inputs["Base Color"].default_value = (*rgb("#0d0a1c"), 1)
    # A white-hot rim, as in the reference, from a facing falloff added to the emission.
    facing = nodes.new("ShaderNodeLayerWeight")
    facing.inputs["Blend"].default_value = 0.5
    rim = nodes.new("ShaderNodeValToRGB")
    rim.color_ramp.elements[0].position = 0.5
    rim.color_ramp.elements[1].position = 0.95
    rim.color_ramp.elements[1].color = (*rgb("#f2e6ff"), 1)
    links.new(facing.outputs["Facing"], rim.inputs["Fac"])
    add = nodes.new("ShaderNodeMix")
    add.data_type = "RGBA"
    add.blend_type = "ADD"
    add.inputs["Factor"].default_value = 1
    links.new(ramp.outputs["Color"], add.inputs["A"])
    links.new(rim.outputs["Color"], add.inputs["B"])
    links.new(add.outputs["Result"], shader.inputs["Emission Color"])
    MATERIALS[mat.name]["procedural_color"] = {
        "type": "warped_noise_color_ramp", "coordinates": "generated",
        "drives": "emission_color", "base_color_srgb_hex": "#0d0a1c",
        "warp": {"scale": 1.7, "detail": 4, "roughness": 0.7},
        "noise": {"scale": 2.3, "detail": 3, "roughness": 0.65},
        "stops": [{"position": p, "color": list(rgb(c)), "srgb_hex": c} for p, c in stops],
        "rim": {"type": "layer_weight_facing_add", "blend": 0.5, "from": 0.5, "to": 0.95,
                "color_srgb_hex": "#f2e6ff"},
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
        "world": {"color": list(rgb("#f4f3f7")), "strength": 1.2},
        "render": {"engine": "CYCLES", "device": "CPU", "samples": scene.cycles.samples,
                   "seed": 17, "denoise": False, "adaptive_threshold": scene.cycles.adaptive_threshold,
                   "width": scene.render.resolution_x, "height": scene.render.resolution_y,
                   "view_transform": scene.view_settings.view_transform,
                   "look": scene.view_settings.look, "exposure": scene.view_settings.exposure,
                   "compositor": {"type": "FOG_GLOW", "threshold": 1.2, "size": 7}},
        "lights": lights, "objects": geometry,
        "exports": {"geometry": "mascot.obj", "materials": "mascot.mtl",
                    "obj_axes": {"forward": "-Y", "up": "+Z"}},
        "notes": [
            "The body uses fine noise bump and sheen in place of individual fur strands.",
            "OBJ/MTL retains geometry and basic colors. The blend and JSON retain procedural materials and lights.",
            "The nebula uses an opaque emissive core, surface light filaments, and a separate glass shell.",
            "The orb point light has shadows off so the emissive core does not block its light.",
            "Primitive recipes use local coordinates; apply each object's scale, rotation, and position.",
        ],
    }
    (OUT / "scene.json").write_text(json.dumps(document, indent=2) + "\n")
    return document


def main():
    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.ops.object.select_all(action="SELECT")
    bpy.ops.object.delete(use_global=False)
    scene = bpy.context.scene
    scene.unit_settings.system = "METRIC"
    scene.render.engine = "CYCLES"
    scene.cycles.device = "CPU"
    scene.cycles.samples = 512
    scene.cycles.seed = 17
    # Debian's Blender build has no OpenImageDenoise support. Keep this scene
    # portable and render enough samples without a separate denoiser.
    scene.cycles.use_denoising = False
    scene.cycles.adaptive_threshold = 0.012
    scene.cycles.sample_clamp_indirect = 3
    scene.cycles.max_bounces = 10
    scene.cycles.transmission_bounces = 6
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
    scene.view_settings.exposure = 0

    plush = material("Warm beige plush", "#c9a07e", 0.91, texture=True, sheen=1.0)
    cream = material("Pale cream face", "#f6c9a6", 0.84, sheen=0.15)
    eye = material("Glossy black eyes", "#101015", 0.13)
    smile = material("Soft dark smile", "#2c2025", 0.6)
    cheek = material("Soft rose cheeks", "#f0a59a", 0.94)
    soften_edges(cheek)
    glint = material("Eye catchlight", "#ffffff", 0.24, emission=0.15)
    ground = material("Soft white ground", "#f6f5f8", 0.94)
    shell = material("Orb clear glass", "#f7f5ff", 0.055, transmission=1, ior=1.16)
    shell.node_tree.nodes.get("Principled BSDF").inputs["Coat Weight"].default_value = 0.08
    MATERIALS[shell.name]["coat_weight"] = 0.08
    nebula = make_nebula()
    violet = material("Pink light filament", "#ff6fd8", 0.4, emission=5)
    blue = material("Cyan light filament", "#45e0ff", 0.4, emission=5)

    make_body(plush)
    sphere("Face", (0, -0.545, 2.69), (0.62, 0.17, 0.49), cream)
    for sign, side in [(-1, "Left"), (1, "Right")]:
        sphere("Eye " + side, (sign * 0.285, -0.714, 2.745), (0.071, 0.046, 0.082), eye)
        sphere("Catchlight " + side, (sign * 0.285 - 0.018, -0.756, 2.774),
               (0.018, 0.010, 0.019), glint, 24, 16)
        sphere("Cheek " + side, (sign * 0.37, -0.686, 2.58), (0.12, 0.02, 0.08), cheek)
        arm = sphere("Arm " + side, (sign * 0.855, -0.245, 1.89),
                     (0.265, 0.29, 0.53), plush)
        # Align the long axis from the hand to the shoulder.
        axis = Vector((sign * 0.03, 0.39, 0.67))
        arm.rotation_euler = axis.to_track_quat("Z", "Y").to_euler()
        sphere("Hand " + side, (sign * 0.575, -0.745, 1.64), (0.335, 0.30, 0.285), plush)
        sphere("Thumb " + side, (sign * 0.42, -0.76, 1.83), (0.125, 0.17, 0.17), plush)
        sphere("Foot " + side, (sign * 0.40, -0.06, 0.30), (0.29, 0.32, 0.30), plush)

    smile_points = []
    for i in range(49):
        x = -0.105 + 0.21 * i / 48
        smile_points.append((x, -0.716, 2.427 + 0.041 * (x / 0.105) ** 2))
    tube("Smile", smile_points, 0.0115, smile)

    orb_center = Vector((0, -0.93, 1.88))
    sphere("Orb nebula core", orb_center, (0.225,) * 3, nebula, 96, 64)
    sphere("Orb glass shell", orb_center, (0.255,) * 3, shell, 96, 64)
    # Thin, irregular luminous wisps just outside the core, inside the glass.
    for index in range(14):
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
        tube("Nebula filament %02d" % (index + 1), points, 0.0026,
             violet if index % 2 == 0 else blue, 3)

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
    world.node_tree.nodes["Background"].inputs["Strength"].default_value = 1.2
    area("Key softbox", (-3.3, -4.2, 6.0), (0, 0, 1.9), 440, 4.0, "#fff4e9")
    area("Fill softbox", (3.2, -2.2, 3.6), (0, 0, 1.8), 180, 3.4, "#e9f1ff")
    area("Rim softbox", (0.7, 2.6, 5.1), (0, 0, 2.1), 340, 3.0, "#fff8f0")
    light_data = bpy.data.lights.new("Orb violet spill", "POINT")
    light_data.energy = 16
    light_data.color = rgb("#b579ff")
    light_data.shadow_soft_size = 0.14
    light_data.use_shadow = False
    light = bpy.data.objects.new("Orb violet spill", light_data)
    bpy.context.collection.objects.link(light)
    light.location = orb_center

    camera_data = bpy.data.cameras.new("Portrait camera")
    camera = bpy.data.objects.new("Portrait camera", camera_data)
    bpy.context.collection.objects.link(camera)
    camera.location = (0.35, -7.8, 3.13)
    target = (0, -0.03, 1.79)
    aim(camera, target)
    camera_data.lens = 58
    camera_data.clip_start = 0.05
    camera_data.clip_end = 300
    scene.camera = camera

    scene.use_nodes = True
    nodes = scene.node_tree.nodes
    nodes.clear()
    render = nodes.new("CompositorNodeRLayers")
    glow = nodes.new("CompositorNodeGlare")
    glow.glare_type = "FOG_GLOW"
    glow.quality = "HIGH"
    glow.threshold = 1.2
    glow.size = 7
    output = nodes.new("CompositorNodeComposite")
    scene.node_tree.links.new(render.outputs["Image"], glow.inputs["Image"])
    scene.node_tree.links.new(glow.outputs["Image"], output.inputs["Image"])

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
    bpy.ops.render.render(write_still=True)
    print("MUSE_BUILD_COMPLETE " + json.dumps({
        "mesh_objects": document["object_count"], "lights": document["light_count"],
        "scene_objects": document["scene_object_count"],
        "vertices": sum(obj["vertices"] for obj in document["objects"]),
        "output_directory": str(OUT),
    }))


if __name__ == "__main__":
    main()
