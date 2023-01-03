use std::collections::HashMap;

use crate::*;
use husky_ast::AstSheet;
use husky_expr::ExprMap;
use husky_term_pattern::*;
use husky_text::TextRange;

#[derive(Debug, PartialEq, Eq)]
pub struct TermPatternInferSheet {
    term_patt_itr: TermPatternInterner,
    var_results: HashMap<(Identifier, TextRange), VarTermPatternInferResults>,
    expr_results: ExprMap<ExprTermPatternInferResults>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarTermPatternInferResults {
    // todo: add qualifier
    const_expr: TermPatternInferResult<Option<ConstExprPatternItd>>,
    ty: TermPatternInferResult<TermPatternItd>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExprTermPatternInferResults {
    const_expr: TermPatternInferResult<Option<ConstExprPatternItd>>,
    ty: TermPatternInferResult<TermPatternItd>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExprTermPatternInferRawResults {
    pub(crate) const_expr: TermPatternInferResult<Option<ConstExprPattern>>,
    pub(crate) ty: TermPatternInferResult<TermPatternItd>,
}

impl TermPatternInferSheet {
    #[cfg(test)]
    pub(crate) fn new_test(
        sheet: ExprSheet,
        fake_var_results: HashMap<(Identifier, TextRange), VarTermPatternInferResults>,
    ) -> Self {
        todo!()
        // Self {
        //     term_patt_itr: Default::default(),
        //     expr_results: ExprMap::new(sheet.expr_arena()),
        //     var_results: fake_var_results,
        // }
    }

    pub(crate) fn insert_result(&mut self, expr: ExprIdx, result: ExprTermPatternInferRawResults) {
        self.expr_results.insert_new(
            expr,
            ExprTermPatternInferResults {
                const_expr: result.const_expr.map(|const_expr| {
                    const_expr.map(|const_expr| ConstExprPatternItd::new(const_expr))
                }),
                ty: result.ty,
            },
        )
    }

    pub(crate) fn insert_term_infer_result(
        &mut self,
        _expr: ExprIdx,
        _term: TermPatternInferResult<Term>,
    ) {
        todo!()
    }

    pub(crate) fn const_expr(
        &self,
        expr: ExprIdx,
    ) -> &TermPatternInferResult<Option<ConstExprPatternItd>> {
        &self.expr_results.get(expr).unwrap().const_expr
    }
}

impl TermPatternInferSheet {
    pub fn ast_text(&self) -> &AstSheet {
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
    pub(crate) fn var_ty_result(
        &self,
        _sheet: &TermPatternInferSheet,
        _ident: Identifier,
        _init_range: TextRange,
    ) -> TermPatternInferResult<TermPatternItd> {
        todo!()
    }

    pub(crate) fn expr_ty_result(
        &self,
        sheet: &TermPatternInferSheet,
        expr: ExprIdx,
    ) -> TermPatternInferResult<TermPatternItd> {
        match sheet.expr_results[expr].ty {
            Ok(ty) => Ok(ty),
            Err(ref e) => self.err_derived(e.clone()),
        }
    }
}
