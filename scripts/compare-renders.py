#!/usr/bin/env python3
"""Compare identical Rust tracing code before and after Vulkan display conversion."""
import array
import json
import math
from pathlib import Path
import sys

root = Path(sys.argv[1])
width, height = 640, 400
count = width * height


def floats(name):
    data = (root / name).read_bytes()
    if len(data) != count * 16:
        raise ValueError(f"{name}: expected {count * 16} bytes, got {len(data)}")
    values = array.array("f")
    values.frombytes(data)
    if sys.byteorder != "little":
        values.byteswap()
    if not all(math.isfinite(x) for x in values):
        raise ValueError(f"{name}: non-finite channel")
    return values


native = floats("native.linear.f32")
wago = floats("wago.linear.f32")
rgba = (root / "wago.rgba").read_bytes()
if len(rgba) != count * 4:
    raise ValueError("invalid Vulkan readback length")

tolerance = 1e-5
maximum = [0.0] * 3
different = [0] * 3
display_maximum = [0] * 3
worst = [None] * 3
for i in range(count):
    if native[i * 4 + 3] != 1 or wago[i * 4 + 3] != 1 or rgba[i * 4 + 3] != 255:
        raise ValueError(f"invalid alpha at pixel {i}")
    for c in range(3):
        j = i * 4 + c
        delta = abs(native[j] - wago[j])
        different[c] += delta > tolerance
        if delta > maximum[c]:
            maximum[c] = delta
            worst[c] = {"x": i % width, "y": i // width,
                        "native": native[j], "wago": wago[j]}
        expected = round(math.sqrt(max(0.0, min(1.0, wago[j]))) * 255)
        display_maximum[c] = max(display_maximum[c], abs(rgba[j] - expected))

grid = []
for gy in range(7):
    for gx in range(9):
        x, y = gx * (width - 1) // 8, gy * (height - 1) // 6
        i = (y * width + x) * 4
        grid.append({"x": x, "y": y, "native_linear": list(native[i:i+3]),
                     "wago_linear": list(wago[i:i+3]), "vulkan_rgba": list(rgba[i:i+4])})

labels = {}
for label in ("surfaces", "reflections"):
    a = (root / f"native.{label}").read_bytes()
    b = (root / f"wago.{label}").read_bytes()
    if len(a) != count or len(b) != count:
        raise ValueError(f"invalid {label} length")
    labels[label] = sum(x != y for x, y in zip(a, b))

linear_ok = not any(different)
display_ok = max(display_maximum) <= 1
blue_linear_ok = different[2] == 0
blue_display_ok = display_maximum[2] <= 1
result = {
    "status": "pass" if linear_ok and display_ok and not any(labels.values()) else "fail",
    "width": width, "height": height, "grid_samples": len(grid),
    "linear_tolerance": tolerance, "linear_max_abs_error_rgb": maximum,
    "linear_pixels_over_tolerance_rgb": different, "worst_linear_pixels_rgb": worst,
    "vulkan_max_byte_error_rgb": display_maximum, "ray_label_mismatches": labels,
    "blue_channel_lead": {
        "verdict": "does not reproduce" if blue_linear_ok and blue_display_ok else "difference detected",
        "scope": "same Rust tracer source, native CPU versus Wago, all pixels and a 63-point grid",
        "entry_stage": "guest execution" if not blue_linear_ok else ("Vulkan presentation" if not blue_display_ok else None),
        "old_sample_location": "not supplied; old C guest is outside this comparison",
    },
    "samples": grid,
}
text = json.dumps(result, indent=2) + "\n"
(root / "comparison.json").write_text(text)
print(text, end="")
sys.exit(0 if result["status"] == "pass" else 1)
