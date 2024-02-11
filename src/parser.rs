use std::collections::HashMap;

use crate::{
    component::{Component, HtmlTag},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug, Default)]
pub struct Parser {
    lx: Lexer,
    pub current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    block_depth: i32,
    known_components: HashMap<HtmlTag, Component>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            lx: l,
            ..Default::default()
        };
        p.next_token();
        p.next_token(); // calling twice so that cur and peek both are filled up;
        p
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lx.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Component> {
        self.parse_component()
    }

    fn parse_component(&mut self) -> Option<Component> {
        match self.current_token.tokentype {
            TokenType::IDENT => {
                let tag_name = self.current_token.literal.clone();
                let mut component = Component::new(Component::tag_from_string(&tag_name));
                // Parse the children of the component
                self.next_if_peek_is(TokenType::LBRACE); // Move to the opening curly brace
                self.block_depth += 1;
                self.next_token(); // Move to the first child or closing curly brace

                // if there are key value pairs parse them
                component.info = self.parse_component_info();

                if self.cur_token_is(TokenType::RBRACE) {
                    self.block_depth -= 1;
                }
                while !(self.block_depth == 0 || self.cur_token_is(TokenType::EOF))
                    && !self.cur_token_is(TokenType::ILLEGAL)
                {
                    while self.cur_token_is(TokenType::RBRACE) {
                        self.block_depth -= 1;
                        self.next_token();
                    }

                    if let Some(child) = self.parse_child() {
                        component.children.push(child);
                    }
                }
                self.known_components
                    .insert(Component::tag_from_string(&tag_name), component.clone());
                Some(component)
            }
            _ => None, // Invalid token for a component
        }
    }

    fn parse_child(&mut self) -> Option<Component> {
        match self.current_token.tokentype {
            TokenType::STRING => {
                let text_content = self.current_token.literal.clone();

                let text_component = Component::new_text(text_content);
                self.next_token(); // Move to the next token after the text content
                Some(text_component)
            }
            TokenType::IDENT => self.parse_component(), // Parse a nested component
            TokenType::EOF => {
                if self.block_depth != 0 {
                    self.errors
                        .push("All the curly braces need to be closed".to_string());
                }
                None
            }
            _ => None, // Invalid token for a child
        }
    }

    fn parse_component_info(&mut self) -> HashMap<String, Value> {
        let mut vec_of_info = HashMap::<String, Value>::new();
        while self.peek_token_is(TokenType::COLON) {
            let key = self.current_token.literal.clone();
            self.next_token(); // move to the colon
            self.next_token(); // move to the starting of the value for the key

            // for now it can be a string or an list of strings
            let value: Value = match self.current_token.tokentype {
                TokenType::STRING => {
                    let v = Value::STRING(self.current_token.literal.clone());
                    self.next_token(); // skip the COMMA
                    self.next_token();
                    v
                }
                TokenType::NUMBER => {
                    let v = Value::NUMBER(self.current_token.literal.parse::<i32>().unwrap());
                    self.next_token();
                    self.next_token();
                    v
                }
                TokenType::LSQBRACE => {
                    self.next_token();
                    let vec_value = match self.current_token.tokentype {
                        TokenType::STRING => {
                            let mut vec_of_s = Vec::<String>::new();
                            while !self.cur_token_is(TokenType::RSQBRACE) {
                                if self.cur_token_is(TokenType::STRING) {
                                    vec_of_s.push(self.current_token.literal.clone());
                                    self.next_token();
                                } else {
                                    self.errors.push(
                                        "Can not push anything but string in this list".to_string(),
                                    );
                                }
                            }

                            Value::VECOFSTRING(vec_of_s)
                        }
                        TokenType::NUMBER => {
                            let mut vec_of_i = Vec::<i32>::new();
                            while !self.cur_token_is(TokenType::RSQBRACE) {
                                if self.cur_token_is(TokenType::STRING) {
                                    vec_of_i
                                        .push(self.current_token.literal.parse::<i32>().unwrap());
                                    self.next_token();
                                } else {
                                    self.errors.push(
                                        "Can not push anything but string in this list".to_string(),
                                    );
                                }
                            }

                            Value::VECOFNUMBER(vec_of_i)
                        }
                        _ => Value::VECOFSTRING(Vec::<String>::new()),
                    };
                    self.next_token(); // skip the ']'
                    self.next_token();
                    vec_value
                }
                _ => Value::STRING("".to_string()),
            };

            vec_of_info.insert(key, value);
        }
        vec_of_info
    }

    fn cur_token_is(&self, tokentype: TokenType) -> bool {
        self.current_token.tokentype == tokentype
    }

    fn peek_token_is(&self, tokentype: TokenType) -> bool {
        self.peek_token.tokentype == tokentype
    }

    fn next_if_peek_is(&mut self, tokentype: TokenType) -> bool {
        if self.peek_token_is(tokentype.clone()) {
            self.next_token();
            true
        } else {
            self.peek_error(tokentype.clone());
            false
        }
    }

    fn peek_error(&mut self, tokentype: TokenType) {
        let found_token = self.peek_token.clone();
        self.errors.push(format!(
            "Expected next token to be {:?}, found {:?} instead",
            tokentype, found_token
        ));
    }

    // pub fn load_default_components(&mut self) {
    //     self.known_components = Component::get_default_tags();
    // }
}

#[derive(Debug, Clone)]
pub enum Value {
    STRING(String),
    NUMBER(i32),
    VECOFNUMBER(Vec<i32>),
    VECOFSTRING(Vec<String>),
}
