use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use lean_coword::ident::LnIdent;
use lean_opr::{
    opr::{binary::LnBinaryOpr, prefix::LnPrefixOpr, suffix::LnSuffixOpr},
    precedence::LnPrecedence,
};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq)]
pub enum LnHirExprData {
    Variable {
        ident: LnIdent,
    },
    Prefix {
        opr: LnPrefixOpr,
        opd: LnHirExprIdx,
    },
    Suffix {
        opd: LnHirExprIdx,
        opr: LnSuffixOpr,
    },
    Binary {
        lopd: LnHirExprIdx,
        opr: LnBinaryOpr,
        ropd: LnHirExprIdx,
    },
    Lambda {
        parameters: LnHirLambdaParameters,
        body: LnHirExprIdx,
    },
    Application {
        function_and_arguments: LnHirExprIdxRange,
    },
}

pub type LnHirExprIdx = ArenaIdx<LnHirExprData>;
pub type LnHirExprIdxRange = ArenaIdxRange<LnHirExprData>;
pub type LnHirExprArena = Arena<LnHirExprData>;
pub type LnHirExprArenaRef<'a> = ArenaRef<'a, LnHirExprData>;

impl LnHirExprData {
    pub(crate) fn precedence(&self) -> LnPrecedence {
        match self {
            LnHirExprData::Variable { ident } => todo!(),
            LnHirExprData::Prefix { opr, opd } => todo!(),
            LnHirExprData::Suffix { opd, opr } => todo!(),
            LnHirExprData::Binary { lopd, opr, ropd } => todo!(),
            LnHirExprData::Lambda { parameters, body } => todo!(),
            LnHirExprData::Application {
                function_and_arguments,
            } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LnHirLambdaParameter {
    ident: LnIdent,
    ty: LnHirExprIdx,
}

impl LnHirLambdaParameter {
    pub fn ident(&self) -> LnIdent {
        self.ident
    }

    pub fn ty(&self) -> LnHirExprIdx {
        self.ty
    }
}

pub type LnHirLambdaParameters = SmallVec<[LnHirLambdaParameter; 4]>;
