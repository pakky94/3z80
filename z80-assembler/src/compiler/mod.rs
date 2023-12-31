use crate::compiler::instructions::{
    compile_instruction, label_not_found, CompileError, CompileErrorType, Placeholder,
    PlaceholderType,
};
use crate::compiler::macros::compile_macro;
use crate::compiler::r#macro::Macro;
pub use crate::compiler::source_provider::{InMemorySourceProvider, SourceHeader, SourceProvider};
use crate::compiler::utilities::relative_delta;
use crate::domain::{Argument, Instruction, ParseItem};
use crate::parser::tokenizer::BufferedTokenizer;
use crate::parser::Parser;
use std::collections::HashMap;

mod instructions;
mod r#macro;
mod macros;
mod source_provider;
mod utilities;

pub struct Compiler<T>
where
    T: SourceProvider,
{
    source_provider: T,
    out: Vec<u8>,
    idx: usize,
    label_map: HashMap<String, usize>,
    placeholders: Vec<Placeholder>,
    constants: HashMap<String, u16>,
    macros: HashMap<String, Macro>,
}

impl<T> Compiler<T>
where
    T: SourceProvider,
{
    pub fn new(source_provider: T, capacity: usize) -> Self {
        Compiler {
            source_provider,
            out: vec![0u8; capacity],
            idx: 0,
            label_map: HashMap::new(),
            placeholders: vec![],
            constants: HashMap::new(),
            macros: HashMap::new(),
        }
    }

    pub fn compile(mut self) -> Result<Vec<u8>, CompileError> {
        for file in self.source_provider.file_list() {
            self.constants.clear();
            let source = self.source_provider.source(&file.filename);
            let mut tokenizer = BufferedTokenizer::new(&source, 0);
            let mut parser = Parser::new();

            loop {
                if let Some(pi) = parser.parse_next(&mut tokenizer)? {
                    self.process_item(pi, &mut tokenizer)?;
                } else {
                    break;
                }
            }
        }

        for ph in self.placeholders.into_iter() {
            let addr = self
                .label_map
                .get(ph.label.as_str())
                .ok_or(label_not_found(&ph))?;

            match ph.ph_type {
                PlaceholderType::ShortValue => {
                    self.out[ph.idx] = self.out[*addr];
                }
                PlaceholderType::WideValue => {
                    self.out[ph.idx] = self.out[*addr];
                    self.out[ph.idx + 1] = self.out[*addr + 1];
                }
                PlaceholderType::AbsAddress => {
                    self.out[ph.idx] = (*addr % 256) as u8;
                    self.out[ph.idx + 1] = (*addr / 256) as u8
                }
                PlaceholderType::RelAddress => {
                    self.out[ph.idx] = relative_delta(ph.idx + 1, *addr).ok_or(CompileError {
                        error: CompileErrorType::UnableToCalculateRelativeJump(ph.clone()),
                        instr: None,
                    })?;
                }
                t => panic!("Invalid placeholder type: {:?}", t),
            }
        }

        Ok(self.out)
    }

    fn process_item(
        &mut self,
        item: ParseItem,
        tokenizer: &mut BufferedTokenizer,
    ) -> Result<(), CompileError> {
        Ok(match item {
            ParseItem::Label(l) => {
                self.label_map.insert(l.name, self.idx);
            }
            ParseItem::Instruction(inst) => {
                let (inst, p0, p1) = self.extract_placeholders(inst);
                let inst = self.replace_constants(inst)?;
                let data = compile_instruction(&inst, p0, p1, &mut self.placeholders).map_err(
                    |mut err| {
                        err.instr = Some(inst.clone());
                        err
                    },
                )?;
                for i in 0..data.len {
                    self.out[self.idx] = data.data[i as usize];
                    self.idx += 1;
                }
            }
            ParseItem::Data(data) => {
                for b in data.iter() {
                    self.out[self.idx] = *b;
                    self.idx += 1;
                }
            }
            ParseItem::Constant(cons) => {
                self.constants.insert(cons.name, cons.value);
            }
            ParseItem::Directive(cmd, tokens) => {
                compile_macro(cmd, tokens, tokenizer, &mut self.macros)?
            }
        })
    }

    fn replace_constants(&self, inst: Instruction) -> Result<Instruction, CompileError> {
        let arg0 = self.try_parse_constant(&inst.arg0, &inst)?;
        let arg1 = self.try_parse_constant(&inst.arg1, &inst)?;
        Ok(Instruction {
            opcode: inst.opcode,
            arg0: arg0.unwrap_or(inst.arg0),
            arg1: arg1.unwrap_or(inst.arg1),
            line: inst.line,
            file_id: inst.file_id,
        })
    }

    fn try_parse_constant(
        &self,
        arg: &Argument,
        inst: &Instruction,
    ) -> Result<Option<Argument>, CompileError> {
        if let Argument::Constant(c) = arg {
            Ok(Some(Argument::Value(
                *self.constants.get(c.as_str()).ok_or(CompileError {
                    error: CompileErrorType::ConstantNotFound(c.clone()),
                    instr: Some(inst.clone()),
                })?,
            )))
        } else {
            Ok(None)
        }
    }

    fn extract_placeholders(&mut self, inst: Instruction) -> (Instruction, isize, isize) {
        let (arg0, p0) = self.try_extract_placeholder(&inst.arg0, inst.line);
        let (arg1, p1) = self.try_extract_placeholder(&inst.arg1, inst.line);
        (
            Instruction {
                opcode: inst.opcode,
                arg0: arg0.unwrap_or(inst.arg0),
                arg1: arg1.unwrap_or(inst.arg1),
                line: inst.line,
                file_id: inst.file_id,
            },
            p0,
            p1,
        )
    }

    fn try_extract_placeholder(
        &mut self,
        arg: &Argument,
        line: usize,
    ) -> (Option<Argument>, isize) {
        match arg {
            Argument::LabelAddress(s) => {
                self.placeholders.push(Placeholder {
                    idx: self.idx,
                    label: s.clone(),
                    ph_type: PlaceholderType::Undefined,
                    line,
                });
                (
                    Some(Argument::DirectAddress(0)),
                    isize::try_from(self.placeholders.len()).unwrap() - 1,
                )
            }
            Argument::LabelValue(s) => {
                self.placeholders.push(Placeholder {
                    idx: self.idx,
                    label: s.clone(),
                    ph_type: PlaceholderType::Undefined,
                    line,
                });
                (
                    Some(Argument::Value(0)),
                    isize::try_from(self.placeholders.len()).unwrap() - 1,
                )
            }
            _ => (None, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::instructions::{CompileError, CompileErrorType};
    use crate::compiler::source_provider::{InMemorySourceProvider, SourceHeader};
    use crate::Compiler;

    #[test]
    #[rustfmt::skip]
    fn test_compile_inst_1() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), },
                r#"
ld A, C
ld b, 12h
"#.to_string(),
            )],
        }, 1024);

        compare_memory(
            vec![
                0b01111001,
                0b00000110,
                0b00010010
            ],
            compiler.compile().unwrap(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_compile_labels() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), },
                r#"
