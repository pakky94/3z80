use crate::parser::errors::{ParseError, UnexpectedToken};
use crate::parser::token::Token;
use crate::parser::tokenizer::Tokenizer;

mod errors;
mod token;
mod tokenizer;

#[derive(Debug)]
struct Parser<'a> {
    source: &'a str,
    tokenizer: Tokenizer<'a>,
    pos: usize,
    items: Vec<ParseItem>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    opcode: String,
    arg0: Argument,
    arg1: Argument,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Argument {
    None,
    ShortReg(ShortReg),
    WideReg(WideReg),
    Short(u8),
    Wide(u16),
    DirectAddress(u16),
    RegAddress(WideReg),
    RegOffsetAddress(WideReg, u8),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ShortReg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum WideReg {
    BC,
    DE,
    HL,
    SP,
    IX,
    IY,
}

#[derive(Debug, Eq, PartialEq)]
struct Label {
    name: String,
    target: usize,
}

#[derive(Debug)]
struct ParseResult {
    items: Vec<ParseItem>,
}

#[derive(Debug, Eq, PartialEq)]
enum ParseItem {
    Label(Label),
    Instruction(Instruction),
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
                Token::Label(_) => self.parse_label()?,
                Token::Identifier(_) => self.parse_instruction()?,
                Token::ShortValue(_) => unimplemented!("unexpected short value {:?}", self.items),
                Token::WideValue(_) => unimplemented!("unexpected wide value {:?}", self.items),
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
        if let Ok(Token::Label(l)) = self.tokenizer.next() {
            self.tokenizer.expect(Token::NewLine)?;
            Ok(ParseItem::Label(Label {
                name: l.to_string(),
                target: self.pos,
            }))
        } else {
            panic!()
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
            Token::ShortValue(v) => Ok(Argument::Short(v)),
            Token::WideValue(v) => Ok(Argument::Wide(v)),
            Token::OpenParen => {
                if let Token::Identifier(i) = self.tokenizer.next()? {
                    if let Token::Plus = self.tokenizer.peek()? {
                        self.tokenizer.next()?;
                        if let Token::ShortValue(offset) = self.tokenizer.next()? {
                            if let ParseRegisterResult::WideReg(wr) = self.parse_register(&i) {
                                self.tokenizer.expect(Token::CloseParen)?;
                                Ok(Argument::RegOffsetAddress(wr, offset))
                            } else {
                                unimplemented!()
                            }
                        } else {
                            unimplemented!()
                        }
                    } else {
                        if let ParseRegisterResult::WideReg(wr) = self.parse_register(&i) {
                            self.tokenizer.expect(Token::CloseParen)?;
                            Ok(Argument::RegAddress(wr))
                        } else {
                            unimplemented!()
                        }
                    }
                } else {
                    unimplemented!()
                }
            }
            Token::Label(_) => unimplemented!(),
            Token::Identifier(i) => Ok(match self.parse_register(&i) {
                ParseRegisterResult::ShortReg(sr) => Argument::ShortReg(sr),
                ParseRegisterResult::WideReg(wr) => Argument::WideReg(wr),
                _ => unimplemented!(),
            }),
            Token::NewLine => unreachable!(),
            _ => unimplemented!(),
        }
    }

    fn parse_register(&mut self, identifier: &str) -> ParseRegisterResult {
        match identifier.to_lowercase().as_str() {
            "a" => ParseRegisterResult::ShortReg(ShortReg::A),
            "b" => ParseRegisterResult::ShortReg(ShortReg::B),
            "c" => ParseRegisterResult::ShortReg(ShortReg::C),
            "d" => ParseRegisterResult::ShortReg(ShortReg::D),
            "e" => ParseRegisterResult::ShortReg(ShortReg::E),
            "h" => ParseRegisterResult::ShortReg(ShortReg::H),
            "l" => ParseRegisterResult::ShortReg(ShortReg::L),
            "bc" => ParseRegisterResult::WideReg(WideReg::BC),
            "de" => ParseRegisterResult::WideReg(WideReg::DE),
            "hl" => ParseRegisterResult::WideReg(WideReg::HL),
            "sp" => ParseRegisterResult::WideReg(WideReg::SP),
            "ix" => ParseRegisterResult::WideReg(WideReg::IX),
            "iy" => ParseRegisterResult::WideReg(WideReg::IY),
            _ => unimplemented!("unimplemented identifier handler: {:?}", identifier),
        }
    }
}

enum ParseRegisterResult {
    WideReg(WideReg),
    ShortReg(ShortReg),
    Error(ParseError),
}

pub fn test() {
    println!("parser test module");
}

#[cfg(test)]
mod tests {
    use crate::parser::{Argument, Instruction, ParseItem, Parser, ShortReg, WideReg};

    #[test]
    fn test_parse1() {
        let mut parser = Parser::new(
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
            assert_eq!(inst1.arg1, Argument::Short(16));
        } else {
            panic!()
        }

        if let ParseItem::Instruction(inst2) = result.items.get(2).unwrap() {
            assert_eq!(inst2.opcode, "add");
            assert_eq!(inst2.arg0, Argument::ShortReg(ShortReg::B));
            assert_eq!(inst2.arg1, Argument::Short(8));
        } else {
            panic!()
        }
    }

    #[test]
    fn test_parse_address_reg_argument() {
        let mut parser = Parser::new("ld A, (IX)");
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
        let mut parser = Parser::new("ld A, (IX + 15h)");
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ld".to_string(),
                arg0: Argument::ShortReg(ShortReg::A),
                arg1: Argument::RegOffsetAddress(WideReg::IX, 21),
            }),
            *parser.parse().unwrap().items.get(0).unwrap()
        );
    }
}
