//! Abstract Syntax Tree definitions for Sprig

/// Type in the Sprig language
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Void,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// Expression nodes
#[derive(Debug, Clone)]
pub enum Expr {
    /// Integer literal
    IntLit(i64),
    /// Boolean literal
    BoolLit(bool),
    /// Variable reference
    Var(String),
    /// Binary operation
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Unary operation
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    /// Function call
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

/// Statement nodes
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Variable declaration with optional init
    Let {
        name: String,
        ty: Type,
        init: Option<Expr>,
    },
    /// Assignment
    Assign {
        name: String,
        value: Expr,
    },
    /// If statement
    If {
        cond: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    /// While loop
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    /// Return statement
    Return(Option<Expr>),
    /// Expression statement (for calls with side effects)
    Expr(Expr),
}

/// Function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

/// Function definition
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub return_ty: Type,
    pub body: Vec<Stmt>,
}

/// A complete program
#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}
