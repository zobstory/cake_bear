//! Lexer: TypeScript source `&str` → token stream.
//!
//! Phase 1 token set: keywords (`function`/`const`/`let`/`return`/`if`/`else`/
//! `while`/`true`/`false`/`null`/`undefined`/`number`/`string`/`boolean`),
//! identifiers, number and string literals, and the punctuation/operators
//! needed for arithmetic, boolean ops, comparisons, and call/assignment
//! syntax.
//!
//! Hand-rolled, zero-dep. Whitespace and `//` / `/* */` comments are skipped.

use cakec_ast::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Ident(String),
    Number(f64),
    Str(String),

    // Keywords
    Function,
    Const,
    Let,
    Return,
    If,
    Else,
    While,
    True,
    False,
    Null,
    Undefined,

    // Type-name keywords
    NumberKw,
    StringKw,
    BooleanKw,

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semi,
    Colon,
    Dot,

    // Operators
    Eq,
    EqEq,
    EqEqEq,
    NotEq,
    NotEqEq,
    Plus,
    Minus,
    Star,
    Slash,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AmpAmp,
    PipePipe,
    Bang,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    UnterminatedString,
    InvalidNumber,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            LexErrorKind::UnexpectedChar(c) => {
                write!(f, "unexpected character `{c}` at {}..{}", self.span.start, self.span.end)
            }
            LexErrorKind::UnterminatedString => {
                write!(f, "unterminated string literal at {}..{}", self.span.start, self.span.end)
            }
            LexErrorKind::InvalidNumber => {
                write!(f, "invalid numeric literal at {}..{}", self.span.start, self.span.end)
            }
        }
    }
}

impl std::error::Error for LexError {}

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut lx = Lexer::new(src);
    let mut out = Vec::new();
    loop {
        let tok = lx.next_token()?;
        let done = matches!(tok.kind, TokenKind::Eof);
        out.push(tok);
        if done {
            return Ok(out);
        }
    }
}

