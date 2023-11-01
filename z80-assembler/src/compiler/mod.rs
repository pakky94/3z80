use crate::compiler::instructions::{
    compile_instruction, CompileError, CompileErrorType, PlaceholderType,
};
use crate::compiler::source_provider::SourceProvider;
use crate::domain::ParseItem;
use crate::parser::Parser;
use std::collections::HashMap;

mod instructions;
mod source_provider;

pub struct Compiler<T>
where
    T: SourceProvider,
{
    source_provider: T,
}

impl<T> Compiler<T>
where
    T: SourceProvider,
{
    pub fn new(source_provider: T) -> Self {
        Compiler { source_provider }
    }

    pub fn compile(&mut self, capacity: usize) -> Result<Vec<u8>, CompileError> {
        let mut out = vec![0u8; capacity];
        let mut idx = 0;
        let mut label_map: HashMap<String, usize> = HashMap::new();
        let mut placeholders = vec![];

        for file in self.source_provider.file_list() {
            let source = self.source_provider.source(&file.filename);
            let res = match (Parser::new(&source)).parse() {
                Ok(r) => r,
                Err(e) => {
                    return Err(CompileError {
                        error: CompileErrorType::ParseError(e),
                        instr: None,
                    })
                }
            };

            for i in res.items {
                match i {
                    ParseItem::Label(l) => {
                        label_map.insert(l.name, idx);
                        ()
                    }
                    ParseItem::Instruction(inst) => match compile_instruction(&inst, idx) {
                        Ok(data) => {
                            for i in 0..data.len {
                                out[idx] = data.data[i as usize];
                                idx += 1;
                            }
                            if let Some(p) = data.placeholder {
                                placeholders.push(p);
                            }
                        }
                        Err(mut err) => {
                            err.instr = Some(inst);
                            return Err(err);
                        }
                    },
                    ParseItem::Data(data) => {
                        for b in data.iter() {
                            out[idx] = *b;
                            idx += 1;
                        }
                    }
                }
            }
        }

        for ph in placeholders.into_iter() {
            if let Some(addr) = label_map.get(ph.label.as_str()) {
                match ph.ph_type {
                    PlaceholderType::Value => {
                        out[ph.idx] = out[*addr];
                    }
                    PlaceholderType::Address => {
                        out[ph.idx] = (*addr % 256) as u8;
                        out[ph.idx + 1] = (*addr / 256) as u8
                    }
                }
            } else {
                unimplemented!("error")
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::source_provider::{InMemorySourceProvider, SourceHeader};
    use crate::Compiler;

    #[test]
    #[rustfmt::skip]
    fn test_compile_inst_1() {
        let mut compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), },
                r#"
ld A, C
ld b, 12h
"#.to_string(),
            )],
        });

        compare_memory(
            vec![
                0b01111001,
                0b00000110,
                0b00010010
            ],
            compiler.compile(1024).unwrap(),
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_compile_labels() {
        let mut compiler = Compiler::new(InMemorySourceProvider {
            files: vec![(
                SourceHeader { filename: "main.z80".to_string(), },
                r#"
.label1: 12h
.label2: 13h
.label3: 14h
.label4: 15h
ld b, *label3
ld c, &label4
ld a, (HL)
ld e, (IX + 5h)
ld l, (IY + a3h)
"#.to_string(),
            )],
        });

        compare_memory(
            vec![
                0b00010010, // label1
                0b00010011, // label2
                0b00010100, // label3
                0b00010101, // label4
                0b00000110, // ld b, *label3
                0b00010100,
                0b00111010, // ld c, &label4
                0b00000011,
                0b00000000,
                0b01111110, // ld a, (HL)
                0b11011101, // ld e, (IX + 5h)
                0b01011110,
                0b00000101,
                0b11111101, // ld l, (IY + a3h)
                0b01101110,
                0b10100011,
            ],
            compiler.compile(1024).unwrap(),
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
                    eprintln!(" {:4X?} | {:#04X?} | {:#04X?}", ia, 0, a);
                    if *a != 0 {
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
