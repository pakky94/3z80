package ez80

import "fmt"

type ez80 struct {
	a  register8
	bc register24
	de register24
	hl register24

	ix register24
	iy register24

	mbase register8
	pc    register24

	ticks uint64
}

func Init() ez80 {
	return ez80{
		a:  register8{},
		bc: newRegister24(),
		de: newRegister24(),
		hl: newRegister24(),

		ix: newRegister24(),
		iy: newRegister24(),

		pc: newRegister24(),

		ticks: 0,
	}
}

func (cpu *ez80) Tick(mem []uint8) uint64 {
	pc := cpu.ProgramCounter()
	opcode := mem[pc]

	switch opcode {
	case 0xC6:
		cpu.a.content += mem[pc+1]
		cpu.pc.setLower16(cpu.pc.lower16() + 2)
		cpu.ticks += 2
		return cpu.ticks
	default:
		panic(fmt.Sprintf("Unimplemented opcode %x", opcode))
	}
}

func (cpu *ez80) ProgramCounter() int {
	return (int(cpu.mbase.content) << 16) | int(cpu.pc.lower16())
}
