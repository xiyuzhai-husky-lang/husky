use crate::*;
use husky_ast::AstText;
use husky_expr_syntax::RawExprMap;
use husky_term_pattern::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferSheet {
    term_patt_itr: TermPatternInterner,
    expr_results: RawExprMap<TermPatternInferEntry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferEntry {
    const_expr: TermPatternInferResult<Option<ConstExprPatternItd>>,
    ty: TermPatternInferResult<TermPatternItd>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermPatternInferEntry {
    pub(crate) const_expr: TermPatternInferResult<Option<ConstExprPattern>>,
    pub(crate) ty: TermPatternInferResult<TermPatternItd>,
}

impl TermPatternInferSheet {
    pub(crate) fn new(arena: &RawExprArena) -> Self {
        Self {
            term_patt_itr: Default::default(),
            expr_results: RawExprMap::new(arena),
        }
    }

    pub(crate) fn insert_result(&mut self, expr: RawExprIdx, result: RawTermPatternInferEntry) {
        self.expr_results.insert_new(
            expr,
            TermPatternInferEntry {
                const_expr: result.const_expr.map(|const_expr| {
                    const_expr.map(|const_expr| ConstExprPatternItd::new(const_expr))
                }),
                ty: result.ty,
            },
        )
    }

    pub(crate) fn insert_term_infer_result(
        &mut self,
        _expr: RawExprIdx,
        _term: TermPatternInferResult<TermItd>,
    ) {
        todo!()
    }

    pub(crate) fn const_expr(
        &self,
        expr: RawExprIdx,
    ) -> &TermPatternInferResult<Option<ConstExprPatternItd>> {
        &self.expr_results.get(expr).unwrap().const_expr
    }
}

impl TermPatternInferSheet {
    pub fn ast_text(&self) -> &AstText {
        todo!()
    }
}

impl InternTermPattern for TermPatternInferSheet {
    fn term_patt_itr(&self) -> &TermPatternInterner {
        &self.term_patt_itr
    }

    fn term_patt_itr_mut(&mut self) -> &mut TermPatternInterner {
        &mut self.term_patt_itr
    }
}

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn expr_ty_result(
        &self,
        sheet: &TermPatternInferSheet,
        expr: RawExprIdx,
    ) -> TermPatternInferResult<TermPatternItd> {
        match sheet.expr_results[expr].ty {
            Ok(ty) => Ok(ty),
            Err(ref e) => self.err_derived(e.clone()),
        }
    }
}
