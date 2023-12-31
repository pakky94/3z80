use crate::parser::errors::{ParseError, UnexpectedToken};
use crate::parser::token::{Token, TokenValue};
use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::CharIndices;

pub trait Tokenizer {
    fn peek(&mut self) -> Result<Token, ParseError>;
    fn expect(&mut self, expected: TokenValue) -> Result<(), ParseError>;
    fn expect_peek(&mut self, expected: TokenValue) -> Result<(), ParseError>;
    fn next(&mut self) -> Result<Token, ParseError>;
}

#[derive(Debug)]
pub struct SimpleTokenizer<'a> {
    source: &'a str,
    file_id: usize,
    chars: Peekable<CharIndices<'a>>,
    curr_line: usize,
    head: Option<Token>,
}

#[derive(Debug)]
pub struct BufferedTokenizer<'a> {
    tokenizer: SimpleTokenizer<'a>,
    buffer: VecDeque<Token>,
}

impl<'a> BufferedTokenizer<'a> {
    pub fn new(source: &'a str, file_id: usize) -> Self {
        BufferedTokenizer {
            tokenizer: SimpleTokenizer {
                source,
                file_id,
                chars: source.char_indices().peekable(),
                curr_line: 1,
                head: None,
            },
            buffer: VecDeque::new(),
        }
    }

    fn expect_token(&self, actual: Token, expected: TokenValue) -> Result<(), ParseError> {
        if actual.token == expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(UnexpectedToken {
                expected,
                actual: actual.token,
                line: self.tokenizer.curr_line,
                file_id: self.tokenizer.file_id,
                char: 0,
            }))
        }
    }

    pub fn push_front(&mut self, tokens: &Vec<Token>) {
        for t in tokens.into_iter().rev() {
            self.buffer.push_front(t.clone())
        }
    }
}

impl<'a> Tokenizer for BufferedTokenizer<'a> {
    fn peek(&mut self) -> Result<Token, ParseError> {
        if let Some(t) = self.buffer.front() {
            Ok(t.clone())
        } else {
            self.tokenizer.peek()
        }
    }

    fn expect(&mut self, expected: TokenValue) -> Result<(), ParseError> {
        if let Some(t) = self.buffer.pop_front() {
            self.expect_token(t, expected)
        } else {
            self.tokenizer.expect(expected)
        }
    }

    fn expect_peek(&mut self, expected: TokenValue) -> Result<(), ParseError> {
        if let Some(t) = self.buffer.front() {
            self.expect_token(t.clone(), expected)
        } else {
            self.tokenizer.expect_peek(expected)
        }
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        if let Some(t) = self.buffer.pop_front() {
            Ok(t.clone())
        } else {
            self.tokenizer.next()
        }
    }
}

impl<'a> SimpleTokenizer<'a> {
    pub fn new(source: &'a str, file_id: usize) -> Self {
        SimpleTokenizer {
            source,
            file_id,
            chars: source.char_indices().peekable(),
            curr_line: 1,
            head: None,
        }
    }

