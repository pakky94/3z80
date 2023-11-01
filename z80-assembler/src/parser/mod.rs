use crate::domain::register::{parse_register, ParsedRegister};
use crate::domain::*;
use crate::parser::errors::{ParseError, UnexpectedToken};
use crate::parser::token::Token;
use crate::parser::tokenizer::Tokenizer;

mod errors;
mod token;
mod tokenizer;

#[derive(Debug)]
pub struct Parser<'a> {
    source: &'a str,
    tokenizer: Tokenizer<'a>,
    pos: usize,
    items: Vec<ParseItem>,
}

#[derive(Debug)]
pub struct ParseResult {
    items: Vec<ParseItem>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser {
            source,
            tokenizer: Tokenizer::new(source),
            items: Vec::new(),
            pos: 0,
        }
    }

    pub fn parse(mut self) -> Result<ParseResult, ParseError> {
        loop {
            let t = self.tokenizer.peek()?;

            let r = match t {
                Token::Dot => self.parse_label()?,
                Token::Identifier(_) => self.parse_instruction()?,
                Token::Value(_) => unimplemented!("unexpected wide value {:?}", self.items),
                Token::Comma => unimplemented!("unexpected comma {:?}", self.items),
                Token::NewLine => {
                    self.tokenizer.next()?;
                    continue;
                }
                Token::EOF => break,
                _ => unimplemented!("unexpected token {:?} - {:?}", t, self.items),
            };
            self.items.push(r)
        }

        Ok(ParseResult { items: self.items })
    }

    fn parse_label(&mut self) -> Result<ParseItem, ParseError> {
        self.tokenizer.next()?;
        if let Ok(Token::Identifier(l)) = self.tokenizer.next() {
            self.tokenizer.expect(Token::Colon)?;
            Ok(ParseItem::Label(Label {
                name: l.to_string(),
                target: self.pos,
            }))
        } else {
            unimplemented!("expected identifier token")
        }
    }

    fn parse_instruction(&mut self) -> Result<ParseItem, ParseError> {
        let code = if let Ok(Token::Identifier(s)) = self.tokenizer.next() {
            s
        } else {
            panic!()
        };
        let mut inst = Instruction {
            opcode: code.to_lowercase(),
            arg0: Argument::None,
            arg1: Argument::None,
        };

        if let Ok(Token::NewLine) = self.tokenizer.peek() {
            return Ok(ParseItem::Instruction(inst));
        }

        inst.arg0 = self.parse_argument()?;

        if self.tokenizer.peek()? != Token::Comma {
            self.tokenizer.expect(Token::NewLine)?;
            return Ok(ParseItem::Instruction(inst));
        }

        self.tokenizer.next()?; // Token::Comma

        inst.arg1 = self.parse_argument()?;

        let t = self.tokenizer.peek()?;
        if t != Token::NewLine && t != Token::EOF {
            return Err(ParseError::UnexpectedToken(UnexpectedToken {
                expected: Token::NewLine,
                actual: t,
                line: 0,
                char: 0,
            }));
        }

        Ok(ParseItem::Instruction(inst))
    }

    fn parse_argument(&mut self) -> Result<Argument, ParseError> {
        match self.tokenizer.next()? {
            Token::Value(v) => Ok(Argument::Value(v)),
            Token::OpenParen => match self.tokenizer.next()? {
                Token::Identifier(i) => {
                    if let Token::Plus = self.tokenizer.peek()? {
                        self.tokenizer.next()?;
                        if let Token::Value(offset) = self.tokenizer.next()? {
                            if let ParsedRegister::WideReg(wr) = parse_register(&i) {
                                self.tokenizer.expect(Token::CloseParen)?;
                                Ok(Argument::RegOffsetAddress(wr, offset))
                            } else {
                                unimplemented!()
                            }
                        } else {
                            unimplemented!()
                        }
                    } else {
                        if let ParsedRegister::WideReg(wr) = parse_register(&i) {
                            self.tokenizer.expect(Token::CloseParen)?;
                            Ok(Argument::RegAddress(wr))
                        } else {
                            unimplemented!()
                        }
                    }
                }
                Token::Value(val) => {
                    self.tokenizer.expect(Token::CloseParen)?;
                    Ok(Argument::DirectAddress(val))
                }
                _ => unimplemented!()
            },
            Token::Identifier(i) => Ok(match parse_register(&i) {
                ParsedRegister::ShortReg(sr) => Argument::ShortReg(sr),
                ParsedRegister::WideReg(wr) => Argument::WideReg(wr),
                _ => unimplemented!(),
            }),
            Token::NewLine => unreachable!(),
            _ => unimplemented!(),
        }
    }
}

pub fn test() {
    println!("parser test module");
}

#[cfg(test)]
mod tests {
    use crate::domain::enums::{ShortReg, WideReg};
    use crate::domain::Label;
    use crate::parser::{Argument, Instruction, ParseItem, Parser};

    #[test]
    fn test_parse1() {
        let parser = Parser::new(
            r#"
.label1:
ld A, 10h
add b, 8h"#,
        );
        let result = parser.parse().unwrap();

        if let ParseItem::Label(label) = result.items.get(0).unwrap() {
            assert_eq!(label.name, "label1");
        } else {
            panic!()
        }

        if let ParseItem::Instruction(inst1) = result.items.get(1).unwrap() {
            assert_eq!(inst1.opcode, "ld");
            assert_eq!(inst1.arg0, Argument::ShortReg(ShortReg::A));
            assert_eq!(inst1.arg1, Argument::Value(16));
        } else {
            panic!()
        }

        if let ParseItem::Instruction(inst2) = result.items.get(2).unwrap() {
            assert_eq!(inst2.opcode, "add");
            assert_eq!(inst2.arg0, Argument::ShortReg(ShortReg::B));
            assert_eq!(inst2.arg1, Argument::Value(8));
        } else {
            panic!()
        }
    }

    #[test]
    fn test_parse_address_reg_argument() {
        let parser = Parser::new("ld A, (IX)");
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ld".to_string(),
                arg0: Argument::ShortReg(ShortReg::A),
                arg1: Argument::RegAddress(WideReg::IX),
            }),
            *parser.parse().unwrap().items.get(0).unwrap()
        );
    }

    #[test]
    fn test_parse_address_reg_argument_with_offset() {
        let parser = Parser::new("ld A, (IX + 15h)");
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ld".to_string(),
                arg0: Argument::ShortReg(ShortReg::A),
                arg1: Argument::RegOffsetAddress(WideReg::IX, 21),
            }),
            *parser.parse().unwrap().items.get(0).unwrap()
        );
    }

    #[test]
    fn test_parse_address() {
        let parser = Parser::new("ld BC, (A2C5h)");
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ld".to_string(),
                arg0: Argument::WideReg(WideReg::BC),
                arg1: Argument::DirectAddress(41669),
            }),
            *parser.parse().unwrap().items.get(0).unwrap()
        );
    }

    #[test]
    fn test_parse_label_instr_same_line() {
        let parser = Parser::new(".my_label: ADD A, A9h");
        let res = parser.parse().unwrap();
        assert_eq!(
            ParseItem::Label(Label {
                name: "my_label".to_string(),
                target: 0,
            }),
            *res.items.get(0).unwrap()
        );
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "add".to_string(),
                arg0: Argument::ShortReg(ShortReg::A),
                arg1: Argument::Value(169),
            }),
            *res.items.get(1).unwrap()
        );
    }
}
