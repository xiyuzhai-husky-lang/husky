use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeRegion {
    path: RegionPath,
    expr_ty_infos: ExprMap<TypeInfo>,
    stmt_ty_infos: StmtMap<TypeInfo>,
    unresolved_term_table: UnresolvedTermTable,
}

impl ExprTypeRegion {
    pub(crate) fn new(
        path: RegionPath,
        mut expr_ty_infos: ExprMap<TypeInfo>,
        mut stmt_ty_infos: StmtMap<TypeInfo>,
        unresolved_term_table: UnresolvedTermTable,
    ) -> Self {
        expr_ty_infos
            .iter_mut()
            .chain(stmt_ty_infos.iter_mut())
            .for_each(|info| info.finalize(&unresolved_term_table));
        Self {
            path,
            expr_ty_infos,
            stmt_ty_infos,
            unresolved_term_table,
        }
    }
}

impl std::ops::Index<ExprIdx> for ExprTypeRegion {
    type Output = TypeInfo;

    fn index(&self, index: ExprIdx) -> &Self::Output {
        &self.expr_ty_infos[index]
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct TypeInfo {
    ty_result: ExprTypeResult<LocalTerm>,
    opt_expectation: OptionExpectationIdx,
    // ideally this should be `MaybeUninit`
    // but Rust's type system is not that handy to do this
    resolved_ty: Option<ExprTypeResult<Term>>,
}

impl TypeInfo {
    pub(crate) fn new(
        ty_result: ExprTypeResult<LocalTerm>,
        opt_expectation: OptionExpectationIdx,
    ) -> Self {
        Self {
            ty_result,
            opt_expectation,
            resolved_ty: None,
        }
    }

    pub(crate) fn ty(&self) -> ExprTypeResultRef<LocalTerm> {
        self.ty_result.as_ref().copied()
    }

    fn finalize(&mut self, unresolved_term_table: &UnresolvedTermTable) {
        let Ok(ty) = self.ty_result else { return };
        self.resolved_ty = Some(match self.opt_expectation.into_option() {
            Some(expectation) => todo!(),
            None => match ty {
                LocalTerm::Resolved(ty) => Ok(ty),
                LocalTerm::Unresolved(_) => todo!(),
            },
        })
    }

    pub(crate) fn resolved_ty(&self) -> &ExprTypeResult<Term> {
        self.resolved_ty.as_ref().expect(
            r#"should be initialized by the time it's called;
            this invariance is guaranteed by `ExprTypeRegion::new`"#,
        )
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, expr_region);
    engine.infer_all();
    engine.finish()
}
