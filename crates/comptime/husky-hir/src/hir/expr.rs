mod literal;
mod pattern;
mod region;
mod stmt;

pub use literal::*;
pub use pattern::*;
pub use region::*;
pub use stmt::*;

use super::*;
use husky_opn_syntax::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub enum ExprHir {
    Literal(LiteralHir),
    EntityPath(EntityPath),
    InheritedSymbol,
    CurrentSymbol,
    FrameVarDecl,
    SelfType,
    SelfValue,
    BinaryOpn {
        lopd: ExprHirIdx,
        opr: BinaryOpr,
        ropd: ExprHirIdx,
    },
    Be {
        src: ExprHirIdx,
        target: BeVariableDeclPatternHir,
    },
    PrefixOpn {
        opr: PrefixOpr,
        opd: ExprHirIdx,
    },
    SuffixOpn {
        opd: ExprHirIdx,
        punctuation: SuffixOpr,
    },
    ApplicationOrFunctionCall {
        function: ExprHirIdx,
        argument: ExprHirIdx,
    },
    FunctionCall {
        function: ExprHirIdx,
        implicit_arguments: Option<ImplicitArgumentListHir>,
        arguments: ExprHirIdxRange,
    },
    Field {
        self_expr: ExprHirIdx,
        ident_token: Identifier,
    },
    MethodCall {
        self_expr: ExprHirIdx,
        ident_token: Identifier,
        implicit_arguments: Option<ImplicitArgumentListHir>,
        nonself_arguments: ExprHirIdxRange,
    },
    TemplateInstantiation {
        template: ExprHirIdx,
        implicit_arguments: ImplicitArgumentListHir,
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

pub struct ImplicitArgumentListHir {}

pub type ExprHirArena = Arena<ExprHir>;
pub type ExprHirIdx = ArenaIdx<ExprHir>;
pub type ExprHirIdxRange = ArenaIdxRange<ExprHir>;
