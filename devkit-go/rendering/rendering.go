package rendering

import (
	"fmt"
	"image/color"
)

type Frame struct {
	Width   int
	Height  int
	Pixels  []uint8
	Palette []uint8
}

type renderSurface interface {
	Set(x int, y int, color color.Color)
}

func RenderToSurface(surface renderSurface, f *Frame) error {
	if f.Width*f.Height != len(f.Pixels) {
		panic(fmt.Sprintf("Width * Height should be equal to Pixels lentgh, got width=%d, height=%d, len(Pixels)=%d", f.Width, f.Height, len(f.Pixels)))
	}

	if len(f.Palette) != 2*256 {
		panic(fmt.Sprintf("Palette should be 512 bytes, got %d", len(f.Palette)))
	}

	colors := mapColors(f.Palette)

	for y := 0; y < f.Height; y++ {
		for x := 0; x < f.Width; x++ {
			surface.Set(x, y, colors[f.Pixels[y*f.Width+x]])
		}
	}

	return nil
}

func WriteColor(palette []uint8, index int, r uint8, g uint8, b uint8) {
	if len(palette) != 2*256 {
		panic(fmt.Sprintf("Palette should be 512 bytes, got %d", len(palette)))
	}

	palette[index*2] = (r & 0b1111_1000) | ((g & 0b1110_0000) >> 5)
	palette[index*2+1] = ((g & 0b0001_1000) << 3) | ((b & 0b1111_1000) >> 2)
}

func mapColors(palette []uint8) [256]color.Color {
	colors := [256]color.Color{}

	for i := 0; i < 256; i++ {
		colors[i] = mapColor(palette[i*2], palette[i*2+1])
	}

	return colors
}

func mapColor(low uint8, high uint8) color.Color {
	r := low & 0b1111_1000
	g := ((low & 0b0111) << 5) | ((high & 0b1100_0000) >> 3)
	b := (high & 0b0011_1110) << 2

	return color.RGBA{R: r, G: g, B: b, A: 255}
}
