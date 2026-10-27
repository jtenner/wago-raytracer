# Muse mascot: fur pass

This pass works through the ranked "still off" list in `taste-review.md`, fur first.
Under the owner's accuracy directive, `reference/muse-mascot.jpg` was treated as
ground truth. Where a measured reference value disagreed with a number in the brief,
I used the reference value; each such case is listed below. Everything lives in
`assets/build_mascot.py`. Running

    /usr/bin/blender --background --python assets/build_mascot.py

regenerates `mascot.blend`, `mascot.obj/.mtl`, `scene.json` and `preview.png`.
`side-by-side.png` puts the reference on the left and the final render on the right.

## Render

| | |
|---|---|
| Device | Cycles **CUDA**, NVIDIA GeForce RTX 4060 Laptop GPU. OptiX is not compiled into Debian's Blender 4.3.2, so the script tries OptiX, then CUDA, then falls back to CPU |
| Samples | 512, adaptive threshold 0.012, no denoiser (this build has no OIDN or OptiX denoiser) |
| Render time | **18.9 s** for the render; 20.6 s wall clock for the whole script |
| One-time cost | The first CUDA render on this machine compiled the sm_89 kernel with nvcc (about 4 minutes). It is cached in `~/.cache/cycles/kernels/` |
| Output | `preview.png`, 640×768, unchanged so it stays comparable with earlier versions |

GPU reruns are not bit-identical; two clean runs differ by RMSE 0.009, which is
sampling noise.

## Fur, as used

The fur is a hair particle system with interpolated children on Body, Arm L/R,
Hand L/R, Thumb L/R and Foot L/R. There is no fur on the face, eyes, catchlights,
smile, cheeks or orb.

| Parameter | Brief | Used | Why it differs |
|---|---|---|---|
| Rendered strands | 60k–120k | **119,700**: 1,995 parents × 60 children, split by surface area (Body 73,980) | Renders in under 20 s on the GPU, so I used the top of the range |
| Strand length | ~0.035 m | **0.05 m** | The reference's fuzzy silhouette band is about 0.05 m deep at this scale. At 0.035 m the fur read as flocking |
| Root radius | ~0.003 m | **0.0035 m**, tip ×0.12 | Slightly thicker strands give more visible strand texture at 640 px |
| Direction | falls downward | normal + (0, 0, −0.72) emission. A "Fur length" vertex group evens strand length between up- and down-facing surfaces | With advanced hair, Blender's strand length is 4 × \|emission velocity\|, so the downward bias alone made crown strands short and underside strands long |
| Clump / kink | light | clump 0.5, shape 0.3; roughness 0.0035 / end 0.010 / random 0.004; **no kink** | Curl made the fur look frizzy. Clumping gives tufts without looking wet |
| Shader | Principled Hair, melanin ~0.25, tint `#c9a07e`, random roughness 0.3 | Principled Hair (Chiang), **melanin 0.08**, redness 0.3, **tint `#e2b39c`**, roughness 0.32, radial 0.4, random roughness 0.3 | At melanin 0.15–0.25 individual strands rendered orange against the pink beige. The reference body is `#d0b3a5`; I measured `#ceb3a1` |
| Undercoat | – | plush surface `#a38878`, sheen 1.0, **Sheen Tint `#e8d2b8`** (item 7), bump 0.2 | A darker, neutral undercoat gives depth between strands. The old orange-beige undercoat showed through as flecks |
| Face exclusion | – | "Fur density" vertex group: no roots inside the face squircle (d < 0.92); no roots on the soles | Roots start just under the tucked face rim, so hood fur spills over the face edge |

Furred objects have their primitive scale baked into the mesh so the strands are not
stretched. Each recipe in `scene.json` records `baked_scale`. As agreed, the OBJ does
not carry the hair.

## Other changes (taste-review items 2–7, plus what the comparison found)

- **Body (items 1 and 6).** The pill lathe (power 0.73) is replaced by a body profile
  traced from the reference silhouette (`BODY_PROFILE`, Catmull-Rom smoothed). It is
  pear-shaped, with a broad flat crown, near-straight sides widening to the belly, and
  a smooth taper into the legs. This also covers item 6, the crown flattening.
- **Arms (item 2).** Scale `(0.22, 0.26, 0.50)` as in the brief. The center is at
  **x ±0.86**, not ±0.80. At ±0.80 the arm and hand silhouette came out 0.027H inside
  the reference's outline. The arms also tilt inward at the shoulder, so they hug the
  body.
- **Feet (item 3).** The ellipsoids are replaced by **flat-soled capsules**: a cylinder
  with a 0.11 m sole fillet and a rounded top hidden inside the body, base at z 0, and
  about 0.42 m visible below the body.
  - I used **r 0.315 at x ±0.455** instead of r 0.27 at ±0.38. The reference legs
    measure 0.65 m wide including fur, with a 0.13 m gap.
  - A true hemispherical capsule cannot have r 0.27 and be only 0.45 m tall, and its
    domed sole did not match the reference's flat soles.