struct Lexer<'a> {
    src: &'a str,
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self { src, bytes: src.as_bytes(), pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn peek_at(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }

    fn skip_trivia(&mut self) {
        loop {
            match self.peek() {
                Some(b' ' | b'\t' | b'\n' | b'\r') => self.pos += 1,
                Some(b'/') if self.peek_at(1) == Some(b'/') => {
                    self.pos += 2;
                    while let Some(b) = self.peek() {
                        self.pos += 1;
                        if b == b'\n' {
                            break;
                        }
                    }
                }
                Some(b'/') if self.peek_at(1) == Some(b'*') => {
                    self.pos += 2;
                    while let Some(b) = self.peek() {
                        if b == b'*' && self.peek_at(1) == Some(b'/') {
                            self.pos += 2;
                            break;
                        }
                        self.pos += 1;
                    }
                }
                _ => break,
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_trivia();
        let start = self.pos as u32;

        let b = match self.peek() {
            None => {
                return Ok(Token {
                    kind: TokenKind::Eof,
                    span: Span { start, end: start },
                });
            }
            Some(b) => b,
        };

        let kind = match b {
            b'(' => self.single(TokenKind::LParen),
            b')' => self.single(TokenKind::RParen),
            b'{' => self.single(TokenKind::LBrace),
            b'}' => self.single(TokenKind::RBrace),
            b',' => self.single(TokenKind::Comma),
            b';' => self.single(TokenKind::Semi),
            b':' => self.single(TokenKind::Colon),
            b'.' => self.single(TokenKind::Dot),
            b'+' => self.single(TokenKind::Plus),
            b'-' => self.single(TokenKind::Minus),
            b'*' => self.single(TokenKind::Star),
            b'/' => self.single(TokenKind::Slash),
            b'=' => self.lex_eq(),
            b'!' => self.lex_bang(),
            b'<' => self.lex_lt(),
            b'>' => self.lex_gt(),
            b'&' if self.peek_at(1) == Some(b'&') => {
                self.pos += 2;
                TokenKind::AmpAmp
            }
            b'|' if self.peek_at(1) == Some(b'|') => {
                self.pos += 2;
                TokenKind::PipePipe
            }
            b'"' | b'\'' => self.lex_string(start)?,
            b'0'..=b'9' => self.lex_number(start)?,
            b if is_ident_start(b) => self.lex_ident_or_keyword(),
            _ => {
                let ch = self.src[self.pos..].chars().next().unwrap_or('?');
                let end = self.pos as u32 + ch.len_utf8() as u32;
                self.pos += ch.len_utf8();
                return Err(LexError {
                    kind: LexErrorKind::UnexpectedChar(ch),
                    span: Span { start, end },
                });
            }
        };

        let end = self.pos as u32;
        Ok(Token { kind, span: Span { start, end } })
    }

    fn single(&mut self, kind: TokenKind) -> TokenKind {
        self.pos += 1;
        kind
    }

    fn lex_eq(&mut self) -> TokenKind {
        self.pos += 1;
        if self.peek() == Some(b'=') {
            self.pos += 1;
            if self.peek() == Some(b'=') {
                self.pos += 1;
                TokenKind::EqEqEq
            } else {
                TokenKind::EqEq
            }
        } else {
            TokenKind::Eq
        }
    }

    fn lex_bang(&mut self) -> TokenKind {
        self.pos += 1;
        if self.peek() == Some(b'=') {
            self.pos += 1;
            if self.peek() == Some(b'=') {
                self.pos += 1;
                TokenKind::NotEqEq
            } else {
                TokenKind::NotEq
            }
        } else {
            TokenKind::Bang
        }
    }

    fn lex_lt(&mut self) -> TokenKind {
        self.pos += 1;
        if self.peek() == Some(b'=') {
            self.pos += 1;
            TokenKind::LtEq
        } else {
            TokenKind::Lt
        }
    }

    fn lex_gt(&mut self) -> TokenKind {
        self.pos += 1;
        if self.peek() == Some(b'=') {
            self.pos += 1;
            TokenKind::GtEq
        } else {
            TokenKind::Gt
        }
    }

    fn lex_string(&mut self, start: u32) -> Result<TokenKind, LexError> {
        let quote = self.bytes[self.pos];
        self.pos += 1;
        let mut s = String::new();
        loop {
            match self.peek() {
                None | Some(b'\n') => {
                    return Err(LexError {
                        kind: LexErrorKind::UnterminatedString,
                        span: Span { start, end: self.pos as u32 },
                    });
                }
                Some(b) if b == quote => {
                    self.pos += 1;
                    return Ok(TokenKind::Str(s));
                }
                Some(b'\\') => {
                    self.pos += 1;
                    match self.peek() {
                        Some(b'n') => { s.push('\n'); self.pos += 1; }
                        Some(b't') => { s.push('\t'); self.pos += 1; }
                        Some(b'r') => { s.push('\r'); self.pos += 1; }
                        Some(b'\\') => { s.push('\\'); self.pos += 1; }
                        Some(b'\'') => { s.push('\''); self.pos += 1; }
                        Some(b'"') => { s.push('"'); self.pos += 1; }
                        Some(b'0') => { s.push('\0'); self.pos += 1; }
                        _ => {
                            let ch = self.src[self.pos..].chars().next().unwrap_or('?');
                            s.push(ch);
                            self.pos += ch.len_utf8();
                        }
                    }
                }
                Some(_) => {
                    let ch = self.src[self.pos..].chars().next().unwrap();
                    s.push(ch);
                    self.pos += ch.len_utf8();
                }
            }
        }
    }

    fn lex_number(&mut self, start: u32) -> Result<TokenKind, LexError> {
        let begin = self.pos;
        while let Some(b) = self.peek() {
            if b.is_ascii_digit() { self.pos += 1; } else { break; }
        }
        if self.peek() == Some(b'.') && self.peek_at(1).map_or(false, |b| b.is_ascii_digit()) {
            self.pos += 1;
            while let Some(b) = self.peek() {
                if b.is_ascii_digit() { self.pos += 1; } else { break; }
            }
        }
        let text = &self.src[begin..self.pos];
        match text.parse::<f64>() {
            Ok(n) => Ok(TokenKind::Number(n)),
            Err(_) => Err(LexError {
                kind: LexErrorKind::InvalidNumber,
                span: Span { start, end: self.pos as u32 },
            }),
        }
    }

    fn lex_ident_or_keyword(&mut self) -> TokenKind {
        let begin = self.pos;
        while let Some(b) = self.peek() {
            if is_ident_cont(b) { self.pos += 1; } else { break; }
        }
        match &self.src[begin..self.pos] {
            "function" => TokenKind::Function,
            "const" => TokenKind::Const,
            "let" => TokenKind::Let,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "undefined" => TokenKind::Undefined,
            "number" => TokenKind::NumberKw,
            "string" => TokenKind::StringKw,
            "boolean" => TokenKind::BooleanKw,
            other => TokenKind::Ident(other.to_owned()),
        }
    }
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b == b'$'
}

fn is_ident_cont(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'$'
}

#[cfg(test)]
mod tests;
