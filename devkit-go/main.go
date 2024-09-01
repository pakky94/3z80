package main

import (
	"devkit-go/rendering"

	"github.com/veandco/go-sdl2/sdl"
)

func main() {
	if err := sdl.Init(sdl.INIT_EVERYTHING); err != nil {
		panic(err)
	}
	defer sdl.Quit()

	window, err := sdl.CreateWindow("test", sdl.WINDOWPOS_UNDEFINED, sdl.WINDOWPOS_UNDEFINED, 800, 600, sdl.WINDOW_SHOWN)
	if err != nil {
		panic(err)
	}
	defer window.Destroy()

	surface, err := sdl.CreateRGBSurface(0, 200, 150, 32, 0, 0, 0, 0)

	renderer, err := sdl.CreateRenderer(window, -1, sdl.RENDERER_TARGETTEXTURE)
	if err != nil {
		panic(err)
	}

	defer renderer.Destroy()

	rect := sdl.Rect{X: 0, Y: 0, W: 200, H: 150}
	rect_dest := sdl.Rect{X: 0, Y: 0, W: 800, H: 600}
	pixels := pixels()
	palette := make([]uint8, 512, 512)
	p := 0
	i := 0
	fillPalette(palette, 1)

	running := true
	for running {
		for event := sdl.PollEvent(); event != nil; event = sdl.PollEvent() {
			switch event.(type) {
			case *sdl.QuitEvent: // NOTE: Please use `*sdl.QuitEvent` for `v0.4.x` (current version).
				println("Quit")
				running = false
				break
			}
		}

		if i == 0 {
			fillPalette(palette, p)
			p = (p + 1) % 3
		}
		i = (i + 1) % 100

		rendering.RenderToSurface(surface, &rendering.Frame{
			Width:   200,
			Height:  150,
			Pixels:  pixels,
			Palette: palette,
		})

		texture, err := renderer.CreateTextureFromSurface(surface)
		if err != nil {
			panic(err)
		}

		// renderer.SetDrawColor(0, 128, 0, 128)
		// renderer.Clear()

		if err = renderer.Copy(texture, &rect, &rect_dest); err != nil {
			panic(err)
		}
		renderer.Flush()
		texture.Destroy()

		window.UpdateSurface()

		sdl.Delay(33)
	}
}

func pixels() []uint8 {
	p := make([]uint8, 200*150, 200*150)

	for y := 0; y < 150; y++ {
		for x := 0; x < 200; x++ {
			// p[y*200+x] = uint8((x + y) * 200 / 350)
			p[y*200+x] = uint8(x + y)
		}
	}

	return p
}

func fillPalette(palette []uint8, color int) {
	for i := 0; i <= 255; i++ {
		if color == 0 {
			rendering.WriteColor(palette, i, uint8(i), 0, 0)
		} else if color == 1 {
			rendering.WriteColor(palette, i, 0, uint8(i), 0)
		} else {
			rendering.WriteColor(palette, i, 0, 0, uint8(i))
		}
	}
}
