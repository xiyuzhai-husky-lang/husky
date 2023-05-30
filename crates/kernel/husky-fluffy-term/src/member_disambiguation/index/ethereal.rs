use super::*;

pub(super) fn ethereal_owner_ty_index_disambiguation(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    owner_ty: EtherealTerm,
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexDisambiguation> {
    if let Some(index_signature) =
        common_ethereal_owner_ty_index_signature(engine, expr_idx, owner_ty, index_ty)
            .into_result_option()?
    {
        todo!()
        // return JustOk(index_signature);
    }
    // fallback to search for trait implementations
    todo!();
    // indirections
    todo!()
}

/// this is an optimization,
/// handles common cases in a quick way
fn common_ethereal_owner_ty_index_signature(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    owner_ty: EtherealTerm,
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexSignature> {
    let db = engine.db();
    let owner_ty_application_expansion = owner_ty.application_expansion(db);
    let TermFunctionReduced::TypeOntology(ty_path) = owner_ty_application_expansion.function() else {
        todo!()
    };
    let Right(prelude_ty_path) =  ty_path.refine(db) else {
        return Nothing
    };
    match prelude_ty_path {
        PreludeTypePath::Basic(_) => todo!(),
        PreludeTypePath::Num(_) => todo!(),
        PreludeTypePath::Borrow(_) => todo!(),
        PreludeTypePath::Nat => todo!(),
        PreludeTypePath::Lifetime => todo!(),
        PreludeTypePath::Module => todo!(),
        PreludeTypePath::Trait => todo!(),
        PreludeTypePath::List => {
            let owner_ty_arguments = &owner_ty_application_expansion.arguments(db);
            if owner_ty_arguments.len() != 1 {
                todo!()
            }
            let element_ty = owner_ty_arguments[0];
            ethereal_list_index_signature(engine, expr_idx, element_ty, index_ty)
        }
        PreludeTypePath::Array => todo!(),
        PreludeTypePath::Array2d => todo!(),
        PreludeTypePath::Array3d => todo!(),
        PreludeTypePath::Array4d => todo!(),
        PreludeTypePath::Array5d => todo!(),
        PreludeTypePath::Slice => todo!(),
        PreludeTypePath::StringLiteral => todo!(),
        PreludeTypePath::Str => todo!(),
    }
}

fn ethereal_list_index_signature(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    owner_ty: EtherealTerm,
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexSignature> {
    match index_ty.data(engine) {
        FluffyTermData::Literal(_) => todo!(),
        FluffyTermData::TypeOntology {
            path,
            refined_path,
            arguments,
            ty_ethereal_term,
        } => todo!(),
        FluffyTermData::PlaceTypeOntology {
            place,
            path,
            refined_path,
            arguments,
            base_ty_ethereal_term,
        } => todo!(),
        FluffyTermData::Curry {
            curry_kind,
            variance,
            parameter_variable,
            parameter_ty,
            return_ty,
        } => todo!(),
        FluffyTermData::Hole(hole_kind, _) => match hole_kind {
            HoleKind::UnspecifiedIntegerType => {
                let expectation = ExpectImplicitlyConvertible::new_pure(
                    engine,
                    engine.term_menu().usize_ty_ontology().into(),
                );
                engine.fluffy_term_region_mut().add_expectation(
                    ExpectationSource::new_expr(expr_idx),
                    index_ty,
                    expectation,
                );
                JustOk(FluffyIndexSignature {})
            }
            HoleKind::UnspecifiedFloatType => todo!(),
            HoleKind::ImplicitType => todo!(),
        },
        FluffyTermData::Category(_) => todo!(),
        FluffyTermData::Ritchie {
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
        } => todo!(),
        FluffyTermData::PlaceHole {
            place,
            hole_kind,
            hole,
        } => todo!(),
    }
}
