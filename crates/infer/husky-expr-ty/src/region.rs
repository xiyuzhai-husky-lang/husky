use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeRegion {
    path: RegionPath,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    expr_local_terms: ExprMap<ExprTermResult<LocalTerm>>,
    inherited_symbol_tys: InheritedSymbolMap<ReducedTerm>,
    current_symbol_tys: CurrentSymbolMap<LocalTerm>,
    local_term_region: LocalTermRegion,
    return_ty: Option<ReducedTerm>,
    self_ty: Option<ReducedTerm>,
}

impl ExprTypeRegion {
    pub(crate) fn new(
        db: &dyn ExprTypeDb,
        reduced_term_menu: ReducedTermMenu,
        path: RegionPath,
        mut expr_ty_infos: ExprMap<ExprTypeInfo>,
        expr_terms: ExprMap<ExprTermResult<LocalTerm>>,
        inherited_symbol_tys: InheritedSymbolMap<ReducedTerm>,
        current_symbol_tys: CurrentSymbolMap<LocalTerm>,
        local_term_region: LocalTermRegion,
        return_ty: Option<ReducedTerm>,
        self_ty: Option<ReducedTerm>,
    ) -> Self {
        expr_ty_infos
            .iter_mut()
            .for_each(|info| info.finalize(&local_term_region));
        Self {
            path,
            expr_ty_infos,
            expr_local_terms: expr_terms,
            inherited_symbol_tys,
            current_symbol_tys,
            local_term_region,
            return_ty,
            self_ty,
        }
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }

    pub fn expr_ty_infos(&self) -> &ExprMap<ExprTypeInfo> {
        &self.expr_ty_infos
    }

    pub fn expr_local_terms(&self) -> &ExprMap<ExprTermResult<LocalTerm>> {
        &self.expr_local_terms
    }

    pub fn inherited_symbol_tys(&self) -> &InheritedSymbolMap<ReducedTerm> {
        &self.inherited_symbol_tys
    }

    pub fn current_symbol_tys(&self) -> &CurrentSymbolMap<LocalTerm> {
        &self.current_symbol_tys
    }

    pub fn local_term_region(&self) -> &LocalTermRegion {
        &self.local_term_region
    }

    pub fn return_ty(&self) -> Option<ReducedTerm> {
        self.return_ty
    }

    pub fn self_ty(&self) -> Option<ReducedTerm> {
        self.self_ty
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, expr_region);
    let mut local_term_region = LocalTermRegion::default();
    engine.infer_all(&mut local_term_region);
    engine.finish(local_term_region)
}

pub(crate) struct PatternExprTypeInfo {
    ty: PatternExprTypeResult<LocalTerm>,
}

impl PatternExprTypeInfo {
    pub(crate) fn new(ty: PatternExprTypeResult<LocalTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&LocalTerm, &PatternExprTypeError> {
        self.ty.as_ref()
    }
}

pub(crate) struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<LocalTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<LocalTerm>) -> Self {
        Self { ty }
    }
}
