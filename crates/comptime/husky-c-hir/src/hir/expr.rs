mod literal;
mod opr;
mod stmt;

pub use literal::*;
pub use opr::*;
pub use stmt::*;

use super::*;
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

impl CHirTranspile for CExprHir {
    fn resistance(&self) -> u8 {
        match self {
            CExprHir::Literal(_) | CExprHir::InheritedSymbol | CExprHir::CurrentSymbol => u8::MAX,
            CExprHir::BinaryOpn { .. } => 2,
            CExprHir::PrefixOpn { .. } => 4,
            CExprHir::SuffixOpn { .. } => 5,
            CExprHir::FunctionCall { .. } => 1,
            CExprHir::Field { .. } => 3,
            CExprHir::NewBoxList { .. } => 1,
            CExprHir::Block { .. } => 0,
        }
    }

    fn transpile(&self, transpiler: &mut CTranspiler, ctx: CTranspilerContext) {
        match self {
            CExprHir::Literal(_) => todo!(),
            CExprHir::InheritedSymbol => todo!(),
            CExprHir::CurrentSymbol => todo!(),
            CExprHir::BinaryOpn { lopd: _, opr: _, ropd: _ } => todo!(),
            CExprHir::PrefixOpn { opr: _, opd: _ } => todo!(),
            CExprHir::SuffixOpn { opd: _, punctuation: _ } => todo!(),
            CExprHir::FunctionCall {
                function,
                arguments,
            } => {
                let c_expr_hir_arena = transpiler.c_expr_hir_arena();
                transpiler.transpile_subhir(&ctx, &c_expr_hir_arena[*function]);
                transpiler.transpile_comma_separated_subhirs(
                    &ctx,
                    CBracket::Parenthesis,
                    arguments
                        .into_iter()
                        .map(|argument| &c_expr_hir_arena[argument]),
                );
            }
            CExprHir::Field { self_expr: _, ident: _ } => todo!(),
            CExprHir::NewBoxList { caller: _, items: _ } => todo!(),
            CExprHir::Block { stmts: _ } => todo!(),
        }
    }
}
