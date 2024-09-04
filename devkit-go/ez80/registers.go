package ez80

type register8 struct {
	content uint8
}

type register24 struct {
	content [3]uint8
}

func newRegister24() register24 {
	return register24{
		content: [3]uint8{0, 0, 0},
	}
}

func (r *register24) low() uint8 {
	return r.content[0]
}

func (r *register24) mid() uint8 {
	return r.content[1]
}

func (r *register24) high() uint8 {
	return r.content[2]
}

func (r *register24) lower16() uint16 {
	return (uint16(r.mid()) << 8) | uint16(r.low())
}

func (r *register24) setLow(val uint8) {
	r.content[0] = val
}

func (r *register24) setMid(val uint8) {
	r.content[1] = val
}

func (r *register24) setHigh(val uint8) {
	r.content[2] = val
}

func (r *register24) setLower16(val uint16) {
	r.content[0] = uint8(val)
	r.content[1] = uint8(val >> 8)
}
