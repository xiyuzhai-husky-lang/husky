mod lex;
mod syntax;

pub enum Literal {
    Integer(i32),
    Float(i32),
    Char(char),
}

pub enum Token {
    Literal(Literal),
    Opr(Opr),
}

pub enum Opr {
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    Binary(BinaryOpr),
}

pub enum PrefixOpr {
    Not,
}
pub enum SuffixOpr {
    Incr,
    Decr,
}
pub enum BinaryOpr {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Expr {
    Literal(Literal),
}

pub struct TokenIdxRange {
    start: usize,
    end: usize,
}

pub struct ExprEntry {
    token_idx_range: TokenIdxRange,
    expr: Expr,
}

pub type ExprArena = Vec<Option<ExprEntry>>;
