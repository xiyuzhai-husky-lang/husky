use super::*;

#[derive(Default, Debug)]
pub(crate) struct AutomataStack {
    oprs: Vec<(PartialOpn, Precedence)>,
    exprs: Vec<Expr>,
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
        self.stack.oprs.len()
    }

    pub(super) fn number_of_exprs(&self) -> usize {
        self.stack.exprs.len()
    }

    pub(super) fn top_expr(&self) -> Option<&Expr> {
        self.stack.exprs.last()
    }

    pub(super) fn top_base_entity_path(&self) -> Option<BaseEntityPath> {
        self.stack.base_entity_paths.last().map(|v| *v)
    }

    pub(super) fn finish_batch(&mut self) -> ExprIdxRange {
        self.sheet.alloc_expr_batch(
            std::mem::take(&mut self.stack.exprs),
            core::mem::take(&mut self.stack.base_entity_paths),
        )
    }

    pub(super) fn topk_exprs(&self, k: usize) -> &[Expr] {
        &self.stack.exprs[(self.stack.exprs.len() - k)..]
    }

    pub(super) fn last_token_in_exprs(&self) -> TokenIdx {
        todo!()
    }
    pub(super) fn expr(&self, idx: usize) -> &Expr {
        &self.stack.exprs[idx]
    }

    pub(super) fn push_expr(&mut self, expr: Expr) {
        self.stack.base_entity_paths.push(expr.base_entity_path());
        self.stack.exprs.push(expr)
    }

    pub(super) fn push_opr(&mut self, opr: PartialOpn) {
        let precedence = opr.precedence();
        self.stack.oprs.push((opr, precedence))
    }

    pub(super) fn top_opn(&self) -> Option<&PartialOpn> {
        self.stack.oprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn pop_opr(&mut self) -> Option<PartialOpn> {
        self.stack.oprs.pop().map(|(opr, _)| opr)
    }

    pub(super) fn drain_exprs(&mut self, k: usize) -> (Vec<Expr>, Vec<BaseEntityPath>) {
        let len = self.stack.exprs.len();
        assert_eq!(len, self.stack.base_entity_paths.len());
        (
            self.stack.exprs.drain((len - k)..).collect(),
            self.stack.base_entity_paths.drain((len - k)..).collect(),
        )
    }
}