- **Orb (items 4–5).**
  - **Core.** The core is now a Principled Volume driven by the same warped-noise ramp,
    with a transparent surface that adds only the white-hot rim. Two changes were
    needed to keep it from rendering as pastel lavender:
    - **Density 8, not 2, with a black scatter color.** At density 2 the volume
      scattered the studio light and averaged the colors along each ray.
    - **Noise stretched along the view axis, plus a diagonal ramp bias.** This gives
      the reference's layout: magenta upper left, violet-blue center, cyan lower
      right.
    - Emission strength is 11.
  - **Wisps:** 8 at r 0.0018.
  - **Halo:** I picked the **halo sphere** (r 0.30, emission `#9a6cff`, density 0.05)
    at emission **0.8**, not 0.4. I also added an orb-only glare (size 8, threshold
    0.9), masked by object index and added outside the orb only, so the orb blooms
    without the rest of the frame blooming.
  - **Global glare:** raised from threshold 1.2 to **3.0**. At 1.2 the key-lit skin
    bloomed across the black eyes, turning the right eye brown, and lightly hazed every
    bright edge.
  - **Spill light:** the violet point light is now hidden from camera, glossy and
    transmission rays. Blender 4 draws point lights as visible spheres, and it showed
    as a white bulb through the translucent core.
- **Face.**
  - The face is now a squircle patch (exponent 2.6, 1.29 × 1.01 m) that follows the
    head surface, sits recessed under the fur level, and tucks under at its rim.
  - Color `#f7c385` renders as `#eed8c5` (reference `#f3dac3`).
- **Eyes and smile.**
  - Eyes are larger and placed by measurement, with the catchlight upper right on both
    eyes as in the reference.
  - Eye and smile base colors are pure black with a low specular level. A near-black
    base still picked up a brown diffuse tint under the strong key.
  - The smile is thicker (r 0.013) and deeper.
- **Cheeks.** Oriented to the face normal so the blush lies on the curved face; larger,
  more saturated (`#ea6f6a`), with a fully radial fade. Measured `#e49892` / `#f5b9b2`
  against the reference's `#e09d95` / `#f4bbaf`.
- **Light and camera.**
  - The reference is keyed from camera right, so the key softbox moved there and the
    fill dropped to 10 W.
  - World strength is 0.25, the cyc and floor glow at 0.92, and exposure is +0.3.
    Background measures `#f3f1f1` against the reference's `#f3f1f4`.
  - The camera is now frontal (x 0, z 2.6) at 62.3 mm. The figure fills 0.799 of the
    frame height (reference 0.797).

## Compare-and-fix log

There were seven measured cycles after the first fur render, each using a scripted
comparison of figure-normalized silhouette rows, feature bounding boxes and color
samples, plus zoomed crops.

1. **Fur 4 m long.** Fixed the hair-length scaling.
2. **Framing, colors, face and eyes.** Fixed framing and fur color, lengthened the fur
   to 0.05 m, made the face more peach, oriented the cheeks, darkened the smile, made
   the background high-key, and lowered the orb emission.
3. **Silhouette.** Widened the lower body and legs, moved the eyes down and in, and
   made the hands and thumbs bigger and rounder.
4. **Orb, fur texture, face depth and brightness.** Found the pastel orb's cause
   (volume scatter and averaging along the ray), made the fur tufted instead of
   fleecy, recessed the face, and brightened the lit side.
5. **Fur flecks.** Lowered the melanin, and moved the face, cheeks and smile.
6. **Framing direction, hood overlap, eyes.** Fixed the framing direction, added hood
   fur over the face rim, and isolated and fixed the brown eye (glare bloom).
7. **Lower body and orb bloom.** Re-tapered the lower body, which had a shelf over the
   legs, and added the orb-masked bloom.

Final numbers, normalized to figure height H (reference → render):

| Measure | Reference | Render |
|---|---|---|
| Figure height / frame | 0.797 | 0.799 |
| Half-width at v 0.05 | 0.168 | 0.160 |
| Half-width at v 0.20 | 0.248 | 0.240 |
| Half-width at v 0.30 | 0.256 | 0.253 |
| Half-width at v 0.50 (arms/hands) | 0.317 | 0.314 |
| Half-width at v 0.65 | 0.281 | 0.283 |
| Half-width at v 0.70 | 0.284 | 0.283 |
| Half-width at v 0.80 | 0.265 | 0.248 |
| Half-width at v 0.90 | 0.224 | 0.211 |
| Eye centers (u, v) | ±0.097, 0.223 | ±0.097, 0.229 |
| Orb center v / diameter | 0.477 / 0.125 | 0.481 / 0.123 |
| Belly color (center) | `#d0b3a5` | `#ceb3a1` |
| Lit-side belly | `#dcc7b6` | `#dcc7ba` |
| Face color | `#f3dac3` | `#eed8c5` |
| Background | `#f2f0f3` | `#f3f1f1` |

