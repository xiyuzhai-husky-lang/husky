mod literal;
mod opr;
mod stmt;

pub use literal::*;
pub use opr::*;
pub use stmt::*;

use idx_arena::*;

pub enum CExprHir {
    Literal(CLiteralHir),
    InheritedSymbol,
    CurrentSymbol,
    BinaryOpn {
        lopd: CExprHirIdx,
        opr: CBinaryOprHir,
        ropd: CExprHirIdx,
    },
    PrefixOpn {
        opr: CPrefixOprHir,
        opd: CExprHirIdx,
    },
    SuffixOpn {
        opd: CExprHirIdx,
        punctuation: CSuffixOprHir,
    },
    FunctionCall {
        function: CExprHirIdx,
        arguments: CExprHirIdxRange,
    },
    Field {
        self_expr: CExprHirIdx,
        ident: CFieldIdentifier,
    },
    NewBoxList {
        caller: Option<CExprHirIdx>,
        items: CExprHirIdxRange,
    },
    Block {
        stmts: CStmtHirIdxRange,
    },
}

pub struct CFieldIdentifier {}

pub type CExprHirArena = Arena<CExprHir>;
pub type CExprHirIdx = ArenaIdx<CExprHir>;
pub type CExprHirIdxRange = ArenaIdxRange<CExprHir>;
