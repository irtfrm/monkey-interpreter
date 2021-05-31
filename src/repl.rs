use std::io;
use crate::lexer::Lexer;
use crate::token;

const PROMPT: &[u8; 3] = b">> ";

pub fn start(reader: &mut io::Stdin, writer: &mut dyn io::Write) {
    let mut buf = String::new();
    writer.write_all(PROMPT).unwrap();
    writer.flush().unwrap();
    let mut prev = 0;
    loop {
        if let Ok(n) = reader.read_line(&mut buf) {
            let input = &buf[prev..prev+n];
            let mut lexer = Lexer::new(input);
            loop {
                let tok = lexer.next_token();
                if tok == token::Token::from_str(token::TokenType::EOF, "") {
                    break;
                }
                writer.write_fmt(format_args!("{}\n", tok)).unwrap();
            }
            prev += n;
        }

        writer.write_all(PROMPT).unwrap();
        writer.flush().unwrap();
    }
}