use crate::{object::Object, token::{Token, TokenType}};
use ascii::{AsciiStr, AsciiChar};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("invalid token {0}")]
    InvalidToken(String)
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
                match ch {
                    AsciiChar::ParenOpen => Ok(Some(self.get_token(TokenType::LeftParen, None))),
                    AsciiChar::ParenClose => Ok(Some(self.get_token(TokenType::RightParen, None))),
                    AsciiChar::CurlyBraceOpen => Ok(Some(self.get_token(TokenType::LeftBrace, None))),
                    AsciiChar::CurlyBraceClose => Ok(Some(self.get_token(TokenType::RightBrace, None))),
                    AsciiChar::Comma => Ok(Some(self.get_token(TokenType::Comma, None))),
                    AsciiChar::Dot => Ok(Some(self.get_token(TokenType::Dot, None))),
                    AsciiChar::Minus => Ok(Some(self.get_token(TokenType::Minus, None))),
                    AsciiChar::Plus => Ok(Some(self.get_token(TokenType::Plus, None))),
                    AsciiChar::Semicolon => Ok(Some(self.get_token(TokenType::Semicolon, None))),
                    AsciiChar::Asterisk => Ok(Some(self.get_token(TokenType::Star, None))),
                    AsciiChar::Exclamation => {
                        if self.matches(AsciiChar::Equal) {
                            Ok(Some(self.get_token(TokenType::BangEqual, None)))
                        } else {
                            Ok(Some(self.get_token(TokenType::Bang, None)))
                        }
                    }
                    AsciiChar::Equal => {
                        if self.matches(AsciiChar::Equal) {
                            Ok(Some(self.get_token(TokenType::EqualEqual, None)))
                        } else {
                            Ok(Some(self.get_token(TokenType::Equal, None)))
                        }
                    }
                    AsciiChar::LessThan => {
                        if self.matches(AsciiChar::Equal) {
                            Ok(Some(self.get_token(TokenType::LessEqual, None)))
                        } else {
                            Ok(Some(self.get_token(TokenType::Less, None)))
                        }
                    }
                    AsciiChar::GreaterThan => {
                        if self.matches(AsciiChar::Equal) {
                            Ok(Some(self.get_token(TokenType::GreaterEqual, None)))
                        } else {
                            Ok(Some(self.get_token(TokenType::Greater, None)))
                        }
                    }
                    AsciiChar::Slash => {
                        if self.matches(AsciiChar::Slash) {
                            while !self.is_done() && self.peek() != AsciiChar::LineFeed {
                                self.advance();
                            }
                            self.next_token()
                        } else {
                            Ok(Some(self.get_token(TokenType::Slash, None)))
                        }
                    },
                    AsciiChar::Space | AsciiChar::CarriageReturn | AsciiChar::Tab => self.next_token(),
                    AsciiChar::LineFeed => {
                        self.line += 1;
                        self.next_token()
                    }
                    _ => Err(ScanError::InvalidToken(ch.to_string()))
                }
            },
            None => Ok(None)
        }
    }

    fn get_token(&self, kind: TokenType, literal: Option<Object>) -> Token<'a> {
        let lexeme = &self.source[self.start..self.current];
        Token::new(kind, lexeme, literal, self.line, self.start, self.current)
    }

    fn matches(&mut self, expected: AsciiChar) -> bool {
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
}