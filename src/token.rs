// definir clase Token que tiene un tipo de token y una literal (su valor en string)
#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, character: char) -> Token {
        Token {
            token_type: token_type,
            literal: character.to_string(),
        }
    }
}

// definir un enum TokenType que tiene los tipos de tokens que se pueden encontrar
#[derive(Debug, PartialEq)]
pub enum TokenType {
    ASSIGN = 1,
    COMMA = 2,
    EOF = 3,
    FUNCTION = 4,
    IDENT = 5,
    ILLEGAL = 6,
    INT = 7,
    LBRACE = 8,
    LET = 9,
    LPAREN = 10,
    RBRACE = 11,
    RPAREN = 12,
    PLUS = 13,
    MINUS = 14,
    SEMICOLON = 15,
    BANG = 16,
    SLASH = 17,
    ASTERISK = 18,
    LT = 19,
    GT = 20,
    EQ = 21,
    NOT_EQ = 22,
    TRUE = 23,
    FALSE = 24,
    IF = 25,
    ELSE = 26,
    RETURN = 27,
}