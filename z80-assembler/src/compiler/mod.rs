use crate::compiler::instructions::{compile_instruction, CompileResult};
use crate::compiler::source_provider::SourceProvider;
use crate::domain::ParseItem;
use crate::parser::{ParseError, Parser};
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

    pub fn compile(&mut self, capacity: usize) -> Result<Vec<u8>, ParseError> {
        let mut out = vec![0u8; capacity];
        let mut idx = 0;
        let mut label_map: HashMap<String, usize> = HashMap::new();
        let mut placeholders = vec![];

        for file in self.source_provider.file_list() {
            let source = self.source_provider.source(&file.filename);
            let res = match (Parser::new(&source)).parse() {
                Ok(r) => r,
                Err(e) => return Err(e),
            };

            for i in res.items {
                match i {
                    ParseItem::Label(l) => {
                        label_map.insert(l.name, idx);
                        ()
                    }
                    ParseItem::Instruction(inst) => {
                        if let CompileResult::Data(data) = compile_instruction(inst, idx) {
                            for i in 0..data.len {
                                out[idx] = data.data[i as usize];
                                idx += 1;
                            }
                            if let Some(p) = data.placeholder {
                                placeholders.push(p);
                            }
                        } else {
                            unimplemented!()
                        }
                        ()
                    }
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
                out[ph.idx] = out[*addr];
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
"#.to_string(),
            )],
        });

        compare_memory(
            vec![
                0b00010010,
                0b00010011,
                0b00010100,
                0b00010101,
                0b00000110,
                0b00010100,
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
                    if *a != 0 {
                        eprintln!("expected: {:?}, actual {:?}", expected, actual);
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
