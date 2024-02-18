use crate::{
    test::lexer_test::lexer::Lexer, 
    test::lexer_test::token::Token,
    test::lexer_test::token::TokenType
};

#[path = "../lexer.rs"]
mod lexer;

#[path = "../token.rs"]
mod token;

#[test]
fn test_illegal() {
    // Arrange
    let source = "¡¿@";
    let mut lexer: Lexer = Lexer::new(source);

    // Act
    // tokens es una variable que va a contener todos los tokens, es una lista de Token inicialmente vacía
    let mut tokens: Vec<Token> = vec![];
    // recorrer el vector
    for _i in 0..source.chars().count() {
        tokens.push(lexer.next_token());
    }

    // Assert
    // expected tokens
    let expected: Vec<Token> = vec![
        Token::new(TokenType::ILLEGAL, "¡".to_string()),
        Token::new(TokenType::ILLEGAL, "¿".to_string()),
        Token::new(TokenType::ILLEGAL, "@".to_string()),
    ];

    assert_eq!(expected, tokens);
}

#[test]
fn test_one_character_operator() {
    // Arrange
    let source = "=+";
    let mut lexer: Lexer = Lexer::new(source);

    // Act
    let mut tokens: Vec<Token> = vec![];
    // recorrer el vector
    for _i in 0..source.chars().count() {
        tokens.push(lexer.next_token());
    }

    // Assert
    // expected tokens
    let expected: Vec<Token> = vec![
        Token::new(TokenType::ASSIGN, "=".to_string()),
        Token::new(TokenType::PLUS, "+".to_string()),
    ];

    assert_eq!(expected, tokens);
}

#[test]
fn test_end_of_file_eof() {
    // Arrange
    let source = "+";
    let mut lexer: Lexer = Lexer::new(source);

    // Act
    let mut tokens: Vec<Token> = vec![];
    // recorrer el vector
    for _i in 0..source.chars().count() + 1 {
        tokens.push(lexer.next_token());
    }

    // Assert
    // expected tokens
    let expected: Vec<Token> = vec![
        Token::new(TokenType::PLUS, "+".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    assert_eq!(expected, tokens);
}

#[test]
fn test_delimiters() {
    // Arrange
    let source = "(){},;";
    let mut lexer: Lexer = Lexer::new(source);

    // Act
    let mut tokens: Vec<Token> = vec![];
    // recorrer el vector
    for _i in 0..source.chars().count() + 1 {
        tokens.push(lexer.next_token());
    }

    // Assert
    // expected tokens
    let expected: Vec<Token> = vec![
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::LBRACE, "{".to_string()),
        Token::new(TokenType::RBRACE, "}".to_string()),
        Token::new(TokenType::COMMA, ",".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    assert_eq!(expected, tokens);
}

#[test]
fn test_assignment() {
    // Arrange
    let source = "let five=3105;";
    let mut lexer: Lexer = Lexer::new(source);

    // Act
    let mut tokens: Vec<Token> = vec![];

    // recorrer el vector
    while lexer.has_next() {
        tokens.push(lexer.next_token());
    }

    // Assert
    // expected tokens
    let expected: Vec<Token> = vec![
        Token::new(TokenType::LET, "let".to_string()),
        Token::new(TokenType::IDENT, "five".to_string()),
        Token::new(TokenType::ASSIGN, "=".to_string()),
        Token::new(TokenType::INT, "3105".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    assert_eq!(expected, tokens);
}