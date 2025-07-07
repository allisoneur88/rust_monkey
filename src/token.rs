#![allow(dead_code)]
use std::collections::HashMap;

pub type TokenType = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

// Define token type constants
pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

pub const LT: &str = "<";
pub const GT: &str = ">";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

// Use lazy_static to initialize the keyword map only once
use once_cell::sync::Lazy;

pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("fn", FUNCTION.to_string());
    m.insert("let", LET.to_string());
    m.insert("true", TRUE.to_string());
    m.insert("false", FALSE.to_string());
    m.insert("if", IF.to_string());
    m.insert("else", ELSE.to_string());
    m.insert("return", RETURN.to_string());
    m
});

pub fn lookup_ident(ident: &str) -> TokenType {
    KEYWORDS
        .get(ident)
        .cloned()
        .unwrap_or_else(|| IDENT.to_string())
}
