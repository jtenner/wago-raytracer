# Muse mascot: taste review (Opus)

Compared `reference/muse-mascot.jpg` against the original `assets/preview.png` render
(commit 3f26df0), then applied the small fixes below and re-rendered.

## Verdict

The original build had the right shape: a tall, soft pill body with an inset cream
face, two mitten hands holding an orb, and stubby feet. The proportions and character
read correctly at a glance. It missed in five places, though, and together they made
it feel like a **vinyl toy in a gray room holding a beach ball**, not a plush in a
bright studio holding a nebula:

1. **The orb read as a marble or beach ball.** It was about 40% too big. Five regular
   white sine bands looked like candy stripes, it had no rim glow, and the pastel
   noise was too fine-grained.
2. **The lighting was flat gray.** The background sampled at `#c3c1bf` against the
   reference's `#f3f1f4`. The reference is a high-key white cyc.
3. **The colors were washed out.** The body rendered gray-pink (`#c3b7b1` vs.
   reference `#ccb0a0`), and the face was nearly the same value as the body, so the
   face inset barely read.
4. **The face was too large.** It took about 85% of the head width (the reference is
   about 72%), and the blush was a hard-edged decal disc.
5. **The feet were detached egg shoes.** They splayed forward under a body that
   seemed to hover above them.

## What works (kept)

- The overall silhouette: a pill body, a head continuous with the body, and the arms
  hugging the sides.
- The eyes: glossy black with catchlights, well spaced, at a good height on the face.
- The smile: a small, thin, shallow U, true to the reference.
- The mitten hands with raised thumbs, which match the reference's cupping gesture.
- The camera angle: slightly high, three-quarter frontal, with a long lens.

## What is still off (ranked; not fixed here)

1. **No fur.** This is the biggest remaining gap. Sheen and bump make the surface read
   as velvet or felt at best. The reference is long-pile faux fur with a visible
   fuzzy silhouette. Fix: add a hair particle system on Body, Arms, Hands, Thumbs and
   Feet (about 0.035 m length, 0.003 m root radius, 60k–120k children, Principled Hair
   BSDF melanin about 0.25, tinted `#c9a07e`, random roughness 0.3). This is not a small
   change. OBJ export won't carry it, and CPU render time will grow several times
   over, so it needs an owner decision.
2. **The arms read as swollen sausages.** They bulge outward at the elbows. In the
   reference the arms are slimmer and hug the body. Fix: shrink the arm scale from
   `(0.265, 0.29, 0.53)` to about `(0.22, 0.26, 0.50)` and move the center from
   x ±0.855 to about ±0.80.
3. **The feet are still ellipsoids.** The reference has short cylindrical legs with
   rounded soles and a clear gap between them. Fix: replace them with capsules
   (radius 0.27, height 0.45, centers at x ±0.38, base z 0) and narrow the gap.
4. **The orb wisps are a little worm-like.** The 14 surface wisps work, but a real
   nebula has depth. Fix: give the core a volume shader (Principled Volume with
   emission driven by the same warped-noise ramp, density about 2), and cut the
   surface wisps to about 8 with a thinner radius of 0.0018.
5. **The orb halo is weaker than the reference's.** Fix: raise the glare to
   `size = 8` and lower the threshold to about 0.9, or add a faint emissive halo sphere
   (r 0.30, Volume Emission `#9a6cff` at 0.4, density 0.05).
6. **The head top is a little pointed.** The reference crown is a flatter, broader
   dome. Fix: raise the lathe `power` from 0.73 to about 0.62, or scale the upper
   half's X to 1.04.
7. **The body still leans slightly pink.** It samples `#d5beb2`, against the
   reference's `#ccb0a0`. The orb spill and the white sheen both push it pink. Fix:
   set the plush `Sheen Tint` to `#e8d2b8` instead of white.

## Changes applied in `build_mascot.py`

| Area | Before | After | Why |
|---|---|---|---|
| Body color | `#d9c6b2`, sheen 0.32, bump 0.24 | `#c9a07e`, sheen 1.0, bump 0.4 | AgX desaturates. The new color renders close to the reference beige, and the extra sheen and bump give a soft, fuzzy rim |
| Face color | `#f2ddc9` | `#f6c9a6` | A warm peach that separates from the body |
| Face size | scale `(0.70, 0.17, 0.535)` | `(0.62, 0.17, 0.49)` | Matches the reference's face-to-head ratio |
| Cheeks | x ±0.433, `#e8b3a6`, hard disc | x ±0.37, scale `(0.12, 0.02, 0.08)`, `#f0a59a`, radial alpha fade (new `soften_edges`) | Moved in to stay on the smaller face, and airbrushed instead of a decal |
| Orb size and position | core r 0.306, shell 0.35, center z 1.94 | core r 0.225, shell 0.255, center (0, −0.93, 1.88) | About 25% of body width as in the reference, with a clear gap below the chin |
| Orb shading | ramp drove base and emission, emission 1.35, noise scale 3.8/4.2 | dark base `#0d0a1c`, emission-only ramp at 1.1, noise scale 1.7/2.3, deeper stops, white-lavender fresnel rim | Larger swirls, saturated blue/magenta/cyan, and a white-hot edge. Studio light no longer washes it to pastel |
| Orb filaments | 5 regular spiral bands, r 0.0038, emission 3 | 14 irregular wisps, r 0.0026, pink `#ff6fd8` and cyan `#45e0ff`, emission 5 | Reads as plasma, not beach-ball stripes |
| Glass shell | coat 0.25 | coat 0.08 | Less milky film |
| Orb light spill | 7 W | 16 W | Violet light now spills onto the hands and chest as in the reference |
| Hands and thumbs | hands x ±0.645 z 1.66; thumbs x ±0.50 z 1.86 | hands x ±0.575 z 1.64; thumbs x ±0.42 z 1.83 | They cradle the smaller orb |
| Feet | scale `(0.325, 0.43, 0.205)` at z 0.205 | scale `(0.29, 0.32, 0.30)` at z 0.30 | Taller and less splayed, so they read as stubby legs carrying the body |
| Studio | world `#cdd1d7` at 0.35, floor `#ecebea` | world `#f4f3f7` at 1.2, floor and cyc `#f6f5f8` | High-key white like the reference |
| Camera | 64 mm | 58 mm | A little more breathing room around the figure |

I reran the script with `/usr/bin/blender --background --python assets/build_mascot.py`,
which regenerated `mascot.blend`, `mascot.obj/.mtl`, `scene.json` and `preview.png`.

Sampled after the fix (reference in parentheses): body `#d5beb2` (`#ccb0a0`), face
`#f3e6de` (`#f4d8c2`), background `#e8e7e8` (`#f3f1f4`). The colors are now in the
right family. The remaining difference is mostly the fur's self-shadowing, which this
model doesn't have.
