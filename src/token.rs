#[derive(Debug, Default, PartialEq, Clone)]
pub enum TokenType {
    // Special tokens
    #[default]
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    NUMBER,
    STRING,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,

    // Delimiters
    COMMA,
    SEMICOLON,
    COLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LSQBRACE,
    RSQBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub literal: String, // using String instead of char for consistency
}

impl Token {
    // Implement a function similar to NewToken in Go
    pub fn new(tokentype: TokenType, literal: char) -> Token {
        Token {
            tokentype,
            literal: literal.to_string(),
        }
    }

    // Implement a function similar to LookupIdent in Go
    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        }
    }
}
