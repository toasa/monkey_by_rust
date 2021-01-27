pub struct Token {
    pub t: Type,
    pub literal: String,
}

#[derive(PartialOrd, PartialEq, Debug)]
pub enum Type {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}
