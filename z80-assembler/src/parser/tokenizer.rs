use std::iter::Peekable;
use std::str::{CharIndices};
use crate::parser::errors::ParseError;
use crate::parser::token::Token;

pub struct Tokenizer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    curr_line: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Tokenizer {
            source,
            chars: source.char_indices().peekable(),
            curr_line: 1,
        }
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        loop {
            if let Some((_, c)) = self.chars.peek() {
                if *c == '\n' {
                    self.curr_line += 1;
                    self.chars.next();
                    return Ok(Token::NewLine)
                }

                if !c.is_whitespace() {
                    break;
                }

                self.chars.next();
            } else {
                return Ok(Token::EOF)
            }
        }

        if let Some((_, c)) = self.chars.peek() {
            match c {
                '.' => self.parse_label(),
                '&' => self.parse_address(),
                ',' => self.parse_single_char(),
                'a'..='z' | 'A'..='Z' | '0'..='9' => self.parse_identifier(),
                _ => Err(ParseError::UnexpectedChar(c.clone(), self.curr_line))
            }
        } else {
            Ok(Token::EOF)
        }
    }

    fn parse_label(&mut self) -> Result<Token, ParseError> {
        let _ = self.chars.next(); // '.'
        if let Some((start, _)) = self.chars.next(){
            loop {
                if let Some((p, c)) = self.chars.next() {
                    if c == ':' {
                        return Ok(Token::Label(&self.source[start..p]))
                    }
                } else {
                    return Err(ParseError::UnexpectedEOF(self.curr_line))
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF(self.curr_line))
        }
    }

    fn parse_identifier(&mut self) -> Result<Token, ParseError> {
        if let Some((start, _)) = self.chars.next(){
            let mut end = start + 1;
            loop {
                if let Some((p, c)) = self.chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' => {
                            end = (*p).clone() + 1;
                            let _ = self.chars.next();
                            continue
                        },
                        _ => return Ok(parse_identifier_or_value(&self.source[start..*p]))
                    }
                } else {
                    return Ok(parse_identifier_or_value(&self.source[start..end]))
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF(self.curr_line))
        }
    }

    fn parse_address(&mut self) -> Result<Token, ParseError> {
        let _ = self.chars.next(); // '&'

        match self.parse_identifier() {
            Ok(Token::WideValue(val)) => Ok(Token::Address(val)),
            e => unimplemented!("error handling for invalid address {:?}", e)
        }
    }

    fn parse_single_char(&mut self) -> Result<Token, ParseError> {
        match self.chars.next() {
            Some((_, ',')) => Ok(Token::Comma),
            _ => unreachable!()
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
                    return Token::Identifier(s);
                }
            }

            if acc < 256 {
                Token::ShortValue(acc as u8)
            } else {
                Token::WideValue(acc)
            }
        }
        Some(_) => Token::Identifier(s),
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::tokenizer::{Token, Tokenizer};

    #[test]
    fn test1() {
        let mut parser = Tokenizer::new(r#"
.test_label:
ADD    INC
.label2:
"#);

        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(Token::Label("test_label"), parser.next().unwrap());
        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(Token::Identifier("ADD"), parser.next().unwrap());
        assert_eq!(Token::Identifier("INC"), parser.next().unwrap());
        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(Token::Label("label2"), parser.next().unwrap());
    }

    #[test]
    fn test_short_value() {
        let mut parser = Tokenizer::new(r"add a, 3Ah");

        assert_eq!(Token::Identifier("add"), parser.next().unwrap());
        assert_eq!(Token::Identifier("a"), parser.next().unwrap());
        assert_eq!(Token::Comma, parser.next().unwrap());
        assert_eq!(Token::ShortValue(58), parser.next().unwrap());
    }

    #[test]
    fn test_wide_value() {
        let mut parser = Tokenizer::new(r"add a, 3bAh");

        assert_eq!(Token::Identifier("add"), parser.next().unwrap());
        assert_eq!(Token::Identifier("a"), parser.next().unwrap());
        assert_eq!(Token::Comma, parser.next().unwrap());
        assert_eq!(Token::WideValue(954), parser.next().unwrap());
    }

    #[test]
    fn test_instruction_with_address() {
        let mut parser = Tokenizer::new(r#"ld bc, &2130h
call"#);

        assert_eq!(Token::Identifier("ld"), parser.next().unwrap());
        assert_eq!(Token::Identifier("bc"), parser.next().unwrap());
        assert_eq!(Token::Comma, parser.next().unwrap());
        assert_eq!(Token::Address(8496), parser.next().unwrap());
        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(Token::Identifier("call"), parser.next().unwrap());
        assert_eq!(Token::EOF, parser.next().unwrap());
    }
}