    fn expect_token(&self, actual: Token, expected: TokenValue) -> Result<(), ParseError> {
        if actual.token == expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(UnexpectedToken {
                expected,
                actual: actual.token,
                line: self.curr_line,
                file_id: self.file_id,
                char: 0,
            }))
        }
    }

    fn parse_identifier(&mut self) -> Result<Token, ParseError> {
        if let Some((start, _)) = self.chars.next() {
            let mut end = start + 1;
            loop {
                if let Some((p, c)) = self.chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'' => {
                            end = (*p).clone() + 1;
                            let _ = self.chars.next();
                            continue;
                        }
                        _ => {
                            let end = (*p).clone();
                            return Ok(self.create_token(parse_identifier_or_value(
                                &self.source[start..end],
                            )));
                        }
                    }
                } else {
                    return Ok(
                        self.create_token(parse_identifier_or_value(&self.source[start..end]))
                    );
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF(self.curr_line))
        }
    }

    fn parse_single_char(&mut self) -> Result<Token, ParseError> {
        match self.chars.next() {
            Some((_, ',')) => Ok(self.create_token(TokenValue::Comma)),
            Some((_, '(')) => Ok(self.create_token(TokenValue::OpenParen)),
            Some((_, ')')) => Ok(self.create_token(TokenValue::CloseParen)),
            Some((_, '+')) => Ok(self.create_token(TokenValue::Plus)),
            Some((_, '-')) => Ok(self.create_token(TokenValue::Minus)),
            Some((_, '.')) => Ok(self.create_token(TokenValue::Dot)),
            Some((_, ':')) => Ok(self.create_token(TokenValue::Colon)),
            Some((_, '&')) => Ok(self.create_token(TokenValue::Amp)),
            Some((_, '*')) => Ok(self.create_token(TokenValue::Asterisk)),
            Some((_, '@')) => Ok(self.create_token(TokenValue::At)),
            _ => unreachable!(),
        }
    }

    fn parse_directive(&mut self) -> Result<Token, ParseError> {
        if let Some((start, _)) = self.chars.next() {
            let mut end = start + 1;
            loop {
                if let Some((p, c)) = self.chars.peek() {
                    match c {
                        ' ' | '\n' => {
                            let end = (*p).clone();
                            return Ok(self.create_token(TokenValue::Directive(
                                self.source[start..end].to_string(),
                            )));
                        }
                        _ => {
                            end = (*p).clone() + 1;
                            let _ = self.chars.next();
                            continue;
                        }
                    }
                } else {
                    return Ok(self
                        .create_token(TokenValue::Directive(self.source[start..end].to_string())));
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF(self.curr_line))
        }
    }

    fn create_token(&self, token: TokenValue) -> Token {
        Token {
            token,
            line: self.curr_line,
            file_id: self.file_id,
        }
    }

    pub fn collect_all(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut out = vec![];

        loop {
            let t = self.next()?;
            if t.token == TokenValue::EOF {
                break;
            }

            out.push(t);
        }

        Ok(out)
    }
}

impl<'a> Tokenizer for SimpleTokenizer<'a> {
    fn peek(&mut self) -> Result<Token, ParseError> {
        if let Some(t) = &self.head {
            return Ok((*t).clone());
        } else {
            let t = self.next()?;
            self.head = Some(t.clone());
            Ok(t)
        }
    }

    fn expect(&mut self, expected: TokenValue) -> Result<(), ParseError> {
        let actual = self.next()?;
        self.expect_token(actual, expected)
    }

    fn expect_peek(&mut self, expected: TokenValue) -> Result<(), ParseError> {
        let actual = self.peek()?;
        self.expect_token(actual, expected)
    }

    fn next(&mut self) -> Result<Token, ParseError> {
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
                    return Ok(self.create_token(TokenValue::NewLine));
                }

                if *c == ';' {
                    // comment
                    loop {
                        match self.chars.peek() {
                            Some((_, '\n')) => {
                                self.curr_line += 1;
                                self.chars.next();
                                return Ok(self.create_token(TokenValue::NewLine));
                            }
                            Some(_) => {
                                self.chars.next();
                            }
                            None => return Ok(self.create_token(TokenValue::EOF)),
                        }
                    }
                }

                if !c.is_whitespace() {
                    break;
                }

                self.chars.next();
            } else {
                return Ok(self.create_token(TokenValue::EOF));
            }
        }

        if let Some((_, c)) = self.chars.peek() {
            match c {
                '#' => self.parse_directive(),
                ',' | '(' | ')' | '+' | '-' | '.' | ':' | '&' | '*' | '@' => {
                    self.parse_single_char()
                }
                'a'..='z' | 'A'..='Z' | '0'..='9' => self.parse_identifier(),
                _ => Err(ParseError::UnexpectedChar(c.clone(), self.curr_line)),
            }
        } else {
            Ok(self.create_token(TokenValue::EOF))
        }
    }
}

