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
        Token::new(TokenType::ILLEGAL, '¡'),
        Token::new(TokenType::ILLEGAL, '¿'),
        Token::new(TokenType::ILLEGAL, '@'),
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
        Token::new(TokenType::ASSIGN, '='),
        Token::new(TokenType::PLUS, '+'),
    ];

    assert_eq!(expected, tokens);
}