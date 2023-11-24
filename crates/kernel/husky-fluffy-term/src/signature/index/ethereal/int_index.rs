use super::*;
use husky_ethereal_signature::helpers::trai_for_ty::ty_side_trai_for_ty_impl_block_signature_templates;

pub(super) fn ethereal_owner_ty_int_index_signature(
    engine: &mut impl FluffyTermEngine,
    syn_expr_idx: SynExprIdx,
    owner_ty: EtherealTerm,
    custom_ty_path: CustomTypePath,
    owner_ty_arguments: &[EtherealTerm],
    index_ty: FluffyTerm,
    final_place: FluffyPlace,
) -> FluffyTermMaybeResult<FluffyIndexSignature> {
    if !coersible_to_int(engine, index_ty) {
        return Nothing;
    };
    let db = engine.db();
    let element_ty = ty_side_trai_for_ty_impl_block_signature_templates(
        db,
        engine.item_path_menu().int_index_trai_path(),
        custom_ty_path.into(),
    )?
    .iter()
    .filter_map(|template| {
        template
            .instantiate_ty(db, owner_ty_arguments, owner_ty)
            .into_option_result()
    })
    .next()??
    .associated_output_template(db)?
    .try_into_signature(db)
    .expect("fully instantiated")
    .ty_term();
    JustOk(FluffyIndexSignature::Int {
        element_ty: FluffyTerm::new_ethereal(final_place, element_ty),
    })
}

fn coersible_to_int(engine: &mut impl FluffyTermEngine, index_ty: FluffyTerm) -> bool {
    match index_ty.data(engine) {
        FluffyTermData::Literal(_) => unreachable!(),
        FluffyTermData::TypeOntology {
            refined_ty_path, ..
        } => match refined_ty_path {
            Left(PreludeTypePath::Num(PreludeNumTypePath::Int(_))) => true,
            _ => false,
        },
        FluffyTermData::Curry { .. } => false,
        FluffyTermData::Hole(hole_kind, _) => match hole_kind {
            HoleKind::UnspecifiedIntegerType => true,
            HoleKind::UnspecifiedFloatType => false,
            HoleKind::ImplicitType => false,
            HoleKind::Any => false,
        },
        FluffyTermData::Category(_) => false,
        FluffyTermData::Ritchie {
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
        } => false,
        FluffyTermData::Symbol { term, ty } => false,
        FluffyTermData::Variable { ty } => unreachable!(),
        FluffyTermData::TypeVariant { path } => unreachable!(),
    }
}
