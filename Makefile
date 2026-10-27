GO ?= go
CARGO ?= cargo
GLSLANG ?= glslangValidator

.PHONY: all run gpu verify native clean
all: guest/build/tracer.wasm host/build/wago-raytracer

guest/build/fullscreen.spv: guest/fullscreen.vert
	mkdir -p guest/build
	$(GLSLANG) -V --target-env vulkan1.0 $< -o $@

guest/build/present.spv: guest/present.frag
	mkdir -p guest/build
	$(GLSLANG) -V --target-env vulkan1.0 $< -o $@

guest/build/tracer.wasm: $(wildcard guest/src/*.rs) guest/Cargo.toml guest/Cargo.lock guest/build/fullscreen.spv guest/build/present.spv guest/.cargo/config.toml
	cd guest && $(CARGO) build --locked --target wasm32-unknown-unknown --release --lib
	cp guest/target/wasm32-unknown-unknown/release/wago_raytracer_guest.wasm $@

host/build/wago-raytracer: $(wildcard host/*.go) go.mod go.sum
	mkdir -p host/build
	CGO_ENABLED=1 $(GO) build -o $@ ./host

native:
	cd guest/native && $(CARGO) build --locked --release

run: all
	./host/build/wago-raytracer

gpu: all
	./host/build/wago-raytracer -require-gpu

verify: run

clean:
	rm -rf guest/build guest/target guest/native/target host/build
