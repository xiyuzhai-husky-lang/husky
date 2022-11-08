use crate::*;
use husky_entity_path::EntityPathItd;
use husky_expr_syntax::{RawExpr, RawExprArena, RawExprIdx, RawExprVariant};
use husky_term::{Term, TermAtom, TermContext, TermItd, TermMenu, Ty};
use husky_term_pattern::TermPatternItd;
use husky_word::InternWord;

pub(crate) struct TermPatternInferContext<'a> {
    db: &'a dyn TermPatternInferQueryGroup,
    expr_arena: &'a RawExprArena,
    expr: RawExprIdx,
    term_menu: &'a TermMenu,
}

impl<'a> InternWord for TermPatternInferContext<'a> {
    fn word_itr(&self) -> &husky_word::WordInterner {
        self.db.word_itr()
    }
}

impl<'a> TermPatternInferContext<'a> {
    pub(crate) fn new(
        db: &'a dyn TermPatternInferQueryGroup,
        expr_arena: &'a RawExprArena,
        expr: RawExprIdx,
        term_menu: &'a TermMenu,
    ) -> Self {
        Self {
            db,
            expr_arena,
            expr,
            term_menu,
        }
    }

    pub(crate) fn write_inference(self, sheet: &mut TermPatternInferSheet) {
        if let Some(subexprs) = self.subexprs() {
            for subexpr in subexprs {
                self.subexpr_context(subexpr).write_inference(sheet)
            }
        }
        let ty = self.infer_term_pattern();
        todo!()
        // self.sheet.insert_ty_infer_result(self.expr, ty)
    }

    fn subexpr_context(&self, subexpr: RawExprIdx) -> Self {
        Self {
            db: self.db,
            expr_arena: self.expr_arena,
            expr: subexpr,
            term_menu: self.term_menu,
        }
    }

    fn subexprs(&self) -> Option<RawExprRange> {
        match self.expr().variant {
            RawExprVariant::Atom(_) => None,
            RawExprVariant::Opn { ref opds, .. } => Some(opds.clone()),
        }
    }

    pub(crate) fn expr(&self) -> &'a RawExpr {
        &self.expr_arena[self.expr]
    }

    pub(crate) fn term_menu(&self) -> &'a TermMenu {
        self.term_menu
    }

    #[inline(always)]
    fn term_ctx(&self) -> TermContext<'a> {
        TermContext::new(self.db.upcast(), self.term_menu)
    }

    pub(crate) fn entity_path_term(&self, path: EntityPathItd) -> TermPatternInferResult<TermItd> {
        self.map_original(self.term_ctx().entity_path_term(path))
    }
}
