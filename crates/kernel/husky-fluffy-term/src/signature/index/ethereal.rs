use super::*;

pub(crate) fn ethereal_owner_ty_index_signature(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
    owner_ty_arguments: &[EtherealTerm],
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexSignature> {
    let db = engine.db();
    if let Left(prelude_ty_path) = refined_ty_path {
        if let Some(index_signature_maybe_result) = common_ethereal_owner_ty_index_signature(
            engine,
            expr_idx,
            prelude_ty_path,
            owner_ty_arguments,
            index_ty,
        ) {
            return index_signature_maybe_result;
        }
    }
    // fallback to search for trait implementations
    todo!()
}

/// this is an optimization,
/// handles common cases in a quick way
/// option means confident or not
fn common_ethereal_owner_ty_index_signature(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    prelude_ty_path: PreludeTypePath,
    owner_ty_arguments: &[EtherealTerm],
    index_ty: FluffyTerm,
) -> Option<FluffyTermMaybeResult<FluffyIndexSignature>> {
    let db = engine.db();
    match prelude_ty_path {
        PreludeTypePath::Basic(_) => todo!(),
        PreludeTypePath::Num(_) => todo!(),
        PreludeTypePath::Borrow(_) => return Some(Nothing),
        PreludeTypePath::Nat => todo!(),
        PreludeTypePath::Lifetime => todo!(),
        PreludeTypePath::Module => todo!(),
        PreludeTypePath::Trait => todo!(),
        PreludeTypePath::List | PreludeTypePath::Slice => {
            if owner_ty_arguments.len() != 1 {
                todo!()
            }
            let element_ty = owner_ty_arguments[0];
            Some(list_index_signature(
                engine,
                expr_idx,
                element_ty.into(),
                index_ty,
            ))
        }
        PreludeTypePath::Array => todo!(),
        PreludeTypePath::Array2d => todo!(),
        PreludeTypePath::Array3d => todo!(),
        PreludeTypePath::Array4d => todo!(),
        PreludeTypePath::Array5d => todo!(),
        PreludeTypePath::StringLiteral => todo!(),
        PreludeTypePath::Str => todo!(),
        PreludeTypePath::Option => todo!(),
        PreludeTypePath::Result => todo!(),
    }
}
