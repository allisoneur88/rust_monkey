mod lexer;
mod repl;
mod token;

use std::io;

fn main() {
    println!("Monkey REPL");
    repl::start(io::stdin().lock(), io::stdout());
}
