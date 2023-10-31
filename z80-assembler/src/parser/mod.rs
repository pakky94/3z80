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

#[derive(Debug)]
struct Instruction {
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
    Address(u16),
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

#[derive(Debug)]
struct Label {
    name: String,
    target: usize,
}

#[derive(Debug)]
struct ParseResult {
    items: Vec<ParseItem>,
}

#[derive(Debug)]
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
                Token::Address(a) => unimplemented!("unexpected address {:?} - {:?}", a, self.items),
                Token::ShortValue(_) => unimplemented!("unexpected short value {:?}", self.items),
                Token::WideValue(_) => unimplemented!("unexpected wide value {:?}", self.items),
                Token::Comma => unimplemented!("unexpected comma {:?}", self.items),
                Token::NewLine => {
                    self.tokenizer.next()?;
                    continue;
                }
                Token::EOF => break,
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
        Ok(match self.tokenizer.next()? {
            Token::ShortValue(v) => Argument::Short(v),
            Token::WideValue(v) => Argument::Wide(v),
            Token::Label(_) => unimplemented!(),
            Token::Identifier(i) => self.parse_argument_identifier(i.to_lowercase().as_str())?,
            Token::Address(a) => Argument::Address(a),
            Token::Comma => unimplemented!(),
            Token::NewLine => unreachable!(),
            Token::EOF => unimplemented!(),
        })
    }

    fn parse_argument_identifier(&mut self, identifier: &str) -> Result<Argument, ParseError> {
        Ok(match identifier {
            "a" => Argument::ShortReg(ShortReg::A),
            "b" => Argument::ShortReg(ShortReg::B),
            "c" => Argument::ShortReg(ShortReg::C),
            "d" => Argument::ShortReg(ShortReg::D),
            "e" => Argument::ShortReg(ShortReg::E),
            "h" => Argument::ShortReg(ShortReg::H),
            "l" => Argument::ShortReg(ShortReg::L),
            "bc" => Argument::WideReg(WideReg::BC),
            "de" => Argument::WideReg(WideReg::DE),
            "hl" => Argument::WideReg(WideReg::HL),
            "sp" => Argument::WideReg(WideReg::SP),
            "ix" => Argument::WideReg(WideReg::IX),
            "iy" => Argument::WideReg(WideReg::IY),
            _ => unimplemented!("unimplemented identifier handler: {:?}", identifier)
        })
    }
}

pub fn test() {
    println!("parser test module");
}

#[cfg(test)]
mod tests {
    use crate::parser::ParseItem::{Instruction, Label};
    use crate::parser::{Argument, Parser, ShortReg};

    #[test]
    fn test_parse1() {
        let mut parser = Parser::new(
            r#"
.label1:
ld A, 10h
add b, 8h"#,
        );
        let result = parser.parse().unwrap();

        if let Label(label) = result.items.get(0).unwrap() {
            assert_eq!(label.name, "label1");
        } else {
            panic!()
        }

        if let Instruction(inst1) = result.items.get(1).unwrap() {
            assert_eq!(inst1.opcode, "ld");
            assert_eq!(inst1.arg0, Argument::ShortReg(ShortReg::A));
            assert_eq!(inst1.arg1, Argument::Short(16));
        } else {
            panic!()
        }

        if let Instruction(inst2) = result.items.get(2).unwrap() {
            assert_eq!(inst2.opcode, "add");
            assert_eq!(inst2.arg0, Argument::ShortReg(ShortReg::B));
            assert_eq!(inst2.arg1, Argument::Short(8));
        } else {
            panic!()
        }
    }
}
