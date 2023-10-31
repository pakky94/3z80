use crate::parser::errors::{ParseError, UnexpectedToken};
use crate::parser::token::Token;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    curr_line: usize,
    head: Option<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source,
            chars: source.char_indices().peekable(),
            curr_line: 1,
            head: None,
        }
    }

    pub fn peek(&mut self) -> Result<Token, ParseError> {
        if let Some(t) = &self.head {
            return Ok((*t).clone());
        } else {
            let t = self.next()?;
            self.head = Some(t.clone());
            Ok(t)
        }
    }

    pub fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let actual = self.next()?;
        if actual == expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(UnexpectedToken {
                expected,
                actual,
                line: self.curr_line,
                char: 0,
            }))
        }
    }

    pub fn next(&mut self) -> Result<Token, ParseError> {
        if let Some(t) = &self.head {
            let temp = (*t).clone();
            self.head = None;
            return Ok(temp);
        }

        loop {
            if let Some((_, c)) = self.chars.peek() {
                if *c == '\n' {
                    self.curr_line += 1;
                    self.chars.next();
                    return Ok(Token::NewLine);
                }

                if !c.is_whitespace() {
                    break;
                }

                self.chars.next();
            } else {
                return Ok(Token::EOF);
            }
        }

        if let Some((_, c)) = self.chars.peek() {
            match c {
                '.' => self.parse_label(),
                ',' | '(' | ')' | '+' => self.parse_single_char(),
                'a'..='z' | 'A'..='Z' | '0'..='9' => self.parse_identifier(),
                _ => Err(ParseError::UnexpectedChar(c.clone(), self.curr_line)),
            }
        } else {
            Ok(Token::EOF)
        }
    }

    fn parse_label(&mut self) -> Result<Token, ParseError> {
        let _ = self.chars.next(); // '.'
        if let Some((start, _)) = self.chars.next() {
            loop {
                if let Some((p, c)) = self.chars.next() {
                    if c == ':' {
                        return Ok(Token::Label(self.source[start..p].to_string()));
                    }
                } else {
                    return Err(ParseError::UnexpectedEOF(self.curr_line));
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF(self.curr_line))
        }
    }

    fn parse_identifier(&mut self) -> Result<Token, ParseError> {
        if let Some((start, _)) = self.chars.next() {
            let mut end = start + 1;
            loop {
                if let Some((p, c)) = self.chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' => {
                            end = (*p).clone() + 1;
                            let _ = self.chars.next();
                            continue;
                        }
                        _ => return Ok(parse_identifier_or_value(&self.source[start..*p])),
                    }
                } else {
                    return Ok(parse_identifier_or_value(&self.source[start..end]));
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF(self.curr_line))
        }
    }

    fn parse_single_char(&mut self) -> Result<Token, ParseError> {
        match self.chars.next() {
            Some((_, ',')) => Ok(Token::Comma),
            Some((_, '(')) => Ok(Token::OpenParen),
            Some((_, ')')) => Ok(Token::CloseParen),
            Some((_, '+')) => Ok(Token::Plus),
            _ => unreachable!(),
        }
    }
}

fn parse_identifier_or_value(s: &str) -> Token {
    match s.chars().last() {
        Some('h') => {
            let len = s.chars().count();
            let mut acc: u16 = 0;

            for c in s.chars().take(len - 1) {
                if let Some(v) = c.to_digit(16) {
                    acc = acc * 16 + v as u16;
                } else {
                    return Token::Identifier(s.to_string());
                }
            }

            Token::Value(acc)
        }
        Some(_) => Token::Identifier(s.to_string()),
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenizer::{Token, Tokenizer};

    #[test]
    fn test1() {
        let mut parser = Tokenizer::new(
            r#"
.test_label:
ADD    INC
.label2:
"#,
        );

        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(
            Token::Label("test_label".to_string()),
            parser.next().unwrap()
        );
        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(Token::Identifier("ADD".to_string()), parser.next().unwrap());
        assert_eq!(Token::Identifier("INC".to_string()), parser.next().unwrap());
        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(Token::Label("label2".to_string()), parser.next().unwrap());
    }

    #[test]
    fn test_short_value() {
        let mut parser = Tokenizer::new(r"add a, 3Ah");

        assert_eq!(Token::Identifier("add".to_string()), parser.next().unwrap());
        assert_eq!(Token::Identifier("a".to_string()), parser.next().unwrap());
        assert_eq!(Token::Comma, parser.next().unwrap());
        assert_eq!(Token::Value(58), parser.next().unwrap());
    }

    #[test]
    fn test_wide_value() {
        let mut parser = Tokenizer::new(r"add a, 3bAh");

        assert_eq!(Token::Identifier("add".to_string()), parser.next().unwrap());
        assert_eq!(Token::Identifier("a".to_string()), parser.next().unwrap());
        assert_eq!(Token::Comma, parser.next().unwrap());
        assert_eq!(Token::Value(954), parser.next().unwrap());
    }

    #[test]
    fn test_instruction_with_address() {
        let mut parser = Tokenizer::new(
            r#"ld bc, (2130h)
call"#,
        );

        assert_eq!(Token::Identifier("ld".to_string()), parser.next().unwrap());
        assert_eq!(Token::Identifier("bc".to_string()), parser.next().unwrap());
        assert_eq!(Token::Comma, parser.next().unwrap());
        assert_eq!(Token::OpenParen, parser.next().unwrap());
        assert_eq!(Token::Value(8496), parser.next().unwrap());
        assert_eq!(Token::CloseParen, parser.next().unwrap());
        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(
            Token::Identifier("call".to_string()),
            parser.next().unwrap()
        );
        assert_eq!(Token::EOF, parser.next().unwrap());
    }

    #[test]
    fn test_address_reg() {
        let mut parser = Tokenizer::new(r#"(BC)"#);
        assert_eq!(Token::OpenParen, parser.next().unwrap());
        assert_eq!(Token::Identifier("BC".to_string()), parser.next().unwrap());
        assert_eq!(Token::CloseParen, parser.next().unwrap());
    }

    #[test]
    fn test_address_reg_offset() {
        let mut parser = Tokenizer::new(r#"(BC + 19h)"#);
        assert_eq!(Token::OpenParen, parser.next().unwrap());
        assert_eq!(Token::Identifier("BC".to_string()), parser.next().unwrap());
        assert_eq!(Token::Plus, parser.next().unwrap());
        assert_eq!(Token::Value(25), parser.next().unwrap());
        assert_eq!(Token::CloseParen, parser.next().unwrap());
    }

    #[test]
    fn test_peek_next() {
        let mut parser = Tokenizer::new(r"add a, 3Ah");

        assert_eq!(Token::Identifier("add".to_string()), parser.peek().unwrap());
        assert_eq!(Token::Identifier("add".to_string()), parser.next().unwrap());
        assert_eq!(Token::Identifier("a".to_string()), parser.next().unwrap());
        assert_eq!(Token::Comma, parser.peek().unwrap());
        assert_eq!(Token::Comma, parser.next().unwrap());
        assert_eq!(Token::Value(58), parser.next().unwrap());
        assert_eq!(Token::EOF, parser.next().unwrap());
    }
}
