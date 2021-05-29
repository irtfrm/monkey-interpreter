use crate::token::Token;
use crate::token::TokenType;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.bytes().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match String::from_utf8(vec![self.ch]){
            Err(why) => panic!("{:?}", why),
            Ok(literal) => {
                match self.ch {
                    b'=' => Token::new(TokenType::ASSIGN, literal),
                    b'+' => Token::new(TokenType::PLUS, literal),
                    b',' => Token::new(TokenType::COMMA, literal),
                    b';' => Token::new(TokenType::SEMICOLON, literal),
                    b'(' => Token::new(TokenType::LPAREN, literal),
                    b')' => Token::new(TokenType::RPAREN, literal),
                    b'{' => Token::new(TokenType::LBRACE, literal),
                    b'}' => Token::new(TokenType::RBRACE, literal),
                    _ => Token::new(TokenType::EOF, "".to_string()),
                }
            }
        };
        self.read_char();
        return tok;
    }
}

#[cfg(test)]
mod test {
    use crate::token::Token;
    use crate::token::TokenType;
    use super::*;

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let tests: [Token; 9] = [
            Token::new(TokenType::ASSIGN, "=".to_string()),
            Token::new(TokenType::PLUS, "+".to_string()),
            Token::new(TokenType::LPAREN, "(".to_string()),
            Token::new(TokenType::RPAREN, ")".to_string()),
            Token::new(TokenType::LBRACE, "{".to_string()),
            Token::new(TokenType::RBRACE, "}".to_string()),
            Token::new(TokenType::COMMA, ",".to_string()),
            Token::new(TokenType::SEMICOLON, ";".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
        ];

        let mut l = Lexer::new(input);

        for expected in &tests {
            let tok: Token = l.next_token();
            assert_eq!(tok, *expected);
        }
        // unimplemented!();
    }
}