.label1: 12h
.label2: 13h
.label3: 14h
.label4: 15h
ld b, *label3
ld a, &label4
ld a, (BC)
ld a, (DE)
ld a, (ABCDh)
ld a, (HL)
ld e, (IX + 5h)
ld l, (IY + a3h)
"#.to_string(),
            )],
        }, 128);

        compare_memory(
            vec![
                0b00010010, // label1
                0b00010011, // label2
                0b00010100, // label3
                0b00010101, // label4
                0b00000110, // ld b, *label3
                0b00010100,
                0b00111010, // ld a, &label4
                0b00000011,
                0b00000000,
                0b00001010, // ld a, (BC)
                0b00011010, // ld a, (DE)
                0b00111010, // ld a, (ABCDh)
                0b11001101,
                0b10101011,
                0b01111110, // ld a, (HL)
                0b11011101, // ld e, (IX + 5h)
                0b01011110,
                0b00000101,
                0b11111101, // ld l, (IY + a3h)
                0b01101110,
                0b10100011,
            ],
            compiler.compile().unwrap(),
        );
    }

    #[test]
    fn label_not_found_error() {
        let compiler = Compiler::new(
            InMemorySourceProvider {
                files: vec![(
                    SourceHeader {
                        filename: "main.z80".to_string(),
                    },
                    r#"
.label1: 12h
ld a, *missing_label
"#
                    .to_string(),
                )],
            },
            1024,
        );

        assert_eq!(
            CompileError {
                error: CompileErrorType::LabelNotFound("missing_label".to_string(), 3),
                instr: None,
            },
            compiler.compile().unwrap_err()
        )
    }

    #[test]
    #[rustfmt::skip]
    fn test_compile_wide_registers() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), },
                r#"
