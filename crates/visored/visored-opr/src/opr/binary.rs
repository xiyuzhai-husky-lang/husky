use crate::precedence::{VdPrecedence, VdPrecedenceRange};
use eterned::db::EternerDb;
use lisp_csv::expr::{LpCsvExpr, LpCsvExprData};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdBaseBinaryOpr {
    /// a-b
    Sub,
    /// a/b
    Div,
}

impl VdBaseBinaryOpr {
    pub const SUB: Self = VdBaseBinaryOpr::Sub;
    pub const DIV: Self = VdBaseBinaryOpr::Div;
}

impl VdBaseBinaryOpr {
    pub fn from_lp_csv_expr(expr: &LpCsvExpr, db: &EternerDb) -> Self {
        let LpCsvExprData::Ident(ref ident) = expr.data else {
            todo!()
        };
        match ident.as_str() {
            "sub" => VdBaseBinaryOpr::Sub,
            "div" => VdBaseBinaryOpr::Div,
            _ => todo!(),
        }
    }
}

impl VdBaseBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        match self {
            VdBaseBinaryOpr::Sub => todo!(),
            VdBaseBinaryOpr::Div => todo!(),
        }
    }

    pub fn left_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdBaseBinaryOpr::Sub => VdPrecedenceRange::ADD_SUB_LEFT,
            VdBaseBinaryOpr::Div => VdPrecedenceRange::MUL_DIV_LEFT,
        }
    }

    pub fn right_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdBaseBinaryOpr::Sub => VdPrecedenceRange::ADD_SUB_RIGHT,
            VdBaseBinaryOpr::Div => VdPrecedenceRange::MUL_DIV_RIGHT,
        }
    }

    pub fn latex_code(self) -> &'static str {
        match self {
            VdBaseBinaryOpr::Sub => "-",
            VdBaseBinaryOpr::Div => "/",
        }
    }

    pub fn unicode(self) -> &'static str {
        match self {
            VdBaseBinaryOpr::Sub => "−",
            VdBaseBinaryOpr::Div => "÷",
        }
    }

    pub fn precedence(self) -> VdPrecedence {
        match self {
            VdBaseBinaryOpr::Sub => VdPrecedence::ADD_SUB,
            VdBaseBinaryOpr::Div => VdPrecedence::MUL_DIV,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VdCompositeBinaryOpr {}

impl VdCompositeBinaryOpr {
    pub fn fmt_str(self) -> &'static str {
        match self {}
    }

    pub fn left_precedence_range(self) -> VdPrecedenceRange {
        todo!()
    }

    pub fn right_precedence_range(self) -> VdPrecedenceRange {
        todo!()
    }

    pub fn latex_code(&self) -> &str {
        match self {
            _ => todo!(),
        }
    }

    pub fn precedence(self) -> VdPrecedence {
        match self {}
    }
}
