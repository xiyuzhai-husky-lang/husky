use husky_hir_opr::binary::HirBinaryOpr;
use husky_opr::{BinaryClosedOpr, BinaryComparisonOpr, BinaryShortcuitLogicOpr};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub(crate) enum RustPrecedence {
    Atom = 255,
    Suffix = 240,
    Prefix = 210,
    Power = 200,
    Multiplicative = 160,
    Additive = 150,
    Shift = 120,
    OrdComparison = 111,
    EqComparison = 110,
    BitAnd = 102,
    BitXor = 101,
    BitOr = 100,
    And = 90,
    Or = 80,
    As = 70,
    Is = 60,
    Be = 40,
    /// means `->`
    Curry = 30,
    KeyedArgument = 22,
    ListItem = 21,
    LambdaHead = 20,
    Range = 12,
    Assign = 11,
    List = 1,
    None = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RustPrecedenceRange {
    Geq(RustPrecedence),
    Greater(RustPrecedence),
    Any,
}

impl RustPrecedenceRange {
    pub(crate) fn include(self, precedence: RustPrecedence) -> bool {
        match self {
            RustPrecedenceRange::Geq(slf) => precedence >= slf,
            RustPrecedenceRange::Greater(slf) => precedence > slf,
            RustPrecedenceRange::Any => true,
        }
    }
}

pub(crate) fn any_precedence(opd: HirEagerExprIdx) -> (RustPrecedenceRange, HirEagerExprIdx) {
    (RustPrecedenceRange::Any, opd)
}

pub trait HasRustPrecedence {
    fn rust_precedence(&self) -> RustPrecedence;
}

pub(super) fn hir_eager_expr_precedence(data: &HirEagerExprData) -> RustPrecedence {
    match data {
        HirEagerExprData::Literal(_)
        | HirEagerExprData::PrincipalEntityPath(_)
        | HirEagerExprData::ConstSymbol(_)
        | HirEagerExprData::Variable(_)
        | HirEagerExprData::NewTuple { .. }
        | HirEagerExprData::NewList { .. }
        | HirEagerExprData::EmptyHtmlTag { .. }
        | HirEagerExprData::Todo
        | HirEagerExprData::AssociatedFn { .. } => RustPrecedence::Atom,
        HirEagerExprData::Binary { opr, .. } => match opr {
            HirBinaryOpr::Closed(opr) => match opr {
                BinaryClosedOpr::Add => RustPrecedence::Additive,
                BinaryClosedOpr::BitAnd => RustPrecedence::BitAnd,
                BinaryClosedOpr::BitOr => RustPrecedence::BitOr,
                BinaryClosedOpr::BitXor => RustPrecedence::BitXor,
                BinaryClosedOpr::Div => RustPrecedence::Multiplicative,
                BinaryClosedOpr::Mul => RustPrecedence::Multiplicative,
                BinaryClosedOpr::RemEuclid => RustPrecedence::Atom, // because it's implemented through function
                BinaryClosedOpr::Power => RustPrecedence::Atom, // because it's implemented through function
                BinaryClosedOpr::Sub => RustPrecedence::Additive,
            },
            HirBinaryOpr::Shift(_) => RustPrecedence::Shift,
            HirBinaryOpr::Assign | HirBinaryOpr::AssignClosed(_) | HirBinaryOpr::AssignShift(_) => {
                RustPrecedence::Assign
            }
            HirBinaryOpr::Comparison(opr) => match opr {
                BinaryComparisonOpr::Eq | BinaryComparisonOpr::Neq => RustPrecedence::EqComparison,
                BinaryComparisonOpr::Geq
                | BinaryComparisonOpr::Greater
                | BinaryComparisonOpr::Leq
                | BinaryComparisonOpr::Less => RustPrecedence::OrdComparison,
            },
            HirBinaryOpr::ShortCircuitLogic(opr) => match opr {
                BinaryShortcuitLogicOpr::And => RustPrecedence::And,
                BinaryShortcuitLogicOpr::Or => RustPrecedence::Or,
            },
            HirBinaryOpr::As => RustPrecedence::As,
        },
        HirEagerExprData::Be { .. } => RustPrecedence::Be,
        HirEagerExprData::Prefix { .. }
        | HirEagerExprData::Suffix { .. }
        | HirEagerExprData::MajorFunctionFnCall { .. }
        | HirEagerExprData::Field { .. }
        | HirEagerExprData::MethodFnCall { .. }
        | HirEagerExprData::Index { .. } => RustPrecedence::Suffix,
        HirEagerExprData::Block { .. } => RustPrecedence::None,
    }
}
