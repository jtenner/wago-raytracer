// The host loads Wago and its Vulkan imports. Scene math and Vulkan commands live in the guest.
package main

import (
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"image"
	"image/png"
	"os"
	"path/filepath"
	"runtime/debug"

	vulkan "github.com/jtenner/wago-vulkan"
	wago "github.com/wago-org/wago"
)

const width, height = 640, 400

type sample struct {
	Label string   `json:"label"`
	X     int      `json:"x"`
	Y     int      `json:"y"`
	RGBA  [4]uint8 `json:"rgba"`
}

type report struct {
	Scene          string   `json:"scene"`
	Device         string   `json:"device"`
	DeviceType     uint32   `json:"device_type"`
	Execution      string   `json:"execution"`
	GuestSHA256    string   `json:"guest_sha256"`
	Wago           string   `json:"wago"`
	Vulkan         string   `json:"wago_vulkan"`
	Width          int      `json:"width"`
	Height         int      `json:"height"`
	DistinctColors int      `json:"distinct_colors"`
	NonblackPixels int      `json:"nonblack_pixels"`
	Samples        []sample `json:"samples"`
}

func exported(in *wago.Instance, name string) (uint32, error) {
	r, err := in.Invoke(name)
	if err != nil {
		return 0, err
	}
	if len(r) != 1 {
		return 0, fmt.Errorf("%s: expected one result", name)
	}
	return uint32(r[0]), nil
}

func read(in *wago.Instance, name string, n uint32) ([]byte, error) {
	p, err := exported(in, name)
	if err != nil {
		return nil, err
	}
	b, ok := in.Read(p, n)
	if !ok {
		return nil, fmt.Errorf("%s: guest memory range is invalid", name)
	}
	return bytes.Clone(b), nil
}

func inspect(img image.Image, ids, reflected []byte, r *report) error {
	r.Width, r.Height = img.Bounds().Dx(), img.Bounds().Dy()
	if r.Width != width || r.Height != height {
		return fmt.Errorf("unexpected dimensions %dx%d", r.Width, r.Height)
	}
	colors := make(map[[4]uint8]struct{})
	pixel := func(i int) [4]uint8 {
		r, g, b, a := img.At(i%width, i/width).RGBA()
		return [4]uint8{uint8(r >> 8), uint8(g >> 8), uint8(b >> 8), uint8(a >> 8)}
	}
	for i := 0; i < width*height; i++ {
		p := pixel(i)
		if p[3] != 255 {
			return fmt.Errorf("pixel %d has alpha %d", i, p[3])
		}
		colors[p] = struct{}{}
		if p[0] != 0 || p[1] != 0 || p[2] != 0 {
			r.NonblackPixels++
		}
	}
	r.DistinctColors = len(colors)
	if r.DistinctColors < 1024 || r.NonblackPixels < width*height*9/10 {
		return fmt.Errorf("empty or nearly black image: %d colors, %d nonblack pixels", r.DistinctColors, r.NonblackPixels)
	}
	// Use the primary and reflected ray labels computed by the guest to locate samples.
	// Choose an interior point near each region's center, rather than a silhouette edge.
	choose := func(label string, surface int, reflection int) bool {
		valid := func(i int) bool { return int(ids[i]) == surface && (reflection < 0 || int(reflected[i]) == reflection) }
		sx, sy, n := 0, 0, 0
		for y := 2; y < height-2; y++ {
			for x := 2; x < width-2; x++ {
				i := y*width + x
				if valid(i) && valid(i-2) && valid(i+2) && valid(i-2*width) && valid(i+2*width) {
					sx += x
					sy += y
					n++
				}
			}
		}
		if n == 0 {
			return false
		}
		cx, cy := sx/n, sy/n
		best, distance := -1, width*width+height*height
		for y := 2; y < height-2; y++ {
			for x := 2; x < width-2; x++ {
				i := y*width + x
				d := (x-cx)*(x-cx) + (y-cy)*(y-cy)
				if d < distance && valid(i) && valid(i-2) && valid(i+2) && valid(i-2*width) && valid(i+2*width) {
					best = i
					distance = d
				}
			}
		}
		r.Samples = append(r.Samples, sample{label, best % width, best / width, pixel(best)})
		return true
	}
	for id, label := range []string{"sky", "ground", "matte", "metal", "glass"} {
		if !choose(label, id, -1) {
			return fmt.Errorf("scene is missing visible %s pixels", label)
		}
	}
	if !choose("metal_reflects_sky", 3, 0) || !choose("metal_reflects_ground", 3, 1) {
		return fmt.Errorf("metal does not show both sky and ground reflections")
	}
	choose("metal_reflects_matte", 3, 2)
	choose("metal_reflects_glass", 3, 4)
	return nil
}

