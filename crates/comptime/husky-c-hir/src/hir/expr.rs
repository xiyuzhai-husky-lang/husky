pub enum CExprHir {
    Literal(LiteralHir),
    InheritedSymbol,
    CurrentSymbol,
    BinaryOpn {
        lopd: CExprHirIdx,
        opr: BinaryOpr,
        ropd: CExprHirIdx,
    },
    PrefixOpn {
        opr: PrefixOpr,
        opd: CExprHirIdx,
    },
    SuffixOpn {
        opd: CExprHirIdx,
        punctuation: SuffixOpr,
    },
    FunctionCall {
        function: CExprHirIdx,
        implicit_arguments: Option<ImplicitArgumentListHir>,
        arguments: CExprHirIdxRange,
    },
    Field {
        self_expr: CExprHirIdx,
        ident_token: Identifier,
    },
    NewBoxList {
        caller: Option<CExprHirIdx>,
        items: CExprHirIdxRange,
    },
    Block {
        stmts: StmtHirIdxRange,
    },
}

pub type CExprHirArena = Arena<CExprHir>;
pub type CExprHirIdx = ArenaIdx<CExprHir>;
pub type CExprHirIdxRange = ArenaIdxRange<CExprHir>;
