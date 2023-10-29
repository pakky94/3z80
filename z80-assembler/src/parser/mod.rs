mod token;

use std::iter::Peekable;
use std::str::{CharIndices};
use token::Token;

pub fn test() {
    println!("parser test module");
}

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
                'a'..='z' | 'A'..='Z' => self.parse_identifier(),
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
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF(self.curr_line))
        }
    }

    fn parse_identifier(&mut self) -> Result<Token, ParseError> {
        if let Some((start, _)) = self.chars.next(){
            loop {
                if let Some((p, c)) = self.chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' => {
                            let _ = self.next();
                            continue
                        },
                        _ => return Ok(Token::Identifier(&self.source[start..*p]))
                    }
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF(self.curr_line))
        }
    }
}

#[derive(Debug)]
enum ParseError {
    UnexpectedChar(char, usize),
    UnexpectedEOF(usize)
}

#[cfg(test)]
mod tests {
    use crate::parser::{Tokenizer, Token};

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
        assert_eq!(Token::Label("test_label"), parser.next().unwrap());
        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(Token::Identifier("ADD"), parser.next().unwrap());
        assert_eq!(Token::Identifier("INC"), parser.next().unwrap());
        assert_eq!(Token::NewLine, parser.next().unwrap());
        assert_eq!(Token::Label("label2"), parser.next().unwrap());
    }
}
