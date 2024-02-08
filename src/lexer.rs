use crate::token::{self, Token, TokenType};

#[derive(Debug, Default)]
pub struct Lexer {
    input: String,
    position: usize,
    next_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        let mut l = Lexer {
            input: code,
            ..Default::default()
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespaces();
        let tk = match self.ch {
            '{' => Token::new(TokenType::LBRACE, self.ch),
            '}' => Token::new(TokenType::RBRACE, self.ch),
            ':' => Token::new(TokenType::COLON, self.ch),
            '"' | '\'' => self.read_string_token(),
            ',' => Token::new(TokenType::COMMA, self.ch),
            '\0' => Token::new(TokenType::EOF, self.ch),
            _ => {
                if self.ch.is_alphabetic() {
                    let ident = self.read_identifier();
                    let mut tk = Token::new(Token::lookup_ident(&ident), '_');
                    tk.literal = ident;
                    return tk;
                } else if self.ch.is_numeric() {
                    // check if its a number
                    let mut tk = Token::new(TokenType::NUMBER, '_');
                    tk.literal = self.read_number();
                    return tk;
                } else {
                    return Token::new(TokenType::ILLEGAL, self.ch);
                }
            }
        };

        self.read_char();
        return tk;
    }

    fn read_number(&mut self) -> String {
        let position = self.position; // starting point
                                      // read the full identifier
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position; // starting point
                                      // read the full identifier
        while self.ch.is_alphanumeric() {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    fn read_char(&mut self) {
        self.ch = match self.input.chars().nth(self.next_position) {
            Some(c) => c,
            None => '\0',
        };
        self.position = self.next_position;
        self.next_position += 1;
    }

    fn read_string_token(&mut self) -> Token {
        let quote = self.ch;
        // println!("quote = {}", quote);
        self.read_char();
        let pos = self.position;
        // println!("current char: {}, pos: {}", self.ch, self.position);
        while self.ch != '\0' && self.ch != quote {
            self.read_char();
        }
        // println!("current char: {}, pos: {}", self.ch, self.position);
        // skip the last qoute
        if self.ch == '\0' {
            return Token::new(TokenType::EOF, self.ch);
        } else if self.ch == quote {
            self.read_char();
        }

        // println!("current char: {}, pos: {}", self.ch, self.position);
        let literal = self.input[pos..self.position - 1].to_string();
        // println!("String : {}", literal);
        let mut tk = Token::new(TokenType::STRING, '_');
        tk.literal = literal;

        // println!("current char: {:?}", tk);
        tk
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        }
    }
}
