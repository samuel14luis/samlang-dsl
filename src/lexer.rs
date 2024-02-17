use crate::{
    test::lexer_test::token::Token,
    test::lexer_test::token::TokenType
};
use regex::Regex;

#[path = "token.rs"]
mod token;

pub struct Lexer {
    // Aquí irían los campos de la clase, source
    _source: String,
    _character: String,
    _read_position: i32,
    _position: i32,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer {
            _source: source.to_string(),
            _character: "".to_string(),
            _read_position: 0,
            _position: 0,
        };

        lexer.read_character(); // Inicializa el primer carácter
        
        return lexer;
    }

    pub fn next_token(&mut self) -> Token {

        let token: Token;

        if Regex::new(r"^=$").unwrap().is_match(&self._character) {
            token = Token { token_type: TokenType::ASSIGN, literal: self._character.clone() };
        } else if Regex::new(r"^\+$").unwrap().is_match(&self._character) {
            token = Token { token_type: TokenType::PLUS, literal: self._character.clone() };
        } else {
            token = Token { token_type: TokenType::ILLEGAL, literal: self._character.clone() };
        }

        self.read_character();

        return token;
    }

    fn read_character(&mut self) {
        // Aquí iría la lógica para leer un carácter
        if self._read_position >= self._source.chars().count() as i32 {
            self._character = "".to_string();
        } else {
            //show in console
            self._character = self._source.chars().nth(self._read_position as usize)
            .map(|c| c.to_string())
            .unwrap_or("".to_string());
        }

        self._position = self._read_position;
        self._read_position += 1;
    }
}