## Verdict

It now reads as a plush toy in fur, not velvet or felt. The silhouette is fuzzy all
the way round, strands fall downward, the hood fur spills over the face rim, the feet
have a fringe, and the left side falls into soft fur shade. Next to the reference,
though, it is a finer, softer, shorter-looking pile, closer to fleece or mink than the
reference's long-pile faux fur with distinct combed tufts and dark gaps between them.
Proportions, face, eyes, cheeks, colors and the studio key now match closely and
measurably. The two things a viewer will notice first are the fur texture and the
orb, which is saturated and correctly arranged but noticeably less luminous than the
reference's glowing nebula.

## Residual differences (not closed)

1. **Fur strand definition.** The reference shows longer, visibly combed strands in
   tufts with darker gaps. Mine reads as a softer, finer pile.
   - At 119,700 strands and 640 px, each strand is under 1 px wide.
   - Clumping beyond 0.5 started to look spiky, and thicker strands looked coarse.
   - I did not go past the brief's 120k cap. The likely fix is 250k–400k children or
     a Geometry Nodes curves groom with combed guides.
2. **Orb luminosity and filaments.** The reference orb is brighter, with white-hot
   streaks and a wide bloom. Mine is saturated but darker, and the 8 tube wisps read
   as thin wires rather than plasma.
   - Under AgX, raising emission trades saturation for brightness: at emission 18 the
     orb turned pastel again.
   - The reference looks composited. Closing this likely needs a hand-tuned emission
     curve or a separate orb render layer.
3. **Orb chest spill.** The reference has a cyan-blue pool of light on the belly under
   the orb. Mine is a lavender-pink haze from a single violet point light.
4. **Leg junction.** At v 0.8–0.9 my body is 0.013–0.017H (about 5–6 cm) narrower. The
   reference legs flare up into the body; mine meet it with a slight step.
5. **Crown.** Slightly narrower at the very top (0.160 vs 0.168 at v 0.05), with a
   squarer shoulder to the dome. Crown brightness is 194 vs 203.
6. **Hood depth.** The reference has a thick, raised fur rim around the face that
   casts a soft shadow onto it. Mine overlaps the edge only thinly, so the face reads
   a little more like an inset panel. A taller hood would need a separate length
   group or a rim ridge in the body mesh. I did not attempt either.
7. **Feature heights.** Eyes sit about 0.006H lower and the smile about 0.009H lower
   than in the reference, roughly 4–5 px at 640×768.
8. **Shade side.** The reference's left belly is darker than mine (152 vs about 176 at
   the same normalized spot). Cutting the fill further darkened the face more than the
   reference shows.
9. **Floor.** Slightly greyer than the reference's (239 vs 245), with a harder contact
   shadow.

## Hands and poof pass

This pass covers only the owner's two notes on the fur render:

- **Hands:** they read as smooth mitts, and the orb looked pasted on.
- **Fur:** it needed to be "poopier", meaning longer, fuller and shaggier, with a broken silhouette edge.

Everything is in `assets/build_mascot.py` (`PAW`, `make_paw`, `FUR`, `add_fur`). The render settings are unchanged: Cycles CUDA on the RTX 4060, 512 samples, the same camera, 640×768. Run the build with

    MUSE_SKIP_OBJ=1 /usr/bin/blender --background --python assets/build_mascot.py

`MUSE_SKIP_OBJ=1` is new. It leaves `mascot.obj/.mtl` untouched, as agreed. Those files still hold the fur-pass hands and thumbs. `scene.json`, `mascot.blend` and `preview.png` are regenerated.

### Hands

The old hand was an ellipsoid with a separate ellipsoid thumb, and its inner edge stopped about 0.045H short of the orb. Each side is now one **metaball paw** converted to a mesh (`Hand Left/Right`). The `Thumb` objects are gone, so the scene has 28 mesh objects instead of 30.

- **Palm.** An ellipsoid with radii (0.27, 0.25, 0.24) at (±0.60, −0.76, 1.70). It is rolled 20° so its inner edge rises toward the orb, as if carrying its weight.
- **Fingers.** Three stubby capsule fingers, radius 0.07 and length 0.13.
  - They are fanned around the orb's lower side at +6°, −24° and −54°.
  - Their tips sit 0.315 m from an aim point 0.05 m behind the orb center, so the orb overlaps the fingertips slightly.
  - Metaball stiffness is 5, not Blender's default 2, so neighboring fingers blend into creases instead of merging into a slab. That slab was what happened in the first, fur-less test.
