#![allow(dead_code)]
use std::fmt::format;

use crate::token::*;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        l.read_char();

        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    Token {
                        token_type: EQ.to_string(),
                        literal: literal,
                    }
                } else {
                    new_token(ASSIGN.to_string(), self.ch)
                }
            }
            '+' => new_token(PLUS.to_string(), self.ch),
            '-' => new_token(MINUS.to_string(), self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    Token {
                        token_type: NOT_EQ.to_string(),
                        literal: literal,
                    }
                } else {
                    new_token(BANG.to_string(), self.ch)
                }
            }
            '/' => new_token(SLASH.to_string(), self.ch),
            '*' => new_token(ASTERISK.to_string(), self.ch),
            '<' => new_token(LT.to_string(), self.ch),
            '>' => new_token(GT.to_string(), self.ch),
            ';' => new_token(SEMICOLON.to_string(), self.ch),
            '(' => new_token(LPAREN.to_string(), self.ch),
            ')' => new_token(RPAREN.to_string(), self.ch),
            ',' => new_token(COMMA.to_string(), self.ch),
            '{' => new_token(LBRACE.to_string(), self.ch),
            '}' => new_token(RBRACE.to_string(), self.ch),
            '\0' => new_token(EOF.to_string(), '\0'),
            _ => {
                if is_letter(self.ch) {
                    Token {
                        literal: self.read_identifier(),
                        token_type: lookup_ident(&self.read_identifier()),
                    }
                } else if is_digit(self.ch) {
                    Token {
                        token_type: INT.to_string(),
                        literal: self.read_number(),
                    }
                } else {
                    new_token(ILLEGAL.to_string(), self.ch)
                }
            }
        };

        self.read_char();

        tok
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;

        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while is_digit(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type: token_type,
        literal: ch.to_string(),
    }
}
