use crate::token;
use crate::token::Token;
use crate::token::TokenType;

pub fn is_letter(ch: u8) -> bool {
    return b'A' <= ch && ch <= b'Z' || b'a' <= ch && ch <= b'z' || ch == b'_';
}

pub fn is_digit(ch: u8) -> bool {
    return b'0' <= ch && ch <= b'9';
}

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
            ch: 0,
        };
        lexer.read_char();
        return lexer;
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

    pub fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.bytes().nth(self.read_position).unwrap()
        }
    }

    pub fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        return &(self.input)[position..self.position]
    }

    pub fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        return &(self.input)[position..self.position]
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match String::from_utf8(vec![self.ch]){
            Err(why) => panic!("{:?}", why),
            Ok(literal) => {
                match self.ch {
                    b'=' => {
                        if self.peek_char() == b'=' {
                            self.read_char();
                            Token::from_str(TokenType::EQ, &(self.input)[self.position-1..=self.position])
                        } else {
                            Token::new(TokenType::ASSIGN, literal)
                        }
                    }
                    b'+' => Token::new(TokenType::PLUS, literal),
                    b',' => Token::new(TokenType::COMMA, literal),
                    b';' => Token::new(TokenType::SEMICOLON, literal),
                    b'(' => Token::new(TokenType::LPAREN, literal),
                    b')' => Token::new(TokenType::RPAREN, literal),
                    b'{' => Token::new(TokenType::LBRACE, literal),
                    b'}' => Token::new(TokenType::RBRACE, literal),
                    b'-' => Token::new(TokenType::MINUS, literal),
                    b'!' => {
                        if self.peek_char() == b'=' {
                            self.read_char();
                            Token::from_str(TokenType::NOT_EQ, &(self.input)[self.position-1..=self.position])
                        } else {
                            Token::new(TokenType::BANG, literal)
                        }
                    }
                    b'*' => Token::new(TokenType::ASTERISK, literal),
                    b'/' => Token::new(TokenType::SLASH, literal),
                    b'<' => Token::new(TokenType::LT, literal),
                    b'>' => Token::new(TokenType::GT, literal),
                    0 => Token::from_str(TokenType::EOF, ""),
                    _ => {
                        if is_letter(self.ch) {
                            let keyword_mutch = token::KeywordMatch::new(); //TODO メンバ変数にしようとしたが所有権が邪魔でできない
                            let ident = self.read_identifier();
                            return Token::from_str(keyword_mutch.lookup_ident(ident), ident);
                        } else if is_digit(self.ch) {
                            return Token::from_str(TokenType::INT, self.read_number());
                        } else {
                            Token::new(TokenType::ILLEGAL, literal)
                        }
                    },
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
        let input = String::from(r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
"#);
        let tests: [Token; 74] = [
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "five"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "ten"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "add"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::FUNCTION, "fn"),
            Token::from_str(TokenType::LPAREN, "("),
            Token::from_str(TokenType::IDENT, "x"),
            Token::from_str(TokenType::COMMA, ","),
            Token::from_str(TokenType::IDENT, "y"),
            Token::from_str(TokenType::RPAREN, ")"),
            Token::from_str(TokenType::LBRACE, "{"),
            Token::from_str(TokenType::IDENT, "x"),
            Token::from_str(TokenType::PLUS, "+"),
            Token::from_str(TokenType::IDENT, "y"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::RBRACE, "}"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::LET, "let"),
            Token::from_str(TokenType::IDENT, "result"),
            Token::from_str(TokenType::ASSIGN, "="),
            Token::from_str(TokenType::IDENT, "add"),
            Token::from_str(TokenType::LPAREN, "("),
            Token::from_str(TokenType::IDENT, "five"),
            Token::from_str(TokenType::COMMA, ","),
            Token::from_str(TokenType::IDENT, "ten"),
            Token::from_str(TokenType::RPAREN, ")"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::BANG, "!"),
            Token::from_str(TokenType::MINUS, "-"),
            Token::from_str(TokenType::SLASH, "/"),
            Token::from_str(TokenType::ASTERISK, "*"),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::LT, "<"),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::GT, ">"),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::IF, "if"),
            Token::from_str(TokenType::LPAREN, "("),
            Token::from_str(TokenType::INT, "5"),
            Token::from_str(TokenType::LT, "<"),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::RPAREN, ")"),
            Token::from_str(TokenType::LBRACE, "{"),
            Token::from_str(TokenType::RETURN, "return"),
            Token::from_str(TokenType::TRUE, "true"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::RBRACE, "}"),
            Token::from_str(TokenType::ELSE, "else"),
            Token::from_str(TokenType::LBRACE, "{"),
            Token::from_str(TokenType::RETURN, "return"),
            Token::from_str(TokenType::FALSE, "false"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::RBRACE, "}"),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::EQ, "=="),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::INT, "10"),
            Token::from_str(TokenType::NOT_EQ, "!="),
            Token::from_str(TokenType::INT, "9"),
            Token::from_str(TokenType::SEMICOLON, ";"),
            Token::from_str(TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        for expected in &tests {
            let tok: Token = l.next_token();
            assert_eq!(tok, *expected);
        }
    }
}
