use idx_arena::{ArenaIdx, ArenaIdxRange};

#[derive(Debug, PartialEq, Eq)]
pub enum LeanHirExprData {
    Lambda {
        parameters: (),
    },
    Application {
        function_and_arguments: LeanHirExprIdxRange,
    },
}

pub type LeanHirExprIdx = ArenaIdx<LeanHirExprData>;
pub type LeanHirExprIdxRange = ArenaIdxRange<LeanHirExprData>;

pub struct LeanHirLambdaParameter {
    ident: LeanIdent,
    ty: LeanHirExprIdx,
}
