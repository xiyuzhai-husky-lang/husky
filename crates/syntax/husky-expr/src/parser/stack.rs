use super::*;

#[derive(Default, Debug)]
pub(crate) struct ExprParserStack {
    unfinished_exprs: Vec<(UnfinishedExpr, Precedence)>,
    top_expr: Option<(Expr, BaseEntityPath)>,
    base_entity_paths: Vec<BaseEntityPath>,
}

impl Expr {
    fn base_entity_path(&self) -> BaseEntityPath {
        match self {
            Expr::Atom(atom) => match atom {
                AtomExpr::Literal(_) => todo!(),
                AtomExpr::Symbol(_) => todo!(),
                AtomExpr::Uncertain(_) => todo!(),
                AtomExpr::Unrecognized(_) => BaseEntityPath::Uncertain,
            },
            Expr::Opn { opn, opds } => match opn {
                Opn::Binary(_) => todo!(),
                Opn::Prefix(_) => todo!(),
                Opn::Suffix(suffix) => match suffix {
                    SuffixPunctuation::Incr
                    | SuffixPunctuation::Decr
                    | SuffixPunctuation::Unveil => BaseEntityPath::None,
                },
                Opn::CurlBracketed => todo!(),
                Opn::List(opr) => match opr {
                    ListOpr::NewTuple => todo!(),
                    ListOpr::NewVec => BaseEntityPath::None,
                    ListOpr::NewDict => todo!(),
                    ListOpr::FunctionCall => todo!(),
                    ListOpr::Index => todo!(),
                    ListOpr::ModuloIndex => todo!(),
                    ListOpr::StructInit => todo!(),
                    ListOpr::MethodCall { ranged_ident } => todo!(),
                },
                Opn::Field(_) => todo!(),
                Opn::Abstraction => todo!(),
            },
            Expr::Bracketed(_) => todo!(),
        }
    }
}

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(super) fn number_of_oprs(&self) -> usize {
        self.stack.unfinished_exprs.len()
    }

    pub(super) fn top_expr(&self) -> Option<&Expr> {
        self.stack.top_expr.as_ref().map(|(expr, _)| expr)
    }

    pub(super) fn top_base_entity_path(&self) -> Option<BaseEntityPath> {
        self.stack.base_entity_paths.last().map(|v| *v)
    }

    pub(super) fn finish_batch(&mut self) -> Option<ExprIdx> {
        core::mem::take(&mut self.stack.top_expr)
            .map(|(expr, path)| self.sheet.alloc_expr(expr, path))
    }

    pub(super) fn push_expr(&mut self, expr: Expr) {
        if self.stack.top_expr.is_none() {
            let base_entity_path = expr.base_entity_path();
            self.stack.top_expr = Some((expr, base_entity_path))
        } else {
            todo!()
        }
        // self.stack.base_entity_paths.push(expr.base_entity_path());
        // self.stack.exprs.push(expr)
    }

    pub(super) fn push_opr(&mut self, opr: UnfinishedExpr) {
        assert!(self.stack.top_expr.is_none());
        let precedence = opr.precedence();
        self.stack.unfinished_exprs.push((opr, precedence))
    }

    pub(super) fn top_opn(&self) -> Option<&UnfinishedExpr> {
        self.stack.unfinished_exprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn pop_opr(&mut self) -> Option<UnfinishedExpr> {
        self.stack.unfinished_exprs.pop().map(|(opr, _)| opr)
    }

    pub(super) fn drain_exprs(&mut self, k: usize) -> (Vec<Expr>, Vec<BaseEntityPath>) {
        todo!()
        // let len = self.stack.exprs.len();
        // assert_eq!(len, self.stack.base_entity_paths.len());
        // (
        //     self.stack.exprs.drain((len - k)..).collect(),
        //     self.stack.base_entity_paths.drain((len - k)..).collect(),
        // )
    }
}
