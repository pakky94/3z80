use crate::compiler::instructions::{
    compile_instruction, label_not_found, CompileError, CompileErrorType, Placeholder,
    PlaceholderType,
};
pub use crate::compiler::source_provider::{InMemorySourceProvider, SourceHeader, SourceProvider};
use crate::domain::{Argument, Instruction, ParseItem};
use crate::parser::Parser;
use std::collections::HashMap;

mod instructions;
mod source_provider;

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
        }
    }

    pub fn compile(mut self) -> Result<Vec<u8>, CompileError> {
        for file in self.source_provider.file_list() {
            self.constants.clear();
            let source = self.source_provider.source(&file.filename);
            let res = (Parser::new(&source)).parse()?;

            for i in res.items {
                self.process_item(i)?;
            }
        }

        for ph in self.placeholders.into_iter() {
            let addr = self
                .label_map
                .get(ph.label.as_str())
                .ok_or(label_not_found(&ph))?;

            match (ph.ph_type, ph.size) {
                (PlaceholderType::Value, 1) => {
                    self.out[ph.idx] = self.out[*addr];
                }
                (PlaceholderType::Value, 2) => {
                    self.out[ph.idx] = self.out[*addr];
                    self.out[ph.idx + 1] = self.out[*addr + 1];
                }
                (PlaceholderType::Address, 2) => {
                    self.out[ph.idx] = (*addr % 256) as u8;
                    self.out[ph.idx + 1] = (*addr / 256) as u8
                }
                (_, _) => panic!("Invalid placeholder type"),
            }
        }

        Ok(self.out)
    }

    fn process_item(&mut self, item: ParseItem) -> Result<(), CompileError> {
        Ok(match item {
            ParseItem::Label(l) => {
                self.label_map.insert(l.name, self.idx);
            }
            ParseItem::Instruction(inst) => {
                let (inst, p0, p1) = self.extract_placeholders(inst);
                let inst = self.replace_constants(inst)?;
                let data =
                    compile_instruction(&inst, p0, p1, &mut self.placeholders).map_err(|err| {
                        CompileError {
                            error: err.error,
                            instr: Some(inst.clone()),
                        }
                    })?;
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
                    size: 0,
                    ph_type: PlaceholderType::Address,
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
                    size: 0,
                    ph_type: PlaceholderType::Value,
                    line,
                });
                (
                    Some(Argument::Value(0)),
                    isize::try_from(self.placeholders.len()).unwrap() - 1,
                )
            }
            // Argument::LabelValue(s) => {
            //     Some(Argument::Value(0))
            // }
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
.LOOP:  LID  A,    (HL)       ;Get next byte from
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
                0b11011111,
                0b11110111,
                0b11000111,
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
