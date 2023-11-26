use crate::domain::conditions::{condition_allowed, parse_condition};
use crate::domain::enums::WideReg;
use crate::domain::register::{parse_register, ParsedRegister};
use crate::domain::*;
pub use crate::parser::errors::ParseError;
use crate::parser::errors::UnexpectedToken;
use crate::parser::token::{Token, TokenValue};
use crate::parser::tokenizer::Tokenizer;

mod errors;
mod token;
mod tokenizer;

#[derive(Debug)]
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    items: Vec<ParseItem>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(source, 0),
            items: Vec::new(),
        }
    }

    pub fn parse_next(&mut self) -> Result<Option<ParseItem>, ParseError> {
        loop {
            let t = self.tokenizer.peek()?;

            let r = match t.token {
                TokenValue::Dot => self.parse_label()?,
                TokenValue::Identifier(_) => self.parse_instruction()?,
                TokenValue::Value(_, _) => self.parse_data()?,
                TokenValue::NewLine => {
                    self.tokenizer.next()?;
                    continue;
                }
                TokenValue::At => self.parse_constant()?,
                TokenValue::Directive(_) => self.parse_directive()?,
                TokenValue::EOF => break,
                _ => unimplemented!("unexpected token {:?} - {:?}", t, self.items),
            };
            return Ok(Some(r));
        }

        Ok(None)
    }

    fn parse_label(&mut self) -> Result<ParseItem, ParseError> {
        self.tokenizer.next()?;
        if let Token {
            token: TokenValue::Identifier(l),
            line,
            file_id,
        } = self.tokenizer.next()?
        {
            self.tokenizer.expect(TokenValue::Colon)?;
            Ok(ParseItem::Label(Label {
                name: l.to_string(),
                line,
                file_id,
            }))
        } else {
            unimplemented!("expected identifier token")
        }
    }

    fn parse_instruction(&mut self) -> Result<ParseItem, ParseError> {
        if let Token {
            token: TokenValue::Identifier(code),
            line,
            file_id,
        } = self.tokenizer.next()?
        {
            let mut inst = Instruction {
                opcode: code.to_lowercase(),
                arg0: Argument::None,
                arg1: Argument::None,
                line,
                file_id,
            };

            if let TokenValue::NewLine = self.tokenizer.peek()?.token {
                return Ok(ParseItem::Instruction(inst));
            }

            inst.arg0 = self.parse_argument(&inst.opcode)?;

            if self.tokenizer.peek()?.token != TokenValue::Comma {
                self.tokenizer.expect_peek(TokenValue::NewLine)?;
                return Ok(ParseItem::Instruction(inst));
            }

            self.tokenizer.next()?; // Token::Comma

            inst.arg1 = self.parse_argument(&inst.opcode)?;

            let t = self.tokenizer.peek()?;
            if t.token != TokenValue::NewLine && t.token != TokenValue::EOF {
                return Err(ParseError::UnexpectedToken(UnexpectedToken {
                    expected: TokenValue::NewLine,
                    actual: t.token,
                    line: t.line,
                    file_id: t.file_id,
                    char: 0,
                }));
            }

            Ok(ParseItem::Instruction(inst))
        } else {
            panic!()
        }
    }

    fn parse_argument(&mut self, code: &str) -> Result<Argument, ParseError> {
        match self.tokenizer.next()?.token {
            TokenValue::Value(v, _) => Ok(Argument::Value(v)), // TODO: pass length?
            TokenValue::OpenParen => self.parse_address_arg(),
            TokenValue::Identifier(i) => {
                if condition_allowed(code) {
                    if let Some(c) = parse_condition(&i) {
                        return Ok(Argument::Condition(c));
                    }
                }
                Ok(match parse_register(&i) {
                    ParsedRegister::ShortReg(sr) => Argument::ShortReg(sr),
                    ParsedRegister::WideReg(wr) => Argument::WideReg(wr),
                    _ => unimplemented!(),
                })
            }
            TokenValue::Amp => {
                if let TokenValue::Identifier(i) = self.tokenizer.next()?.token {
                    Ok(Argument::LabelAddress(i))
                } else {
                    unimplemented!("expected identifier")
                }
            }
            TokenValue::Asterisk => {
                if let TokenValue::Identifier(i) = self.tokenizer.next()?.token {
                    Ok(Argument::LabelValue(i))
                } else {
                    unimplemented!("expected identifier")
                }
            }
            TokenValue::At => {
                if let TokenValue::Identifier(i) = self.tokenizer.next()?.token {
                    Ok(Argument::Constant(i))
                } else {
                    unimplemented!("expected identifier")
                }
            }
            t => unimplemented!("unhandled token {:?}", t),
        }
    }

    fn parse_address_arg(&mut self) -> Result<Argument, ParseError> {
        match self.tokenizer.next()?.token {
            TokenValue::Identifier(i) => {
                if let TokenValue::Plus = self.tokenizer.peek()?.token {
                    self.tokenizer.next()?;
                    if let TokenValue::Value(offset, _) = self.tokenizer.next()?.token {
                        // TODO: validate length here???
                        if let ParsedRegister::WideReg(wr) = parse_register(&i) {
                            self.tokenizer.expect(TokenValue::CloseParen)?;
                            Ok(Argument::RegOffsetAddress(wr, offset))
                        } else {
                            unimplemented!()
                        }
                    } else {
                        unimplemented!()
                    }
                } else {
                    if let ParsedRegister::WideReg(wr) = parse_register(&i) {
                        self.tokenizer.expect(TokenValue::CloseParen)?;
                        if wr == WideReg::IX || wr == WideReg::IY {
                            Ok(Argument::RegOffsetAddress(wr, 0))
                        } else {
                            Ok(Argument::WideRegAddress(wr))
                        }
                    } else {
                        unimplemented!()
                    }
                }
            }
            TokenValue::Value(val, _) => {
                self.tokenizer.expect(TokenValue::CloseParen)?;
                Ok(Argument::DirectAddress(val))
            }
            t => unimplemented!("unhandled token {:?}", t),
        }
    }

    fn parse_data(&mut self) -> Result<ParseItem, ParseError> {
        if let TokenValue::Value(val, size) = self.tokenizer.next()?.token {
            Ok(match size {
                1 => ParseItem::Data(vec![val as u8]),
                2 => ParseItem::Data(vec![(val % 256) as u8, (val / 256) as u8]),
                _ => panic!("unexpected Value size {:?}", size),
            })
        } else {
            unreachable!()
        }
    }
    fn parse_constant(&mut self) -> Result<ParseItem, ParseError> {
        self.tokenizer.next()?;
        if let TokenValue::Identifier(l) = self.tokenizer.next()?.token {
            self.tokenizer.expect(TokenValue::Colon)?;
            if let TokenValue::Value(val, _) = self.tokenizer.next()?.token {
                Ok(ParseItem::Constant(Constant {
                    name: l,
                    value: val,
                }))
            } else {
                unimplemented!("expected value token")
            }
        } else {
            unimplemented!("expected identifier token")
        }
    }
    fn parse_directive(&mut self) -> Result<ParseItem, ParseError> {
        if let TokenValue::Directive(s) = self.tokenizer.next()?.token {
            Ok(ParseItem::Directive(s))
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::enums::{Condition, ShortReg, WideReg};
    use crate::domain::{Constant, Label};
    use crate::parser::{Argument, Instruction, ParseItem, Parser};

    #[test]
    fn test_parse1() {
        let parser = Parser::new(
            r#"
.label1:
ld A, 10h
add b, 8h"#,
        );
        let res = parse_all(parser);

        if let ParseItem::Label(label) = res.get(0).unwrap() {
            assert_eq!(label.name, "label1");
        } else {
            panic!()
        }

        if let ParseItem::Instruction(inst1) = res.get(1).unwrap() {
            assert_eq!(inst1.opcode, "ld");
            assert_eq!(inst1.arg0, Argument::ShortReg(ShortReg::A));
            assert_eq!(inst1.arg1, Argument::Value(16));
        } else {
            panic!()
        }

        if let ParseItem::Instruction(inst2) = res.get(2).unwrap() {
            assert_eq!(inst2.opcode, "add");
            assert_eq!(inst2.arg0, Argument::ShortReg(ShortReg::B));
            assert_eq!(inst2.arg1, Argument::Value(8));
        } else {
            panic!()
        }
    }

    #[test]
    fn test_parse_address_reg_argument() {
        let parser = Parser::new("ld A, (HL)");
        let res = parse_all(parser);
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ld".to_string(),
                arg0: Argument::ShortReg(ShortReg::A),
                arg1: Argument::WideRegAddress(WideReg::HL),
                line: 1,
                file_id: 0,
            }),
            *res.get(0).unwrap()
        );
    }

    #[test]
    fn test_parse_address_reg_argument_with_offset() {
        let parser = Parser::new("ld A, (IX + 15h)");
        let res = parse_all(parser);
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ld".to_string(),
                arg0: Argument::ShortReg(ShortReg::A),
                arg1: Argument::RegOffsetAddress(WideReg::IX, 21),
                line: 1,
                file_id: 0,
            }),
            *res.get(0).unwrap()
        );
    }

    #[test]
    fn test_parse_address() {
        let parser = Parser::new("ld BC, (A2C5h)");
        let res = parse_all(parser);
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ld".to_string(),
                arg0: Argument::WideReg(WideReg::BC),
                arg1: Argument::DirectAddress(41669),
                line: 1,
                file_id: 0,
            }),
            *res.get(0).unwrap()
        );
    }

    #[test]
    fn test_parse_label_instr_same_line() {
        let parser = Parser::new(".my_label: ADD A, A9h");
        let res = parse_all(parser);
        assert_eq!(
            ParseItem::Label(Label {
                name: "my_label".to_string(),
                line: 1,
                file_id: 0,
            }),
            *res.get(0).unwrap()
        );
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "add".to_string(),
                arg0: Argument::ShortReg(ShortReg::A),
                arg1: Argument::Value(169),
                line: 1,
                file_id: 0,
            }),
            *res.get(1).unwrap()
        );
    }

    #[test]
    fn test_parse_label_argument() {
        let parser = Parser::new(
            r#"
CALL &label1
LD BC, *label2
"#,
        );
        let res = parse_all(parser);
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "call".to_string(),
                arg0: Argument::LabelAddress("label1".to_string()),
                arg1: Argument::None,
                line: 2,
                file_id: 0,
            }),
            *res.get(0).unwrap()
        );
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ld".to_string(),
                arg0: Argument::WideReg(WideReg::BC),
                arg1: Argument::LabelValue("label2".to_string()),
                line: 3,
                file_id: 0,
            }),
            *res.get(1).unwrap()
        );
    }

    #[test]
    fn test_parse_condition() {
        let parser = Parser::new(
            r#"
CALL C, *label1
JP PO, 1234h
JR NZ, a7h
RET M
"#,
        );
        let res = parse_all(parser);
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "call".to_string(),
                arg0: Argument::Condition(Condition::C),
                arg1: Argument::LabelValue("label1".to_string()),
                line: 2,
                file_id: 0,
            }),
            *res.get(0).unwrap()
        );
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "jp".to_string(),
                arg0: Argument::Condition(Condition::PO),
                arg1: Argument::Value(4660),
                line: 3,
                file_id: 0,
            }),
            *res.get(1).unwrap()
        );
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "jr".to_string(),
                arg0: Argument::Condition(Condition::NZ),
                arg1: Argument::Value(167),
                line: 4,
                file_id: 0,
            }),
            *res.get(2).unwrap()
        );
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "ret".to_string(),
                arg0: Argument::Condition(Condition::M),
                arg1: Argument::None,
                line: 5,
                file_id: 0,
            }),
            *res.get(3).unwrap()
        );
    }

    #[test]
    fn test_parse_data() {
        let parser = Parser::new(
            r#"
.data1: 15h
.data2: aa15h"#,
        );
        let res = parse_all(parser);
        assert_eq!(ParseItem::Data(vec![21u8]), *res.get(1).unwrap());
        assert_eq!(ParseItem::Data(vec![21u8, 170u8]), *res.get(3).unwrap());
    }

    #[test]
    fn test_parse_constants() {
        let parser = Parser::new(
            r#"
@const1: 15h
add a, @const1"#,
        );
        let res = parse_all(parser);
        assert_eq!(
            ParseItem::Constant(Constant {
                name: "const1".to_string(),
                value: 21,
            }),
            *res.get(0).unwrap()
        );
        assert_eq!(
            ParseItem::Instruction(Instruction {
                opcode: "add".to_string(),
                arg0: Argument::ShortReg(ShortReg::A),
                arg1: Argument::Constant("const1".to_string()),
                line: 3,
                file_id: 0,
            }),
            *res.get(1).unwrap()
        );
    }

    #[test]
    fn test_parse_directives() {
        let parser = Parser::new(
            r#"
#include "test.z80"
#test dir 123
"#,
        );
        let res = parse_all(parser);
        assert_eq!(
            ParseItem::Directive(r#"#include "test.z80""#.to_string()),
            *res.get(0).unwrap()
        );
        assert_eq!(
            ParseItem::Directive(r#"#test dir 123"#.to_string()),
            *res.get(1).unwrap()
        );
    }

    fn parse_all(mut parser: Parser) -> Vec<ParseItem> {
        let mut res = vec![];
        loop {
            if let Some(pi) = parser.parse_next().unwrap() {
                res.push(pi);
            } else {
                break;
            }
        }
        res
    }
}
