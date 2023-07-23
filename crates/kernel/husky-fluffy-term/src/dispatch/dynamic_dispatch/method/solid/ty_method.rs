use super::*;

impl HasFluffyTypeMethodDispatch for SolidTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        self.ty_method_dispatch_aux(engine, expr_idx, ident_token, smallvec![])
    }
}

impl SolidTerm {
    pub(super) fn ty_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentToken,
        mut indirections: SmallVec<[FluffyDynamicDispatchIndirection; 2]>,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::TypeOntologyAtPlace {
                place,
                path,
                refined_path,
                arguments,
                base_ty_term,
            } => match base_ty_term.as_ref() {
                Some(&base_ty_term) => {
                    indirections.push(FluffyDynamicDispatchIndirection::Place(*place));
                    JustOk(
                        base_ty_term
                            .ty_method_dispatch(engine, expr_idx, ident_token)?
                            .merge(indirections),
                    )
                }
                None => todo!(),
            },
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => Nothing,
            SolidTermData::SymbolAtPlace { .. } => todo!(),
        }
    }
}