func render(wasm, output string, requireGPU bool) (err error) {
	data, err := os.ReadFile(wasm)
	if err != nil {
		return err
	}
	ctx := context.Background()
	rt := wago.NewRuntime(wago.WithRuntimeConfig(wago.NewRuntimeConfig().WithCoreFeatures(wago.SupportedFeatures() & wago.CoreFeaturesV3)))
	defer rt.Close()
	if err = rt.LoadPlugins(ctx, vulkan.PluginSet()); err != nil {
		return err
	}
	mod, err := rt.Compile(data)
	if err != nil {
		return fmt.Errorf("compile guest: %w", err)
	}
	defer mod.Close()
	in, err := rt.Instantiate(ctx, mod)
	if err != nil {
		return fmt.Errorf("instantiate guest: %w", err)
	}
	defer in.Close()
	defer func() {
		result, e := in.Invoke("cleanup")
		if e != nil {
			err = errors.Join(err, fmt.Errorf("guest cleanup: %w", e))
		} else if int32(result[0]) != 0 {
			err = errors.Join(err, fmt.Errorf("guest cleanup VkResult: %d", int32(result[0])))
		}
	}()
	var gpu uint64
	if requireGPU {
		gpu = 1
	}
	result, err := in.Invoke("render", gpu)
	if err != nil {
		return fmt.Errorf("guest render: %w", err)
	}
	if code := int32(result[0]); code != 0 {
		line, _ := exported(in, "error_line")
		return fmt.Errorf("guest render failed: code %d at guest/render.c:%d (require-gpu=%t; -100000 means an unmet guest requirement)", code, line, requireGPU)
	}
	frame, err := read(in, "frame", width*height*4)
	if err != nil {
		return err
	}
	ids, err := read(in, "surface_ids", width*height)
	if err != nil {
		return err
	}
	reflected, err := read(in, "reflection_ids", width*height)
	if err != nil {
		return err
	}
	name, err := read(in, "device_name", 256)
	if err != nil {
		return err
	}
	if i := bytes.IndexByte(name, 0); i >= 0 {
		name = name[:i]
	}
	kind, err := exported(in, "device_type")
	if err != nil {
		return err
	}
	hash := sha256.Sum256(data)
	r := report{Scene: "postcard", Device: string(name), DeviceType: kind, Execution: "software Vulkan", GuestSHA256: hex.EncodeToString(hash[:])}
	if kind == 1 || kind == 2 {
		r.Execution = "hardware Vulkan"
	}
	if info, ok := debug.ReadBuildInfo(); ok {
		for _, dep := range info.Deps {
			if dep.Path == "github.com/wago-org/wago" {
				r.Wago = dep.Version
			}
			if dep.Path == "github.com/jtenner/wago-vulkan" {
				r.Vulkan = dep.Version
			}
		}
	}
	img := &image.RGBA{Pix: frame, Stride: width * 4, Rect: image.Rect(0, 0, width, height)}
	var encoded bytes.Buffer
	if err = png.Encode(&encoded, img); err != nil {
		return err
	}
	// Verify the actual PNG bytes, not just the input buffer.
	decoded, err := png.Decode(bytes.NewReader(encoded.Bytes()))
	if err != nil {
		return err
	}
	if err = inspect(decoded, ids, reflected, &r); err != nil {
		return err
	}
	if err = os.MkdirAll(filepath.Dir(output), 0755); err != nil {
		return err
	}
	if err = os.WriteFile(output, encoded.Bytes(), 0644); err != nil {
		return err
	}
	metadata, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		return err
	}
	metadata = append(metadata, '\n')
	if err = os.WriteFile(output+".json", metadata, 0644); err != nil {
		return err
	}
	fmt.Printf("%s\n%s", output, metadata)
	return nil
}

func main() {
	wasm := flag.String("guest", "guest/build/tracer.wasm", "Wasm guest path")
	output := flag.String("out", "out/frame.png", "PNG output path")
	gpu := flag.Bool("require-gpu", false, "require a discrete or integrated GPU; reject software Vulkan")
	flag.Parse()
	if err := render(*wasm, *output, *gpu); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
