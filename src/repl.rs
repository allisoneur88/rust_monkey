use crate::lexer::Lexer;
use crate::token::{EOF, Token};
use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) {
    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        let mut line = String::new();
        let bytes_read = input.read_line(&mut line).unwrap();

        if bytes_read == 0 {
            // EOF or user exied
            break;
        }

        let mut lexer = Lexer::new(line);

        loop {
            let tok: Token = lexer.next_token();
            if tok.token_type == EOF {
                break;
            }
            writeln!(output, "{:?}", tok).unwrap();
        }
    }
}
