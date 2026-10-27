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
