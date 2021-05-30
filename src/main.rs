mod token;
mod lexer;
mod repl;
use std::io;

fn main() {
    let mut r = io::stdin();
    let mut w = io::stdout();
    repl::start(&mut r, &mut w);
}
