use crate::{
    test::lexer_test::token::Token,
    test::lexer_test::token::TokenType
};
use regex::Regex;
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[path = "token.rs"]
mod token;

static PATTERNS: Lazy<Vec<(&'static str, TokenType)>> = Lazy::new(|| {
    vec![
        (r"^=$", TokenType::ASSIGN),
        (r"^\+$", TokenType::PLUS),
        (r"^\($", TokenType::LPAREN),
        (r"^\)$", TokenType::RPAREN),
        (r"^\{$", TokenType::LBRACE),
        (r"^\}$", TokenType::RBRACE),
        (r"^\,$", TokenType::COMMA),
        (r"^\;$", TokenType::SEMICOLON),
        (r"^$", TokenType::EOF),
    ]
});

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("fn", TokenType::FUNCTION);
    m.insert("let", TokenType::LET);
    m.insert("var", TokenType::LET);
    m.insert("variable", TokenType::LET);
    m.insert("true", TokenType::TRUE);
    m.insert("false", TokenType::FALSE);
    m.insert("if", TokenType::IF);
    m.insert("else", TokenType::ELSE);
    m.insert("return", TokenType::RETURN);
    return m;
});

pub struct Lexer {
    // Aquí irían los campos de la clase, source
    _source: String,
    _character: String,
    _read_position: i32,
    _position: i32,
    _has_next: bool,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer {
            _source: source.to_string(),
            _character: "".to_string(),
            _read_position: 0,
            _position: 0,
            _has_next: true,
        };

        lexer.read_character(); // Inicializa el primer carácter
        
        return lexer;
    }

    pub fn has_next(&self) -> bool {
        return self._has_next;
    }

    pub fn next_token(&mut self) -> Token {
        self._skip_whitespace();

        let mut token: Token = Token { token_type: TokenType::ILLEGAL, literal: self._character.clone() };
        let _char = self._character.clone();
        
        if self._is_letter(&_char) {
            let literal = self._read_identifier();
            let token_type = self._lookup_token_type(&literal);
            token = Token { token_type: token_type, literal: literal };
        } else if self._is_number(&_char) {
            let literal = self._read_number();
            token = Token { token_type: TokenType::INT, literal: literal };
        } else {
            for (pattern, token_type) in PATTERNS.iter() {
                if Regex::new(pattern).unwrap().is_match(&_char) {
                    token = Token { token_type: token_type.clone(), literal: _char };
                    break;
                }
            }
        }

        self.read_character(); // OJO: Siempre revisar que no esté eliminando chars

        self._has_next = token.token_type != TokenType::EOF;

        return token;
    }

    fn _is_letter(&mut self, character: &str) -> bool {
        let re = Regex::new(r"^[a-záéíóúA-ZÁÉÍÓÚñÑ_]$").unwrap();
        return re.is_match(character);
    }

    fn _is_number(&mut self, character: &str) -> bool {
        let re = Regex::new(r"^\d$").unwrap();
        return re.is_match(character);
    }

    fn _read_identifier(&mut self) -> String {
        let initial_position = self._position;
        let mut character = self._character.clone();

        while self._is_letter(&character) {
            self.read_character();
            character = self._character.clone();
        }

        let final_position = self._position;
        self._undo_read_character();
        return self._source.chars().clone().skip(initial_position as usize).take((final_position - initial_position) as usize).collect();
    }

    fn _lookup_token_type(&mut self, literal: &str) -> TokenType {
        let default_token_type = KEYWORDS.get(literal).unwrap_or(&TokenType::IDENT);
        return default_token_type.clone();
    }
    
    fn read_character(&mut self) {
        let rp = self._read_position;
        // Aquí iría la lógica para leer un carácter
        if rp >= self._source.chars().count() as i32 {
            self._character = "".to_string();
        } else {
            //show in console
            self._character = self._source.chars().nth(rp as usize)
            .map(|c| c.to_string())
            .unwrap_or("".to_string());
        }

        self._position = rp;
        self._read_position += 1;
    }

    fn _undo_read_character(&mut self) {
        self._read_position -= 1;
        self._position -= 1;
    }

    fn _read_number(&mut self) -> String {
        let initial_position = self._position.clone();
        let mut character = self._character.clone();

        while self._is_number(&character) {
            self.read_character();
            character = self._character.clone();
        }

        let final_position = self._position.clone();
        self._undo_read_character();
        return self._source.chars().clone().skip(initial_position as usize).take((final_position - initial_position) as usize).collect();
    }

    fn _skip_whitespace(&mut self) {
        let re = Regex::new(r"^[\s\t]$").unwrap();
        while re.is_match(&self._character) {
            self.read_character();            
        }
    }
}
