mod expr;
mod stmt;
mod symbol;

use husky_opn_syntax::PrefixOpr;
use husky_print_utils::p;
use symbol::*;

use crate::*;

pub(crate) struct ExprTypeEngine<'a> {
    db: &'a dyn ExprTypeDb,
    term_menu: &'a TermMenu,
    expr_region_data: &'a ExprRegionData,
    signature_term_region: &'a SignatureTermRegion,
    expr_ty_infos: ExprMap<TypeInfo>,
    inherited_symbol_tys: InheritedSymbolMap<Term>,
    current_symbol_ty_infos: CurrentSymbolMap<TypeInfo>,
    unresolved_term_table: UnresolvedTermTable,
    output_ty: Option<Term>,
}

impl<'a> std::ops::Index<ExprIdx> for ExprTypeEngine<'a> {
    type Output = Expr;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_region_data[index]
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn new(db: &'a dyn ExprTypeDb, expr_region: ExprRegion) -> Self {
        let expr_region_data = expr_region.data(db);
        let output_ty = expr_region_data
            .parent()
            .map(|parent| {
                db.expr_ty_region(parent)[parent.data(db).output_ty()?]
                    .resolved_ty()
                    .as_ref()
                    .ok()
                    .copied()
            })
            .flatten();
        let symbol_region = expr_region_data.symbol_region();
        Self {
            db,
            term_menu: db.term_menu(expr_region.toolchain(db)).as_ref().unwrap(),
            expr_region_data,
            signature_term_region: db.signature_term_region(expr_region),
            expr_ty_infos: ExprMap::new(expr_region_data.expr_arena()),
            inherited_symbol_tys: InheritedSymbolMap::new(symbol_region.inherited_symbol_arena()),
            current_symbol_ty_infos: CurrentSymbolMap::new(symbol_region.current_symbol_arena()),
            unresolved_term_table: Default::default(),
            output_ty,
        }
    }

    pub(crate) fn infer_all(&mut self) {
        self.infer_all_parameter_symbols();
        self.infer_all_exprs();
    }

    fn infer_all_exprs(&mut self) {
        for root in self.expr_region_data.roots() {
            let ty = self.infer_new_expr(root.expr(), Expectation::None);
            // todo: check coherence
        }
    }

    pub(crate) fn finish(self) -> ExprTypeRegion {
        ExprTypeRegion::new(
            self.expr_region_data.path(),
            self.expr_ty_infos,
            self.unresolved_term_table,
        )
    }
}