- **Thumb.** A capsule with radius 0.08, from (±0.70, −0.76, 1.86) to (±0.60, −0.84, 1.98). It leans about 40° in toward the orb, instead of standing up like the old thumbs-up ears.
- **Contact squash.** After meshing, every paw vertex closer than 0.262 m to the orb center goes through a smooth max (blend 0.025) against that radius. The glass shell is r 0.255. This flattens the finger pads where they press on the orb and pushes a small bulge out around each pad.
- **Paw fur.**
  - The fur is shorter here (0.035 m), so the fingers survive the pile.
  - There are no roots within 0.302 m of the orb center, so the pads stay bare and no strands poke into the glass.
  - In the creases between fingers, root density drops to 5%. A crease is any point within 0.03 m of two finger capsules at nearly equal distance.
- **Arms.** Unchanged. The new paws sit where the old hands met the arms.

### Fur numbers landed on

| | Fur pass | This pass |
|---|---|---|
| Rendered strands | 119,700 (1,995 parents × 60) | **299,500** (2,995 parents × 100): Body 191,100, each Arm 15,400, each Paw 12,500, each Leg 26,300 |
| Length | 0.05 m everywhere | **body 0.085 m**, blending (smoothstep, z 2.0 → 2.3) to **head 0.042 m**; arms 0.05; paws 0.035; legs 0.07 |
| Length variation | parent random 0.12 | parent random **0.3**; **50% of children at 0.6× length** (`child_length` 0.6, threshold 0.5) |
| Root radius / tip | 0.0035 / ×0.12 | **0.0032 / ×0.1** |
| Clump | 0.5, shape 0.3 | **0.75, shape 0.0, clump noise on (size 0.4)** |
| Frizz (roughness) | uniform 0.0035, end 0.010, random 0.004 | **uniform 0.005, end 0.020, random 0.012** |
| Direction | normal + down 0.72 | normal + down 0.72 + **comb back 0.3**. Without the comb, the clumped crown strands stood straight up like a crew cut |
| Hair shader | melanin 0.08, redness 0.3, tint `#e2b39c` | **melanin 0.05, redness 0.25, tint `#e2c4bc`**. With 2.5× the strands, less undercoat shows and the pile went orange; this brings it back |
| Render time | 18.9 s | **27.2 s** |
| Peak GPU memory | – | **about 1.5 GB of 8 GB** (sampled with `nvidia-smi` during a 299,700-strand render) |

I didn't need to back off at any point. 300k fits easily in memory and only adds about 8 s. I stopped in the middle of the owner's range rather than at 350k because the edge was already shaggier than the reference (see below). There is room to go higher if wanted.

The face, eyes, smile, cheeks and orb have no fur, and the face stays clear. The head pile is shorter than in the fur pass (0.042 vs 0.05 m) because longer body fur is now combed back.

### Compare-and-fix log

I ran five full 512-sample cycles, plus two fur-less 64-sample geometry checks of the paws. Each cycle was measured with figure-normalized silhouette rows, region color means, a gridded paw crop against the reference, and zoomed edge and crown crops.

1. **300k strands, first paws.**
   - The paws read as cartoon gloves with long, thin fingers.
   - The pile went orange: lower belly `#c19c88` vs `#c5ab9e`.
   - Half-width at v 0.6 grew 0.006H, and the edge was a soft blur.
   - Fix: fatter, shorter fingers on a bigger palm; clump 0.75 with clump noise; arm and leg fur shortened.
2. **Stubby paws.**
   - The fingers read on the shade-side paw only.
   - Strict height share went to 0.803 from stray crown tips.
   - The belly was still too saturated.
   - Fix: head fur 0.042, arm fur 0.05, a paler and pinker tint, wider finger spacing, sparse crease fur.
3. **Crown.** The crown was spiky and stood up; up-facing strands get no downward lean.
   - The right paw's fingers were washed flat by the key light.
   - Fix: comb back 0.3, finger radius 0.07 at +6/−24/−54°.
4. **Combed.**
   - The crown lies down, and strict height is back to 0.797.
   - Fingers read on both paws.
   - Belly color is within a few levels of the reference.
   - The legs were still narrow.
5. **Leg fur 0.07 m.** Leg width barely moved (+0.001H), because the leg fur hangs down rather than out. I kept it because the legs now match the body's shag.

### Measured, before → after

Values are normalized to figure height H. "Before" is the committed fur-pass `preview.png`, re-measured with this pass's scripts. Some sample spots differ from the fur-pass table above, so compare within this table.

