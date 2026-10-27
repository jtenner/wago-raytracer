CLANG ?= /usr/bin/clang
GO ?= go
GLSLANG ?= glslangValidator

.PHONY: all run gpu verify clean
all: guest/build/tracer.wasm host/build/wago-raytracer

guest/build/fullscreen.spv: guest/fullscreen.vert
	mkdir -p guest/build
	$(GLSLANG) -V --target-env vulkan1.0 $< -o $@

guest/build/present.spv: guest/present.frag
	mkdir -p guest/build
	$(GLSLANG) -V --target-env vulkan1.0 $< -o $@

guest/build/shaders.h: guest/build/fullscreen.spv guest/build/present.spv scripts/embed-shaders.py
	python3 scripts/embed-shaders.py

guest/build/tracer.wasm: guest/render.c guest/tracer.c guest/tracer.h guest/imports.h guest/vendor/wire.h guest/build/shaders.h
	$(CLANG) --target=wasm32 -std=c11 -O2 -ffreestanding -fno-builtin -nostdlib \
		-idirafter /usr/include -Iguest/build -Wall -Wextra -Werror \
		-Wl,--no-entry -Wl,--export-memory -Wl,-z,stack-size=262144 \
		-Wl,--initial-memory=8388608 -Wl,--max-memory=16777216 \
		guest/render.c guest/tracer.c -o $@

host/build/wago-raytracer: $(wildcard host/*.go) go.mod go.sum
	mkdir -p host/build
	CGO_ENABLED=1 $(GO) build -o $@ ./host

run: all
	./host/build/wago-raytracer

gpu: all
	./host/build/wago-raytracer -require-gpu

verify:
	python3 scripts/verify-image.py out/frame.png

clean:
	rm -rf guest/build host/build
