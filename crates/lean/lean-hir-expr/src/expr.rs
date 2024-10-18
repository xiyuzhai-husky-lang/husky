use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use lean_coword::ident::LeanIdent;
use lean_opr::{
    opr::{binary::LeanBinaryOpr, prefix::LeanPrefixOpr, suffix::LeanSuffixOpr},
    precedence::LeanPrecedence,
};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq)]
pub enum LeanHirExprData {
    Variable {
        ident: LeanIdent,
    },
    Prefix {
        opr: LeanPrefixOpr,
        opd: LeanHirExprIdx,
    },
    Suffix {
        opd: LeanHirExprIdx,
        opr: LeanSuffixOpr,
    },
    Binary {
        lopd: LeanHirExprIdx,
        opr: LeanBinaryOpr,
        ropd: LeanHirExprIdx,
    },
    Lambda {
        parameters: LeanHirLambdaParameters,
        body: LeanHirExprIdx,
    },
    Application {
        function_and_arguments: LeanHirExprIdxRange,
    },
}
impl LeanHirExprData {
    pub(crate) fn precedence(&self) -> LeanPrecedence {
        match self {
            LeanHirExprData::Variable { ident } => todo!(),
            LeanHirExprData::Prefix { opr, opd } => todo!(),
            LeanHirExprData::Suffix { opd, opr } => todo!(),
            LeanHirExprData::Binary { lopd, opr, ropd } => todo!(),
            LeanHirExprData::Lambda { parameters, body } => todo!(),
            LeanHirExprData::Application {
                function_and_arguments,
            } => todo!(),
        }
    }
}

pub type LeanHirExprIdx = ArenaIdx<LeanHirExprData>;
pub type LeanHirExprIdxRange = ArenaIdxRange<LeanHirExprData>;
pub type LeanHirExprArena = Arena<LeanHirExprData>;
pub type LeanHirExprArenaRef<'a> = ArenaRef<'a, LeanHirExprData>;

#[derive(Debug, PartialEq, Eq)]
pub struct LeanHirLambdaParameter {
    ident: LeanIdent,
    ty: LeanHirExprIdx,
}

impl LeanHirLambdaParameter {
    pub fn ident(&self) -> LeanIdent {
        self.ident
    }

    pub fn ty(&self) -> LeanHirExprIdx {
        self.ty
    }
}

pub type LeanHirLambdaParameters = SmallVec<[LeanHirLambdaParameter; 4]>;
