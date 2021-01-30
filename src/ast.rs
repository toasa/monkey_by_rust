use crate::token;

enum Node {
    Program(Program),
    Stmt(Stmt),
    Expr(Expr),
}

pub enum Stmt {
    Let(Let),
    Return(Return),
}

pub enum Expr {
    Ident(Ident),
}

pub struct Program {
    pub stmts: Vec<Stmt>,
}

pub struct Let {
    pub token: token::Token,
    pub name: Ident,
    // val: Expr,
}

pub struct Return {
    pub token: token::Token,
    // pub val: Expr,
}

pub struct Ident {
    pub token: token::Token,
    pub val: String,
}