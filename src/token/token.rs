type TokenType = string

struct Token {
    token_type: TokenType,
    literal: string,
}

enum TOKEN_TYPES {
    ILLEGAL = "ILLEGAL",
    EOF = "EOF",
    
    // identifiers & literals
    IDENT = "IDENT",
    INT = "INT",

    // operators
    ASSIGN = ",",
    PLUS = "+",

    // delimiters
    COMMA = ",",
    SEMICOLON = ";",

    LPAREN = "(",
    RPAREN = ")",
    LBRACE = "{",
    RBRACE = "}",

    // keywords
    FUNCTION = "FUNCTION",
    LET = "LET",
}