fn parse_identifier_or_value(s: &str) -> TokenValue {
    match s.chars().last() {
        Some('h') => {
            let len = s.chars().count();
            let mut acc: u16 = 0;

            for c in s.chars().take(len - 1) {
                if let Some(v) = c.to_digit(16) {
                    acc = acc * 16 + v as u16;
                } else {
                    return TokenValue::Identifier(s.to_string());
                }
            }

            TokenValue::Value(acc, (len / 2) as u8)
        }
        Some(_) => TokenValue::Identifier(s.to_string()),
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::TokenValue;
    use crate::parser::tokenizer::{SimpleTokenizer, Tokenizer};

    #[test]
    fn test1() {
        let mut parser = SimpleTokenizer::new(
            r#"
.test_label:
ADD    INC
.label2:
"#,
            0,
        );

        assert_eq!(TokenValue::NewLine, parser.next().unwrap().token);
        assert_eq!(TokenValue::Dot, parser.next().unwrap().token);
        assert_eq!(
            TokenValue::Identifier("test_label".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::Colon, parser.next().unwrap().token);
        assert_eq!(TokenValue::NewLine, parser.next().unwrap().token);
        assert_eq!(
            TokenValue::Identifier("ADD".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(
            TokenValue::Identifier("INC".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::NewLine, parser.next().unwrap().token);
        assert_eq!(TokenValue::Dot, parser.next().unwrap().token);
        assert_eq!(
            TokenValue::Identifier("label2".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::Colon, parser.next().unwrap().token);
    }

    #[test]
    fn test_short_value() {
        let mut parser = SimpleTokenizer::new(r"add a, 3Ah", 0);

        assert_eq!(
            TokenValue::Identifier("add".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(
            TokenValue::Identifier("a".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::Comma, parser.next().unwrap().token);
        assert_eq!(TokenValue::Value(58, 1), parser.next().unwrap().token);
    }

    #[test]
    fn test_wide_value() {
        let mut parser = SimpleTokenizer::new(r"add a, 3bAh", 0);

        assert_eq!(
            TokenValue::Identifier("add".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(
            TokenValue::Identifier("a".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::Comma, parser.next().unwrap().token);
        assert_eq!(TokenValue::Value(954, 2), parser.next().unwrap().token);
    }

    #[test]
    fn test_instruction_with_address() {
        let mut parser = SimpleTokenizer::new(
            r#"ld bc, (2130h)
call"#,
            0,
        );

        assert_eq!(
            TokenValue::Identifier("ld".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(
            TokenValue::Identifier("bc".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::Comma, parser.next().unwrap().token);
        assert_eq!(TokenValue::OpenParen, parser.next().unwrap().token);
        assert_eq!(TokenValue::Value(8496, 2), parser.next().unwrap().token);
        assert_eq!(TokenValue::CloseParen, parser.next().unwrap().token);
        assert_eq!(TokenValue::NewLine, parser.next().unwrap().token);
        assert_eq!(
            TokenValue::Identifier("call".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::EOF, parser.next().unwrap().token);
    }

    #[test]
    fn test_address_reg() {
        let mut parser = SimpleTokenizer::new(r#"(BC)"#, 0);
        assert_eq!(TokenValue::OpenParen, parser.next().unwrap().token);
        assert_eq!(
            TokenValue::Identifier("BC".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::CloseParen, parser.next().unwrap().token);
    }

    #[test]
    fn test_address_reg_offset() {
        let mut parser = SimpleTokenizer::new(r#"(BC + 9h)"#, 0);
        assert_eq!(TokenValue::OpenParen, parser.next().unwrap().token);
        assert_eq!(
            TokenValue::Identifier("BC".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::Plus, parser.next().unwrap().token);
        assert_eq!(TokenValue::Value(9, 1), parser.next().unwrap().token);
        assert_eq!(TokenValue::CloseParen, parser.next().unwrap().token);
    }

    #[test]
    fn test_peek_next() {
        let mut parser = SimpleTokenizer::new(r"add a, 3Ah", 0);

        assert_eq!(
            TokenValue::Identifier("add".to_string()),
            parser.peek().unwrap().token
        );
        assert_eq!(
            TokenValue::Identifier("add".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(
            TokenValue::Identifier("a".to_string()),
            parser.next().unwrap().token
        );
        assert_eq!(TokenValue::Comma, parser.peek().unwrap().token);
        assert_eq!(TokenValue::Comma, parser.next().unwrap().token);
        assert_eq!(TokenValue::Value(58, 1), parser.next().unwrap().token);
        assert_eq!(TokenValue::EOF, parser.next().unwrap().token);
    }
}