ld HL, 1234h
ld IX, 2345h
"#.to_string(),
            )],
        }, 1024);

        compare_memory(
            vec![
                0b00100001,
                0b00110100,
                0b00010010,
                0b11011101,
                0b00100001,
                0b01000101,
                0b00100011,
            ],
            compiler.compile().unwrap(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_compile_constants() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), },
                r#"
@const1: 2345h
@const2: 23h
ld IX, @const1
LD A, @const2
"#.to_string(),
            )],
        }, 1024);

        compare_memory(
            vec![
                0b11011101,
                0b00100001,
                0b01000101,
                0b00100011,
                0b00111110,
                0b00100011,
            ],
            compiler.compile().unwrap(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_compile_rst() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), },
                r#"
@const1: 18h
RST @const1
RST 30h
RST 0h
"#.to_string(),
            )],
        }, 1024);

        compare_memory(
            vec![
                0b11011111,
                0b11110111,
                0b11000111,
            ],
            compiler.compile().unwrap(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_compile_djnz() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), }, r#"
@Inbuf:  A000h
@Outbuf: A100h
        LD   C,    80h        ;Set up counter
        LD   HL,   @Inbuf     ;Set up pointers
        LD   DE,   @Outbuf
.LOOP:  LD   A,    (HL)       ;Get next byte from
                              ;input buffer
        LD   (DE), A          ;Store in output buffer
        CP   0Dh              ;Is it a CR?
        JR   Z,    &DONE      ;Yes finished
        INC  HL               ;Increment pointers
        INC  DE
        DJNZ &LOOP             ;Loop back if 80
                              ;bytes have not
                              ;been moved
.DONE:
"#.to_string(),
            )],
        }, 1024);

        compare_memory(
            vec![
                0b00001110, // LD C, 80h
                0x80,
                0b00100001, // LD HL, @Inbuf
                0x00,
                0xA0,
                0b00010001, // LD DE, @Outbuf
                0x00,
                0xA1,
                0b01111110, // LD A, (HL)    .LOOP:
                0x12,       // LD (DE), A
                0xFE,       // CP 0Dh
                0x0D,
                0x28,       // JR Z, &DONE
                4,
                0b00100011, // INC HL
                0b00010011, // INC DE
                0x10,       // DJNZ &LOOP
                -10i8 as u8,
            ],
            compiler.compile().unwrap(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_compile_16bit_multiplication() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), }, r#"
.Mult16:
            LD   B,   10h           ; number of bits init
            LD   C,   D             ; move multiplier
            LD   A,   E             ;
            EX   DE,  HL            ; move multiplicand
            LD   HL,  0h            ; clear partial result
.mloop:     SRL  C                  ; shift multiplier right
            RRA                     ; least-significat bit is in carry
            JR   NC,  &noadd        ; skip add if no carry
            ADD  HL,  DE            ; else add multiplicand to partialresult
