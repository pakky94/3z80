use crate::parser::errors::ParseError;
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
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum WideReg {
    BC,
    DE,
    HL,
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
                Token::Identifier(_) => unimplemented!(),
                Token::Address(_) => unimplemented!(),
                Token::ShortValue(_) => unimplemented!(),
                Token::WideValue(_) => unimplemented!(),
                Token::Comma => unimplemented!(),
                Token::NewLine => unimplemented!(),
                Token::EOF => break,
            };
            self.items.push(r)
        }

        Ok(ParseResult { items: self.items })
    }

    fn parse_label(&mut self) -> Result<ParseItem, ParseError> {
        if let Ok(Token::Label(l)) = self.tokenizer.next() {
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

        inst.arg0 = match self.tokenizer.next()? {
            Token::ShortValue(v) => Argument::Short(v),
            Token::WideValue(v) => Argument::Wide(v),
            Token::Label(_) => unimplemented!(),
            Token::Identifier(_) => unimplemented!(),
            Token::Address(a) => Argument::Address(a),
            Token::Comma => unimplemented!(),
            Token::NewLine => unreachable!(),
            Token::EOF => unimplemented!(),
        };

        Ok(ParseItem::Instruction(inst))
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
add A, 8h"#,
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
            assert_eq!(inst2.arg0, Argument::ShortReg(ShortReg::A));
            assert_eq!(inst2.arg1, Argument::Short(8));
        } else {
            panic!()
        }
    }
}
