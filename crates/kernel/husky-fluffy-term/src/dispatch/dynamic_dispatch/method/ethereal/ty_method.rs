use super::*;

impl HasFluffyTypeMethodDispatch for EtherealTerm {
    fn ty_method_dispatch(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        indirections: FluffyDynamicDispatchIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        // todo: check scope
        match self {
            EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
                ethereal_ty_ontology_path_ty_method_dispatch(
                    engine,
                    expr_idx,
                    ty_path,
                    ident_token,
                    indirections,
                )
            }
            EtherealTerm::Application(ty_term) => ethereal_term_application_ty_method_dispatch(
                engine,
                expr_idx,
                ty_term,
                ident_token,
                indirections,
            ),
            _ => Nothing,
        }
    }
}

fn ethereal_ty_ontology_path_ty_method_dispatch(
    engine: &mut impl FluffyTermEngine,
    expr_idx: SynExprIdx,
    ty_path: TypePath,
    ident_token: IdentRegionalToken,
    indirections: FluffyDynamicDispatchIndirections,
) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
    ethereal_ty_method_dispatch_aux(engine, expr_idx, ty_path, &[], ident_token, indirections)
}

fn ethereal_term_application_ty_method_dispatch(
    engine: &mut impl FluffyTermEngine,
    expr_idx: SynExprIdx,
    ty_term: EtherealTermApplication,
    ident_token: IdentRegionalToken,
    indirections: FluffyDynamicDispatchIndirections,
) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
    let application_expansion = ty_term.application_expansion(engine.db());
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => ethereal_ty_method_dispatch_aux(
            engine,
            expr_idx,
            ty_path,
            application_expansion.arguments(engine.db()),
            ident_token,
            indirections,
        ),
        TermFunctionReduced::Trait(_) | TermFunctionReduced::Other(_) => Nothing,
    }
}

fn ethereal_ty_method_dispatch_aux(
    engine: &mut impl FluffyTermEngine,
    expr_idx: SynExprIdx,
    ty_path: TypePath,
    arguments: &[EtherealTerm],
    ident_token: IdentRegionalToken,
    mut indirections: FluffyDynamicDispatchIndirections,
) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
    match ty_path.refine(engine.db()) {
        Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)) => {
            match prelude_indirection_ty_path {
                PreludeIndirectionTypePath::Ref => todo!(),
                PreludeIndirectionTypePath::RefMut => todo!(),
                PreludeIndirectionTypePath::Leash => {
                    indirections.push(FluffyDynamicDispatchIndirection::Leash);
                    if arguments.len() != 1 {
                        p!((&arguments).debug(engine.db()));
                        todo!()
                    }
                    return arguments[0].ty_method_dispatch(
                        engine,
                        expr_idx,
                        ident_token,
                        indirections,
                    );
                }
                PreludeIndirectionTypePath::At => todo!(),
            }
        }
        _ => (),
    }
    if let Some(signature) = ty_method_fluffy_signature(
        engine,
        expr_idx,
        ty_path,
        arguments,
        /* ad hoc */ &[],
        ident_token,
    )
    .into_result_option()?
    {
        return JustOk(FluffyDynamicDispatch {
            indirections,
            signature,
        });
    };
    // ad hoc
    // needs to consider `Deref` `DerefMut` `Carrier`
    Nothing
}
