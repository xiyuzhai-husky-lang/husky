use super::LnMirExprIdx;
use lean_opr::{
    opr::{binary::LnBinaryOpr, prefix::LnPrefixOpr, suffix::LnSuffixOpr},
    precedence::LnPrecedence,
};
use lean_term::instantiation::{
    menu::{ln_instantiation_menu, LnInstantiationMenu},
    LnInstantiation,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LnMirFunc {
    PrefixOpr {
        opr: LnPrefixOpr,
        instantiation: LnInstantiation,
    },
    BinaryOpr {
        opr: LnBinaryOpr,
        instantiation: LnInstantiation,
    },
    SuffixOpr {
        opr: LnSuffixOpr,
        instantiation: LnInstantiation,
    },
    Expr(LnMirExprIdx),
}

impl LnMirFunc {
    pub fn expr(self) -> Option<LnMirExprIdx> {
        match self {
            LnMirFunc::Expr(expr) => Some(expr),
            _ => None,
        }
    }

    pub(crate) fn outer_precedence(self) -> LnPrecedence {
        match self {
            LnMirFunc::PrefixOpr { opr, instantiation } => opr.outer_precedence(),
            LnMirFunc::BinaryOpr { opr, instantiation } => opr.outer_precedence(),
            LnMirFunc::SuffixOpr { opr, instantiation } => opr.outer_precedence(),
            LnMirFunc::Expr(expr) => LnPrecedence::Application,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LnMirFuncKey {
    PrefixOpr {
        opr: LnPrefixOpr,
        instantiation: LnInstantiation,
    },
    BinaryOpr {
        opr: LnBinaryOpr,
        instantiation: LnInstantiation,
    },
    SuffixOpr {
        opr: LnSuffixOpr,
        instantiation: LnInstantiation,
    },
}

impl From<LnMirFuncKey> for LnMirFunc {
    fn from(value: LnMirFuncKey) -> Self {
        match value {
            LnMirFuncKey::BinaryOpr { opr, instantiation } => {
                LnMirFunc::BinaryOpr { opr, instantiation }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LnMirFuncKeyMenu {
    pub nat_add: LnMirFuncKey,
    pub int_add: LnMirFuncKey,
    pub rat_add: LnMirFuncKey,
    pub real_add: LnMirFuncKey,
    pub complex_add: LnMirFuncKey,
    pub nat_mul: LnMirFuncKey,
    pub int_mul: LnMirFuncKey,
    pub rat_mul: LnMirFuncKey,
    pub real_mul: LnMirFuncKey,
    pub complex_mul: LnMirFuncKey,
    pub nat_eq: LnMirFuncKey,
    pub int_eq: LnMirFuncKey,
    pub rat_eq: LnMirFuncKey,
    pub real_eq: LnMirFuncKey,
    pub complex_eq: LnMirFuncKey,
    pub nat_le: LnMirFuncKey,
    pub int_le: LnMirFuncKey,
    pub rat_le: LnMirFuncKey,
    pub real_le: LnMirFuncKey,
    pub nat_ge: LnMirFuncKey,
    pub int_ge: LnMirFuncKey,
    pub rat_ge: LnMirFuncKey,
    pub real_ge: LnMirFuncKey,
}

impl LnMirFuncKeyMenu {
    pub fn new(db: &::salsa::Db) -> Self {
        use LnBinaryOpr::*;

        let LnInstantiationMenu {
            nat_add,
            int_add,
            rat_add,
            real_add,
            complex_add,
            nat_mul,
            int_mul,
            rat_mul,
            real_mul,
            complex_mul,
            nat_eq,
            int_eq,
            rat_eq,
            real_eq,
            complex_eq,
            nat_le,
            int_le,
            rat_le,
            real_le,
            nat_ge,
            int_ge,
            rat_ge,
            real_ge,
        } = *ln_instantiation_menu(db);
        let b = |opr, instantiation| LnMirFuncKey::BinaryOpr { opr, instantiation };
        Self {
            nat_add: b(Add, nat_add),
            int_add: b(Add, int_add),
            rat_add: b(Add, rat_add),
            real_add: b(Add, real_add),
            complex_add: b(Add, complex_add),
            nat_mul: b(Mul, nat_mul),
            int_mul: b(Mul, int_mul),
            rat_mul: b(Mul, rat_mul),
            real_mul: b(Mul, real_mul),
            complex_mul: b(Mul, complex_mul),
            nat_eq: b(Eq, nat_eq),
            int_eq: b(Eq, int_eq),
            rat_eq: b(Eq, rat_eq),
            real_eq: b(Eq, real_eq),
            complex_eq: b(Eq, complex_eq),
            nat_le: b(Le, nat_le),
            int_le: b(Le, int_le),
            rat_le: b(Le, rat_le),
            real_le: b(Le, real_le),
            nat_ge: b(Ge, nat_ge),
            int_ge: b(Ge, int_ge),
            rat_ge: b(Ge, rat_ge),
            real_ge: b(Ge, real_ge),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn ln_mir_func_key_menu(db: &::salsa::Db) -> LnMirFuncKeyMenu {
    LnMirFuncKeyMenu::new(db)
}