| Measure | Reference | Before | After |
|---|---|---|---|
| Figure height / frame, strict (any figure pixel) | 0.797 | 0.801 | **0.798** |
| Figure height / frame, robust (rows ≥ 20 px wide) | 0.797 | 0.797 | **0.796** |
| Background | `#f1eff2` | `#f0efef` | `#f0efef` |
| Lower belly mean (u ±0.18, v 0.64–0.78) | `#c5ab9e` | `#c4a79b` | `#c2a698` |
| Belly point (0, 0.66) | `#c2aaab` | `#c5a9a4` | `#bfa09c` |
| Lit belly (+0.12, 0.66) | `#b79889` | `#c0a395` | `#bea08e` |
| Shade belly (−0.15, 0.66) | `#ae927d` | `#bda096` | `#ba998a` |
| Half-width L/R, v 0.05 | 0.170 / 0.156 | 0.158 / 0.154 | 0.169 / 0.149 |
| v 0.10 | 0.212 / 0.211 | 0.203 / 0.198 | 0.205 / 0.195 |
| v 0.20 | 0.247 / 0.237 | 0.242 / 0.236 | 0.241 / 0.237 |
| v 0.30 | 0.256 / 0.256 | 0.250 / 0.250 | 0.252 / 0.252 |
| v 0.40 | 0.277 / 0.275 | 0.275 / 0.278 | 0.286 / 0.280 |
| v 0.50 (arms, paws) | 0.314 / 0.317 | 0.309 / 0.314 | 0.314 / 0.312 |
| v 0.60 | 0.289 / 0.295 | 0.298 / 0.306 | 0.306 / 0.303 |
| v 0.70 | 0.281 / 0.286 | 0.281 / 0.283 | 0.285 / 0.281 |
| v 0.80 | 0.263 / 0.267 | 0.242 / 0.246 | 0.247 / 0.241 |
| v 0.90 | 0.222 / 0.226 | 0.210 / 0.211 | 0.213 / 0.210 |
| Eye centers (v) | 0.223 | 0.229 | 0.227 |
| Orb | – | unchanged geometry | unchanged geometry |
| Thumb tops (u, v) | (−0.17, 0.43) / (+0.19, 0.44) | (−0.18, 0.40) / (+0.17, 0.41) | (−0.18, 0.43) / (+0.18, 0.43) |
| Paw inner edge \|u\| | 0.075–0.09, touching the orb | 0.11–0.12, 0.045H gap to the orb | 0.07, touching the orb and overlapped by it |
| Paw bottom (v) | 0.595 | 0.59–0.60 | 0.60–0.605 |
| Paw size (w × h) | 0.23 × 0.17 | 0.19 × 0.19 | 0.23 × 0.175 |
| Fringe depth (80% → 15% contrast), L / R | 0.019 / 0.072 | 0.022 / 0.033 | **0.051 / 0.057** |

Paw positions are read from a 0.05H grid and are good to about ±0.01H. The reference's right-side fringe value is inflated: its lit fur is close to the background's brightness.

**Silhouette edge, in plain words.** The reference edge is a dense, long pile combed downward. It has a fine fuzz of tips and small separate tufts, but the outline stays fairly crisp. Before this pass, the render's edge was a uniform soft fleece blur, about half as deep. Now it is a deep, irregular fringe of clumped tufts with stray strands, about twice as deep as before (1.7–2.3×). It is broken along the sides of the body, arms and legs, and visibly shaggier than the reference. The crown lies down and its outline is soft and broken, close to the reference.

### What still does not match the reference

1. **The fur is now shaggier than the reference, not just fuller.** The reference pile is long but dense and neatly combed, with crisp tufts and darker gaps. Mine reads more like a shaggy muppet, with wispy strays at the shoulders and elbows; v 0.40 is 0.009H wide on the left. I went with the owner's note. If it is too much, the first knobs are clump 0.75 → 0.6 and roughness end 0.02 → 0.012.
2. **The paws are more explicit than the reference's.** In the reference, the fingers are only suggested under the fur. Here three fingers and the creases read clearly, as the owner asked. As a result, the paws look slightly more like gloves than the reference's round bear paws. Paw bottoms sit 0.005–0.01H low.
3. **Leg junction.** This is still the fur pass's residual 4. At v 0.8 the figure is 0.016–0.026H narrower, and the legs are 0.01–0.016H narrower at v 0.9. The longer belly fur now hangs over the step a little more visibly.
4. **v 0.6 (elbows and arm bottoms)** is 0.008–0.017H wider than the reference, as it was before (the left side is 0.008 worse).
5. **Shade side.** It is still lighter than the reference: `#ba998a` vs `#ae927d`.
6. **Crown, right side.** At v 0.05 it is 0.007H narrow, because the tufts make the top outline lumpy.
7. **Orb.** Not touched in this pass. Luminosity, filaments and the cyan chest spill are still the fur pass's residuals 2–3.
8. **Hood depth, eye and smile height, floor.** Not touched; the fur pass's residuals 6, 7 and 9 still apply.
9. **OBJ.** `mascot.obj/.mtl` was not regenerated, as agreed, and still has the fur-pass hands and thumbs.

