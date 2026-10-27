# wago-raytracer

A Rust WebAssembly guest for Wago. The guest traces rays in Wasm, creates the
Vulkan resources, and issues a fullscreen draw through the `vulkan.wasm32`
imports from wago-vulkan. The fragment shader reads the guest's linear-color
storage buffer, applies square-root display gamma, and writes an RGBA8 Vulkan
image. Vulkan copies that image to a readback buffer. The Go host loads Wago and
the imports, calls the guest, verifies the PNG, and saves the evidence.

Ray intersections and shading currently run in the Rust guest on the CPU.
The Vulkan draw runs on the selected Vulkan device. This is not a hardware
ray-tracing implementation.

## Current status

This branch adopts the saved work at `9d1ba93`. The build now selects Rust and
the C guest is removed. `guest/vendor/wire.h` stays only as the upstream ABI
reference, with its license and notice. It is not compiled into the guest.
`guest/src/wire.rs` contains the Rust wasm32 layouts and compile-time layout
checks. `scripts/generate-wire.py` takes the vendored header and a Vulkan SDK
`vulkan_core.h` to regenerate the used layouts and constants. Imports are
maintained in `guest/src/imports.rs`.

The first Rust build failed with exit status 101 before the migration edits.
Its stderr is saved in `docs/rust-first-build.log`. The generator emitted the
Rust keyword `type` without an escape and copied C checks for the wasm64 ABI.
Those source errors have been corrected. A passing build has not yet been
recorded. The global measurement hold stopped builds, tests, and rendering.
The source edits and host test package are prepared, but milestone 1 is not
validated.

The container's recorded device is **llvmpipe (LLVM 19.1.7, 256 bits)**, Vulkan
device type 4 (CPU). The RTX 4060 is not reachable from this container. See
[the recorded device check](docs/device-check.md). No new render was made in
this continuation. `out/legacy-c-frame.png` and its JSON report are preserved
outputs from the old C guest. They do not validate the Rust guest.

## Host-agent test

**HOLD: do not start a host agent, build, test, or render until the measurement
coordinator explicitly says `WINDOW CLOSED`.** The host shares the held laptop.
The test script exits with status 75 unless `WAGO_TEST_WINDOW=CLOSED` is set.
Set that variable only after the explicit release. No host work is queued by
this repository.

The host needs Bash, Python 3, GNU Make, Rust 1.99.0 with rustup, Go 1.27.3,
a C compiler for CGO, `glslangValidator`, a Vulkan loader, and the NVIDIA Vulkan
driver that exposes the RTX 4060. `mise.toml` pins Rust and Go. Dependencies
must be available before the unattended run. Cargo and Go can download their
pinned packages during the build.

After the explicit release, the host agent runs this one command from the
repository root:

```sh
WAGO_TEST_WINDOW=CLOSED bash scripts/host-test.sh
```

The script builds the shaders, Rust Wasm guest, Go host, and native reference.
It then runs the whole test without prompts. It uses an offscreen Vulkan image;
no display server, window controls, or screenshot tool is needed. The saved PNG
contains the Vulkan readback. `-require-gpu` rejects CPU Vulkan devices, and the
script also checks for the name `RTX 4060` and device type 2 (discrete GPU).
If needed, the host agent can set the Vulkan loader's ICD selection environment
before the command to select its installed NVIDIA driver.

The optional `--built` argument skips compilation and uses binaries already
built for the host OS and architecture. Binary hashes are recorded in either
mode. The default command forces a build from the current source so that old
build outputs cannot be used as evidence for a new commit.

All test evidence goes under `out/host-run/`:

- `run.log`: build and execution stdout and stderr.
- `status.json`: final exit status, including a failed build or render.
- `build.json`: source commit, working-tree state, platform, and artifact hashes.
- `frame.png`: the 640 by 400 postcard from the Rust guest and Vulkan draw.
- `frame.png.json`: actual device, guest hash, distinct color count, nonblack
  pixel count, and RGBA samples for sky, ground, matte, metal, and glass.
  Metal samples include primary rays whose reflection hits sky and ground.
- `native.linear.f32` and `wago.linear.f32`: linear RGBA float32 values in
  little-endian order for every pixel, before Vulkan display conversion.
- `wago.rgba`: the Vulkan RGBA8 readback. Surface and reflection label files
  locate each ray sample and check native versus Wasm ray classification.
- `comparison.json`: 63 fixed grid samples, full-image errors by channel,
  ray-label mismatches, and the blue-channel verdict.
- `run.json`: success summary, device, frame path, and pixel samples. This file
  is written only after all stages pass. `status.json` is authoritative on error.

The PNG check requires exactly 640 by 400 pixels, opaque alpha, at least 1024
colors, at least 90 percent nonblack pixels, all five scene regions, and both
sky and ground reflections in the metal sphere. It decodes the encoded PNG
before checking the result. The host agent should return the full evidence
directory, including failed-run logs if the script exits nonzero.

## Blue-channel check

The native reference includes the exact same `guest/src/tracer.rs` source as
the Wasm guest, uses the same fixed RNG seeds, camera, and 32 samples per pixel,
and writes all linear RGBA values. The comparison checks all 256000 pixels and
prints a fixed 9 by 7 grid. A linear channel error above 0.00001 is reported as
a mismatch. A Vulkan byte error above 1 against the expected clamped square-root
conversion is a presentation mismatch. It also rejects non-finite values,
incorrect alpha, and different surface or reflection labels.

The earlier blue values, native 0.89959323 versus Wago 0.48589650, are an
**untested lead**. Their pixel location was not supplied. No verdict is claimed
until host evidence returns. A new difference before upload points to guest
execution or compilation. A difference only after conversion points to the
upload, shader, draw, or readback path. Those stages need further isolation if
a difference occurs; the comparison does not assume a runtime defect.

## Scene 2 and performance

After milestone 1 passes on the host, import `assets/mascot.obj` with
`assets/scene.json` from main and save the second render as `out/mascot.png`.
The known asset commit is `3f26df0`; a separate model review can update it.
Scene 2 has not started, and no asset merge or model rendering is claimed here.

The migration edits do not change the saved Rust ray loop or its fixed pixel
buffers. Raw evidence capture adds one 4096000-byte host copy after rendering
when `-evidence` is enabled. It does not add per-ray allocations. Runtime and
peak memory have not been measured because of the hold. The host handoff must
establish functional results before any later performance measurements obtain
an approved measurement window.
