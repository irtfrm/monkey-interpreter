use std::collections::HashMap;
use std::fmt;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    
    // identifiers & literals
    IDENT,
    INT,

    // operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    EQ,
    NOT_EQ,
    LT,
    GT,

    // delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    RETURN,
    LET,
    IF,
    ELSE,
    TRUE,
    FALSE,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_name: &str = match *self {
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::EOF => "EOF",
            TokenType::IDENT => "IDENT",
            TokenType::INT => "INT",
            TokenType::ASSIGN => "=",
            TokenType::PLUS => "+",
            TokenType::COMMA => ",",
            TokenType::SEMICOLON => ";",
            TokenType::LPAREN => "(",
            TokenType::RPAREN => ")",
            TokenType::LBRACE => "{",
            TokenType::RBRACE => "}",
            TokenType::FUNCTION => "FUNCTION",
            TokenType::LET => "LET",
            TokenType::MINUS => "-",
            TokenType::BANG => "!",
            TokenType::ASTERISK => "*",
            TokenType::SLASH => "/",
            TokenType::EQ => "==",
            TokenType::NOT_EQ => "!=",
            TokenType::LT => "<",
            TokenType::GT => ">",
            TokenType::RETURN => "RETURN",
            TokenType::IF => "IF",
            TokenType::ELSE => "ELSE",
            TokenType::TRUE => "TRUE",
            TokenType::FALSE => "FALSE",
        };
        write!(f, "{}", token_name)
    }
}

pub struct KeywordMatch {
    map: HashMap<String, TokenType>,
}

impl KeywordMatch {
    pub fn new() -> KeywordMatch {
        let mut map = HashMap::new();
        
        map.insert("fn".to_string(), TokenType::FUNCTION);
        map.insert("let".to_string(), TokenType::LET);
        map.insert("return".to_string(), TokenType::RETURN);
        map.insert("if".to_string(), TokenType::IF);
        map.insert("else".to_string(), TokenType::ELSE);
        map.insert("true".to_string(), TokenType::TRUE);
        map.insert("false".to_string(), TokenType::FALSE);
        return KeywordMatch{map: map};
    }

    pub fn lookup_ident(& self, ident: &str) -> TokenType {
        match self.map.get(ident) {
            Some(tok) => *tok,
            None => TokenType::IDENT,
        }
    }
}

#[derive(Debug,PartialEq,Eq)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal
        }
    }
    pub fn from_str(token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type,
            literal: literal.to_string()
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type:{} Literal:{}", self.token_type, self.literal)
    }
}