use crate::{
    test::lexer_test::lexer::Lexer, 
    test::lexer_test::token::Token,
    test::lexer_test::token::TokenType
};

use pretty_assertions::assert_eq;

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
    while lexer.has_next() {
        tokens.push(lexer.next_token());
    }

    // Assert
    // expected tokens
    let expected: Vec<Token> = vec![
        Token::new(TokenType::ILLEGAL, "¡".to_string()),
        Token::new(TokenType::ILLEGAL, "¿".to_string()),
        Token::new(TokenType::ILLEGAL, "@".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    assert_eq!(expected, tokens);
}

#[test]
fn test_one_character_operator() {
    // Arrange
    let source = "=+-/*<>!";
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
        Token::new(TokenType::ASSIGN, "=".to_string()),
        Token::new(TokenType::PLUS, "+".to_string()),
        Token::new(TokenType::MINUS, "-".to_string()),
        Token::new(TokenType::DIVISION, "/".to_string()),
        Token::new(TokenType::MULTIPLICATION, "*".to_string()),
        Token::new(TokenType::LT, "<".to_string()),
        Token::new(TokenType::GT, ">".to_string()),
        Token::new(TokenType::NEGATION, "!".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
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
    while lexer.has_next() {
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
    while lexer.has_next() {
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
    let source = "let five=     3105;";
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

#[test]
fn test_function_declaration() {
    // Arrange
    let source = "variable suma = función(x, y) { x + y };";
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
        Token::new(TokenType::LET, "variable".to_string()),
        Token::new(TokenType::IDENT, "suma".to_string()),
        Token::new(TokenType::ASSIGN, "=".to_string()),
        Token::new(TokenType::FUNCTION, "función".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENT, "x".to_string()),
        Token::new(TokenType::COMMA, ",".to_string()),
        Token::new(TokenType::IDENT, "y".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::LBRACE, "{".to_string()),
        Token::new(TokenType::IDENT, "x".to_string()),
        Token::new(TokenType::PLUS, "+".to_string()),
        Token::new(TokenType::IDENT, "y".to_string()),
        Token::new(TokenType::RBRACE, "}".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    assert_eq!(expected, tokens);
}

#[test]
fn test_function_call() {
    // Arrange
    let source = "variable resultado = suma(dos, tres);";
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
        Token::new(TokenType::LET, "variable".to_string()),
        Token::new(TokenType::IDENT, "resultado".to_string()),
        Token::new(TokenType::ASSIGN, "=".to_string()),
        Token::new(TokenType::IDENT, "suma".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::IDENT, "dos".to_string()),
        Token::new(TokenType::COMMA, ",".to_string()),
        Token::new(TokenType::IDENT, "tres".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    assert_eq!(expected, tokens);
}

#[test]
fn test_control_statement() {
    // Arrange
    let source = "si (5 < 10) { retorna verdadero; } sino { retorna falso; }";
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
        Token::new(TokenType::IF, "si".to_string()),
        Token::new(TokenType::LPAREN, "(".to_string()),
        Token::new(TokenType::INT, "5".to_string()),
        Token::new(TokenType::LT, "<".to_string()),
        Token::new(TokenType::INT, "10".to_string()),
        Token::new(TokenType::RPAREN, ")".to_string()),
        Token::new(TokenType::LBRACE, "{".to_string()),
        Token::new(TokenType::RETURN, "retorna".to_string()),
        Token::new(TokenType::TRUE, "verdadero".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        Token::new(TokenType::RBRACE, "}".to_string()),
        Token::new(TokenType::ELSE, "sino".to_string()),
        Token::new(TokenType::LBRACE, "{".to_string()),
        Token::new(TokenType::RETURN, "retorna".to_string()),
        Token::new(TokenType::FALSE, "falso".to_string()),
        Token::new(TokenType::SEMICOLON, ";".to_string()),
        Token::new(TokenType::RBRACE, "}".to_string()),
        Token::new(TokenType::EOF, "".to_string()),
    ];

    assert_eq!(expected, tokens);
}