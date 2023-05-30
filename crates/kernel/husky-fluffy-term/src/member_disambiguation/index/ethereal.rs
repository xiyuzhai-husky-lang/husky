use super::*;

pub(super) fn ethereal_owner_ty_index_disambiguation(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    owner_ty: EtherealTerm,
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexDisambiguation> {
    ethereal_owner_ty_index_disambiguation_aux(
        engine,
        expr_idx,
        owner_ty,
        index_ty,
        Default::default(),
    )
}

pub(super) fn ethereal_owner_ty_index_disambiguation_aux(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    owner_ty: EtherealTerm,
    index_ty: FluffyTerm,
    mut indirections: FluffyIndirections,
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
        Left(prelude_ty_path) => match prelude_ty_path {
            PreludeTypePath::Borrow(prelude_borrow_ty_path) => match prelude_borrow_ty_path {
                PreludeBorrowTypePath::Ref => todo!(),
                PreludeBorrowTypePath::RefMut => todo!(),
                PreludeBorrowTypePath::Leash => {
                    indirections.push(FluffyIndirection::Leash);
                    if owner_ty_arguments.len() != 1 {
                        todo!()
                    }
                    ethereal_owner_ty_index_disambiguation_aux(
                        engine,
                        expr_idx,
                        owner_ty_arguments[0],
                        index_ty,
                        indirections,
                    )
                }
            },
            PreludeTypePath::Basic(_)
            | PreludeTypePath::Num(_)
            | PreludeTypePath::Nat
            | PreludeTypePath::Lifetime
            | PreludeTypePath::Module
            | PreludeTypePath::Trait
            | PreludeTypePath::List
            | PreludeTypePath::Array
            | PreludeTypePath::Array2d
            | PreludeTypePath::Array3d
            | PreludeTypePath::Array4d
            | PreludeTypePath::Array5d
            | PreludeTypePath::Slice
            | PreludeTypePath::StringLiteral
            | PreludeTypePath::Str => Nothing,
        },
        Right(_) => todo!(),
    }
}