## Taming pass

This pass covers the owner's review of the poof render:

- **Fullness and length are approved.** I kept the strand count and lengths.
- **Too wild.** The coat read feral: wispy strays at the shoulders and elbows, frizz, a ragged edge. The reference reads groomed.
- **Paws slightly glove-like.** The fingers were more explicit than the reference's.

Everything is in `assets/build_mascot.py` (`FUR`, `PAW`, `comb_fur`, `fur_material`). The render settings are unchanged: Cycles CUDA on the RTX 4060, 512 samples, the same camera, 640×768, 25.2 s. Run the build with

    MUSE_SKIP_OBJ=1 /usr/bin/blender --background --python assets/build_mascot.py

`mascot.obj/.mtl` are still not regenerated, as agreed.

### What made the difference

The dials the poof pass named (clump, tip frizz, the share of short children) were the first step, but they barely moved the edge: stray score 5.31 → 5.17. Three causes were found from crops and A/B renders:

1. **Strand direction.** With emitted hair, a strand's direction is the surface normal plus one fixed comb vector. On the sides, the strands left the skin at about 50° and the random velocity (0.3) scattered them. A comb strong enough to lay the side fur down would push the crown strands into the head. So **`comb_fur` now grooms the parent keys directly.**
   - Each parent starts at its root and arcs from 40° to 12° above the skin, along the surface tangent of "down and slightly back".
   - It gets ±10° of azimuth jitter, so the pile is combed but not machined.
   - It keeps its emitted length, so lengths and fullness are unchanged.
   - Paws are combed toward the orb. Up-facing surfaces lie flatter (`groom_up_flatten` 0.6). Where the comb runs into the surface, an outward comb is blended in, with a backward bias at the crown (0.8). Without the bias the top radiated from a whorl with a visible parting.
   - The hood pile stands up more (60° → 35°), like the reference's plush seen end-on. Lying flat there, it read as flakes.
   - Blender quirks this works around:
     - Every `co_object_set` invalidates the emitter's evaluated copy. A second Python query on the emitter then crashes Blender or returns empty keys. An untouched twin of the emitter (same mesh and seed, so the same roots) supplies the transform, and is then deleted.
     - A write through the emitter's own modifier sets the "edited" flag. Without it, the system re-emits on the next evaluation and silently discards the groom.
     - The saved `.blend` holds the groomed (edited) hair, so change fur settings in the script, not in the UI.
2. **The arm tops.** The arm ellipsoids stand clear of the body, so their up-facing top caps are fully exposed at the shoulders. Their fur fanned out past the outline. These were the shoulder strays. The fix is 0.35× fur length over the top of the arm (z 2.0 → 2.2), plus no arm or leg roots buried inside the body.
3. **Translucent edge.** At melanin 0.05 the Chiang hair is nearly clear. Wherever only a few strands overlap at the outline, they rendered as a pale haze, which is the "lives outdoors" ragged edge. The hair shader now mixes in **60% opaque diffuse `#e4c9a8`**, so the edge stays fur-colored like the reference's faux fur. This was the largest single improvement: stray score 5.39 → 2.54.
4. **Clump noise off.** The Voronoi clump noise (size 0.4) tiled the hood into blocky patches with seams. Without it the crown is smooth and the body keeps its tufts.

### Settings landed on

| | Poof pass | Taming pass |
|---|---|---|
| Strands | 299,500 budget (build reports 299,800) | **unchanged** |
| Length | body 0.085, head 0.042, arms 0.05, paws 0.035, legs 0.07 | **unchanged**, except paws **0.042**, arm-top cap **×0.35**, sole hem (z < 0.15) **×0.6** |
| Clump | 0.75, noise on (0.4) | **0.5, noise off** |
| Tip frizz (roughness end) | 0.02 | **0.012** |
| Random frizz | 0.012 on 60% of children | **0.008 on 40%** (threshold 0.6) |
| Uniform frizz | 0.005 | 0.005 |
| Short children (the ragged half-length strays) | 50% at 0.6× | **30% at 0.75×** |
| Comb | emission: normal + (0, 0.3, −0.72) | **groomed parent keys**: lift 40° → 12°, jitter ±10°, up-facing ×(1 − 0.6·n_z), crown back bias 0.8, hood 60° → 35°, paws 65° → 40° toward the orb |
| Buried roots | – | none for arm or leg roots inside the body |
| Hair shader | Chiang only | Chiang mixed with **60% diffuse `#e4c9a8`** |
| Fingers | radius 0.07, length 0.13, fan +6/−24/−54°, stiffness 5, crease density 5% | **radius 0.08, length 0.10, fan +2/−20/−42°, stiffness 3.5, crease density 70%** |
| Fingertip reach and orb squash | 0.315 m / 0.262 m | **unchanged**, so the contact is unchanged |