.noadd:     EX   DE,  HL            ; shift multiplicand left
            ADD  HL,  HL            ; by multiplying it by two
            EX   DE,  HL            ;
            DJNZ &mloop             ; repeat until no more bits
            RET                     ;
"#.to_string(),
            )],
        }, 1024);

        compare_memory(
            vec![
                0x06, 0x10,
                0x4A,
                0x7B,
                0xEB,
                0x21, 0x00, 0x00,
                0xCB, 0x39,
                0x1F,
                0x30, 0x01,
                0x19,
                0xEB,
                0x29,
                0xEB,
                0x10, 0xF5,
                0xC9,
            ],
            compiler.compile().unwrap(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_compile_bubble_sort() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), }, r#"
.BSort:
@flag:  0h
            LD   &data, HL          ; save data address
.loop:      RES  @flag, H           ; initialize exchange flag
            LD   B,     C           ; initialize length counter
            DEC  B                  ; adjust for testing
            LD   IX,    &data       ; initialize array pointer
.next:      LD   A,     (IX)        ; first element in comparison
            LD   D,     A           ; temporary storage for element
            LD   E,     (IX+1h)     ; second element in comparison
            SUB  E                  ; comparison first to second
            JR   NC,    &noex       ; if first > second, no jump
            LD   (IX),  E           ; exchange array elements
            LD   (IX+1h), D
            SET  @flag, H           ; record exchange occurred
.noex:      INC  IX                 ; point to next data element
            DJNZ &next              ; count number of comparisons, repeat if more data pairs
            BIT  @flag, H           ; etermine if exchange occurred
            JR   NZ, &loop          ; continue if data unsorted
            RET

.data:      0000h
.test:      01h
"#.to_string(),
            )],
        }, 1024);

        compare_memory(
            vec![
                0x22, 0x26, 0x00,
                0xCB, 0x84,
                0x41,
                0x05,
                0xDD, 0x2A, 0x26, 0x00,
                0xDD, 0x7E, 0x00,
                0x57,
                0xDD, 0x5E, 0x01,
                0x93,
                0x30, 0x08,
                0xDD, 0x73, 0x00,
                0xDD, 0x72, 0x01,
                0xCB, 0xC4,
                0xDD, 0x23,
                0x10, 0xEA,
                0xCB, 0x44,
                0x20, 0xDE,
                0xC9,
                0x00, 0x00,
                0x01,
            ],
            compiler.compile().unwrap(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_macros_1() {
        let compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), },
                r#"
#defm nested arg1, arg2
ld arg1, arg2
#endm

#defm macro123 arg1, arg2, arg3, arg4
#exec nested arg1, arg4    ; ld A, (IX + 5h)
#exec nested arg3, arg2    ; ld (hl), C
#endm

#exec macro123 A, C, (hl), (IX + 5h)
"#.to_string(),
            )],
        }, 1024);

        compare_memory(
            vec![
                0xDD,
                0b01111110,
                0b00000101,
                0b01110001,
            ],
            compiler.compile().unwrap(),
        );
    }

    fn compare_memory(expected: Vec<u8>, actual: Vec<u8>) {
        if actual.len() < expected.len() {
            eprintln!("expected: {:?}, actual {:?}", expected.len(), actual.len());
            panic!();
        }
        eprintln!(" idx  | expe | actu");
        for (ia, a) in actual.iter().enumerate() {
            match expected.get(ia) {
                None => {
                    if *a != 0 {
                        eprintln!(" {:4X?} | {:#04X?} | {:#04X?}", ia, 0, a);
                        panic!()
                    }
                }
                Some(e) => {
                    eprintln!(" {:4X?} | {:#04X?} | {:#04X?}", ia, e, a);
                    if *a != *e {
                        panic!()
                    }
                }
            }
        }
    }
}
