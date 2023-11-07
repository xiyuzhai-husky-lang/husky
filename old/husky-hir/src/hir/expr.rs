mod literal;
mod pattern;
mod region;
mod stmt;

pub use literal::*;
pub use pattern::*;
pub use region::*;
pub use stmt::*;

use super::*;
use husky_opr::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum ExprHir {
    Literal(LiteralHir),
    EntityPath(ItemPath),
    InheritedSynSymbol,
    CurrentSynSymbol,
    FrameVarDecl,
    SelfType,
    SelfValue,
    BinaryOpn {
        lopd: ExprHirIdx,
        opr: SynBinaryOpr,
        ropd: ExprHirIdx,
    },
    Be {
        src: ExprHirIdx,
        target: BeVariableDeclPatternHir,
    },
    PrefixOpn {
        opr: SynPrefixOpr,
        opd: ExprHirIdx,
    },
    SuffixOpn {
        opd: ExprHirIdx,
        punctuation: SynSuffixOpr,
    },
    ApplicationOrFunctionCall {
        function: ExprHirIdx,
        argument: ExprHirIdx,
    },
    FunctionCall {
        function: ExprHirIdx,
        generic_arguments: Option<GenericArgumentListHir>,
        arguments: ExprHirIdxRange,
    },
    Field {
        self_expr: ExprHirIdx,
        ident_token: Ident,
    },
    MethodCall {
        self_expr: ExprHirIdx,
        ident_token: Ident,
        generic_arguments: Option<GenericArgumentListHir>,
        nonself_arguments: ExprHirIdxRange,
    },
    TemplateInstantiation {
        template: ExprHirIdx,
        generic_arguments: GenericArgumentListHir,
    },
    Application {
        function: ExprHirIdx,
        argument: ExprHirIdx,
    },
    NewTuple {
        items: ExprHirIdxRange,
    },
    NewBoxList {
        caller: Option<ExprHirIdx>,
        items: ExprHirIdxRange,
    },
    BoxColon {
        caller: Option<ExprHirIdx>,
    },
    Block {
        stmts: StmtHirIdxRange,
    },
}

pub struct GenericArgumentListHir {}

pub type ExprHirArena = Arena<ExprHir>;
pub type ExprHirIdx = ArenaIdx<ExprHir>;
pub type ExprHirIdxRange = ArenaIdxRange<ExprHir>;
