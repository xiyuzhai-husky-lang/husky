use super::*;

pub(super) fn ethereal_owner_ty_index_disambiguation(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    owner_ty: EtherealTerm,
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexDisambiguation> {
    let db = engine.db();
    let owner_ty_application_expansion = owner_ty.application_expansion(db);
    let TermFunctionReduced::TypeOntology(ty_path) = owner_ty_application_expansion.function() else {
        todo!()
    };
    let refined_ty_path = ty_path.refine(db);
    let owner_ty_arguments = owner_ty_application_expansion.arguments(db);
    if let Some(index_signature) = ethereal_owner_ty_index_signature(
        engine,
        expr_idx,
        refined_ty_path,
        owner_ty_arguments,
        index_ty,
    )
    .into_result_option()?
    {
        return JustOk(FluffyIndexDisambiguation::new(index_signature));
    };
    // indirections
    match refined_ty_path {
        Left(_) => todo!(),
        Right(_) => todo!(),
    }
}
