use crate::{object::Object, token::{Token, TokenType}};
use ascii::{AsciiStr, AsciiChar};
use thiserror::Error;
use phf::phf_map;

static RESERVED: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While
};

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("invalid token {0}")]
    InvalidToken(String),

    #[error("unterminated string")]
    UnterminatedString,

    #[error("invalid number")]
    InvalidNumber(#[from] std::num::ParseFloatError)
}

pub struct Scanner<'a> {
    source: &'a AsciiStr,
    start: usize,
    current: usize,
    line: usize
}

impl <'a> Scanner<'a> {
    pub fn new(source: &'a AsciiStr) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 0
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token<'a>>, ScanError> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token)
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token<'a>>, ScanError> {
        self.start = self.current;
        match self.advance() {
            Some(ch) => {
                match ch.as_byte() {
                    b'(' => Ok(Some(self.get_token(TokenType::LeftParen, Object::None))),
                    b')' => Ok(Some(self.get_token(TokenType::RightParen, Object::None))),
                    b'{' => Ok(Some(self.get_token(TokenType::LeftBrace, Object::None))),
                    b'}' => Ok(Some(self.get_token(TokenType::RightBrace, Object::None))),
                    b','=> Ok(Some(self.get_token(TokenType::Comma, Object::None))),
                    b'.' => Ok(Some(self.get_token(TokenType::Dot, Object::None))),
                    b'-' => Ok(Some(self.get_token(TokenType::Minus, Object::None))),
                    b'+' => Ok(Some(self.get_token(TokenType::Plus, Object::None))),
                    b';' => Ok(Some(self.get_token(TokenType::Semicolon, Object::None))),
                    b'*' => Ok(Some(self.get_token(TokenType::Star, Object::None))),
                    b'!' => {
                        if self.matches(b'=') {
                            Ok(Some(self.get_token(TokenType::BangEqual, Object::None)))
                        } else {
                            Ok(Some(self.get_token(TokenType::Bang, Object::None)))
                        }
                    }
                    b'=' => {
                        if self.matches(b'=') {
                            Ok(Some(self.get_token(TokenType::EqualEqual, Object::None)))
                        } else {
                            Ok(Some(self.get_token(TokenType::Equal, Object::None)))
                        }
                    }
                    b'<' => {
                        if self.matches(b'=') {
                            Ok(Some(self.get_token(TokenType::LessEqual, Object::None)))
                        } else {
                            Ok(Some(self.get_token(TokenType::Less, Object::None)))
                        }
                    }
                    b'>' => {
                        if self.matches(b'=') {
                            Ok(Some(self.get_token(TokenType::GreaterEqual, Object::None)))
                        } else {
                            Ok(Some(self.get_token(TokenType::Greater, Object::None)))
                        }
                    }
                    b'/' => {
                        if self.matches(b'/') {
                            while !self.is_done() && self.peek() != AsciiChar::LineFeed {
                                self.advance();
                            }
                            self.next_token()
                        } else {
                            Ok(Some(self.get_token(TokenType::Slash, Object::None)))
                        }
                    },
                    b' ' | b'\r' | b'\t' => self.next_token(),
                    b'\n' => {
                        self.line += 1;
                        self.next_token()
                    },
                    b'"' => {
                        let literal = self.string()?;
                        let owned = literal.to_owned();
                        Ok(Some(self.get_token(TokenType::String, Object::Str(owned))))
                    },
                    d if d.is_ascii_digit() => {
                        let literal = self.number()?;
                        Ok(Some(self.get_token(TokenType::Number, Object::Num(literal))))
                    },
                    c if c.is_ascii_alphabetic() => {
                        self.identifier();
                        let id = &self.source[self.start..self.current];
                        match RESERVED.get(id.as_str()) {
                            Some(&kw) => Ok(Some(self.get_token(kw, Object::None))),
                            None => Ok(Some(self.get_token(TokenType::Identifier(id), Object::None)))
                        }
                    },
                    _ => Err(ScanError::InvalidToken(ch.to_string()))
                }
            },
            None => Ok(None)
        }
    }

    fn get_token(&self, kind: TokenType<'a>, literal: Object) -> Token<'a> {
        let lexeme = &self.source[self.start..self.current];
        Token::new(kind, lexeme, literal, self.line, self.start, self.current)
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.is_done() {
            false
        } else if self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> AsciiChar {
        self.source[self.current]
    }

    fn peek_next(&self) -> AsciiChar {
        self.source[self.current + 1]
    }

    fn is_done(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<AsciiChar> {
        let c = if self.is_done() {
            None
        } else {
            Some(self.source[self.current])
        };
        self.current += 1;
        c
    }

    fn string(&mut self) -> Result<&AsciiStr, ScanError> {
        while !self.is_done() && self.peek() != AsciiChar::Quotation {
            if self.peek() == AsciiChar::LineFeed {
                self.line += 1
            }
            self.advance();
        }
        if self.is_done() {
            Err(ScanError::UnterminatedString)
        } else {
            self.advance();
            Ok(&self.source[self.start + 1..self.current - 1])
        }
    }

    fn number(&mut self) -> Result<f64, ScanError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == AsciiChar::Dot && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let chars = self.source[self.start..self.current].as_str();
        Ok(chars.parse::<f64>()?)
    }

    fn identifier(&mut self) {
        while !self.is_done() && self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
    }

}