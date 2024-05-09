use super::token::Token;

#[derive(Debug)]
pub struct Program {
    pub funcs: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Declaration(String, Option<Expression>),
    Return(Expression),
    Expression(Expression),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
}

#[derive(Debug)]
pub enum Expression {
    Num(u32),
    Var(String),
    UnOp(UnOp, Box<Expression>),
    BinOp(BinOp, Box<Expression>, Box<Expression>),
    Assign(String, Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum UnOp {
    Negation,
    BitNot,
    LogicNot,
}

#[derive(Debug)]
pub enum BinOp {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    LogicAnd,
    LogicOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
    Comma,
}

impl BinOp {
    fn from(token: Token) -> Result<Self, String> {
        match token {
            Token::Negation => Ok(BinOp::Subtraction),
            err => Err(format!("Cannot convert {:?} into BinOp", err)),
        }
    }
}

impl UnOp {
    fn from(token: Token) -> Result<Self, String> {
        match token {
            Token::Negation => Ok(UnOp::Negation),
            Token::BitNot => Ok(UnOp::BitNot),
            Token::LogicNot => Ok(UnOp::LogicNot),
            err => Err(format!("Cannot convert {:?} to UnOp, can only use: !, ~, -", err)),
        }
    }
}
