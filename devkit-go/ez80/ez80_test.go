package ez80

import "testing"

func TestAddImmediate(t *testing.T) {
	cpu := Init()
	mem := []uint8{0xC6, 0x80, 0xC6, 0x50, 0xC6, 0x40}

	ticks := cpu.Tick(mem)
	expectTicks(t, ticks, 2)

	if cpu.a.content != 0x80 {
		t.Fatalf("expected accumulator to be 0x80, got %x", cpu.a.content)
	}

	ticks = cpu.Tick(mem)
	expectTicks(t, ticks, 4)

	if cpu.a.content != 0xd0 {
		t.Fatalf("expected accumulator to be 0xd0, got %x", cpu.a.content)
	}

	ticks = cpu.Tick(mem)
	expectTicks(t, ticks, 6)

	if cpu.a.content != 0x10 {
		t.Fatalf("expected accumulator to be 0x10, got %x", cpu.a.content)
	}
}

func expectTicks(t *testing.T, actual uint64, expected uint64) {
	if actual != expected {
		t.Fatalf("expected ticks to be %d, got %d", expected, actual)
	}
}