### Compare-and-fix log

Each cycle is a full 512-sample render, measured against the reference with the poof pass's scripts:

- figure-normalized silhouette rows
- fringe depth
- edge jitter
- region colors
- gridded paw crops

There is one new metric: **stray**, the number of pixels per edge row that lie outside the morphologically opened silhouette, within 0.06H of it, and differ from the local background on that row. Wisps and frizz raise it. It measures **3.20 on the reference** and repeats within ±0.05. The left fringe depth turned out to be noisy: two renders with identical settings gave 18.9 and 34.9.

| Cycle | Change | Height strict/robust | Stray (ref 3.20) | Shoulder / elbow / body / legs (ref 3.10 / 3.33 / 3.02 / 2.05) | Crown jitter (ref 1.12) | Half-width v 0.40 L/R (ref 0.277/0.275) |
|---|---|---|---|---|---|---|
| before | poof pass | 0.798 / 0.796 | 5.31 | 3.80 / 3.36 / 7.24 / 5.93 | 1.84 | 0.286 / 0.280 |
| 1 | clump 0.6, tip frizz 0.012, random frizz 0.008 on 40%, short children 30% at 0.75×; paw fingers shorter, fatter fan, crease 35%, paw fur 0.042 | 0.799 / 0.796 | 5.17 | 4.14 / 3.14 / 6.55 / 5.60 | 2.03 | 0.279 / 0.277 |
| 2 | parent-key groom | 0.801 / 0.797 | 5.42 | 3.35 / 3.84 / 6.91 / 6.44 | 2.28 | 0.275 / 0.276 |
| 3 | arm-top cap fur ×0.35, no buried arm or leg roots | 0.802 / 0.798 | 5.39 | **2.63** / 4.43 / 7.15 / 6.15 | 2.27 | 0.273 / 0.269 |
| 4 | 60% opaque diffuse; flatter up-facing strands; sole hem | 0.798 / 0.797 | **2.54** | 2.08 / 1.73 / 2.67 / 2.14 | 1.30 | 0.275 / 0.275 |
| 5 | merged fingers (radius 0.08, stiffness 3.5, crease 70%), plush paw lift, warmer diffuse | 0.798 / 0.797 | 2.51 | 2.21 / 2.00 / 2.62 / 1.91 | 1.33 | 0.277 / 0.273 |
| 6 | up-facing flatten 0.85, clump 0.5 | 0.797 / 0.796 | 2.30 | 1.85 / 1.63 / 2.42 / 1.85 | 1.10 | 0.278 / 0.278 |
| 7 | blended crown comb, upright hood pile, clump noise off | 0.796 / 0.794 | 2.20 | 2.64 / 1.35 / 1.96 / 1.73 | 0.96 | 0.278 / 0.280 |
| 8 | flatten 0.7, to recover the crown pixel | 0.796 / 0.796 | 2.07 | 2.17 / 1.43 / 1.97 / 1.71 | 1.06 | 0.278 / 0.277 |
| **final** | flatten 0.6, sole hem ×0.6, crown back bias 0.8 | **0.798 / 0.796** | **2.04** | 2.17 / 1.37 / 1.91 / 1.74 | 1.07 | 0.277 / 0.281 |

Notes per cycle:

- **Cycles 1–3.** The interior looked combed after cycle 2, but the stray score did not fall until cycle 4. The pale translucent band along the outline dominated it until then.
- **Cycle 2.** The first attempt rendered identically to cycle 1. The keys were being re-emitted because the "edited" flag was on the twin, not the emitter. Fixed before the numbers above.
- **Cycle 3** was picked from single-change variants; the rejected and reverted ones are listed below.
- **Cycle 4, first version (60% diffuse `#c8ab9c`).** The belly went mauve (`#b79792`) and the strict height went to 0.805, because the newly opaque crown and leg-hem fur registered as figure. Warming the color to `#e0c8b0`, then `#e4c9a8`, flattening up-facing strands and shortening the hem brought both back.
- **Cycle 5.** The paws had become one dim blob with two sausage fingers.
- **Cycle 6.** The crown outline is right, but blocky tiles remained.
- **Cycle 7.** Clump noise was the cause of the tiles. Height slipped 1 px.
- **Final.** The very top had a dark whorl with a parting; the crown back bias removes it.

**Reverted or rejected, with the measured result:**

| Variant | Result | What I did |
|---|---|---|
| Tip radius ×0.1 → ×0.3 | stray 5.42 → 5.69; strays more visible | Reverted |
| No buried roots on every part, including the body and paws | stray 5.81; bared a pole and dark seam at the shoulder | Reverted. Kept for arms and legs only; that effect was measured only in combination with other changes |
| Arm comb leaning outward 0.5 | strays still fanned past the outline | Reverted |
| Arm comb leaning inward 0.6 | no fix | Reverted |
| Rim softbox 340 → 170 W | stray 4.91, a small gain | Not adopted: the opaque shader fixed the edge without touching the lighting |
| Zero frizz | no measurable change at that stage (cycle 2 base) | Rejected; frizz stays at the cycle 1 values |
| Flatter lift 25° → 4° | no measurable change at that stage | Rejected |
| Clump 0.3 | no measurable change at that stage | Rejected |
| Clump 0.35 (cycle 6 base) | the crown tiles did not change | Rejected; clump stays 0.5 |
| Clump noise size 0.08 | about the same as noise off | Chose noise off |
| Second color warm-up `#e2ccaa` | within a level of `#e4c9a8` | Not adopted |

### Anchors, before → final

| Measure | Reference | Before (poof) | Final |
|---|---|---|---|
| Height / frame, strict | 0.797 | 0.798 | **0.798** |
| Height / frame, robust | 0.797 | 0.796 | **0.796** |
| Background | `#f1eff2` | `#f0efef` | `#f0efef` |
| Lower belly mean | `#c5ab9e` | `#c2a698` | `#c4a598` |
| Shade region | `#b3998a` | `#bda292` | `#c09f91` |
| Lit region | `#c7ab9c` | `#c9af9e` | `#cbad9f` |
| Leg | `#cbae9b` | `#ceb6a8` | `#d0b5a4` |
| Eye centers (v) | 0.223 | 0.227 | 0.226 / 0.227 |
| Orb | – | – | geometry and shader untouched. Visible width reads 0.119 vs 0.122 because the fatter fingers cover a little more of its rim |
| Paw inner edge | touching the orb | touching | **touching**, \|u\| ≈ 0.07, no gap |
| Thumb tops (v) | 0.43 / 0.44 | 0.43 | ≈ 0.43 |
| Paw bottom (v) | 0.595 | 0.60–0.605 | ≈ 0.60 |
| Half-width L/R, v 0.05 | 0.170 / 0.156 | 0.169 / 0.149 | 0.166 / 0.158 |
| v 0.40 | 0.277 / 0.275 | 0.286 / 0.280 | 0.277 / 0.281 |
| v 0.60 | 0.289 / 0.295 | 0.306 / 0.303 | 0.305 / 0.308 |
| v 0.80 | 0.263 / 0.267 | 0.247 / 0.241 | 0.250 / 0.251 |
| v 0.90 | 0.222 / 0.226 | 0.213 / 0.210 | 0.214 / 0.219 |
| Edge jitter, body sides | 1.04 / 1.03 | 1.23 / 1.40 | 1.08 / 0.93 |
| Edge jitter, crown | 1.12 | 1.84 | 1.07 |

The face, eyes, smile, cheeks, orb, camera and lights are unchanged.

### Side by side

It now reads as the same groomed creature. The pile is long, full and combed downward, with crisp small tufts. The outline is clean all the way round, the shoulders are smooth, and the crown is an even dome. The paws read as round paws with a thumb up: the fingers are one soft lobe with faint creases under short plush fur, cradling the orb.

### What still does not match the reference

1. **The color is slightly cool and mauve.** The reference pile is a warmer peach-tan. The region means are within a few levels (lower belly `#c4a598` vs `#c5ab9e`), but G and B both run about 6 low there. The lit head side is paler and less saturated (`#e7d2c4` vs `#e4cab9`), and the legs are lighter (`#d0b5a4` vs `#cbae9b`). Next to each other, the render looks a touch pinker and greyer. Part of this is the violet orb spill on the chest.
2. **The shade side is still too light:** `#c09f91` vs `#b3998a`, slightly worse than before (`#bda292`). The opaque diffuse fills in the shadowed fur.
3. **The edge is now a little cleaner than the reference's.** The stray score is 2.04 against the reference's 3.20. The reference has a fine fuzz of tips that the render lacks in places, so at full size the outline reads slightly velvety rather than long-pile. The lit-side fringe is 48 vs the reference's 71.
4. **The paws are the same tone as the body.** In the reference, the paws are brighter than the body and cast a soft shadow onto the belly, so they pop. Here their outline against the body is softer and they sit back. The fingers are now hinted, as asked, possibly a touch less than the reference's at the right paw.
5. **The arms still read as separate forms.** A crease shows where each arm meets the body at the shoulder. In the reference the arm flows into the body.
6. **Leg junction and elbows (unchanged geometry).** At v 0.8 the figure is 0.013–0.016H narrower, and at v 0.6 it is 0.013–0.016H wider. These are the fur pass's residual 4 and the poof pass's residual 4.
7. **Not touched:**
   - orb luminosity, filaments and the cyan chest spill
   - hood depth
   - eye and smile height (eyes 0.003–0.004H low)
   - floor
   - `mascot.obj/.mtl`, which still has the fur-pass hands and thumbs and no groom
