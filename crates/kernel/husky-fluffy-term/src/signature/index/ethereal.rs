mod int_index;

use self::int_index::*;
use super::*;

pub(crate) fn ethereal_owner_ty_index_signature(
    engine: &mut impl FluffyTermEngine,
    syn_expr_idx: SynExprIdx,
    owner_ty: EtherealTerm,
    refined_ty_path: Either<PreludeTypePath, CustomTypePath>,
    owner_ty_arguments: &[EtherealTerm],
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexSignature> {
    let db = engine.db();
    match refined_ty_path {
        Left(prelude_ty_path) => prelude_ethereal_owner_ty_index_signature(
            engine,
            syn_expr_idx,
            prelude_ty_path,
            owner_ty_arguments,
            index_ty,
        ),
        Right(custom_ty_path) => {
            // fallback to search for trait implementations
            let vfs_menu = engine.item_path_menu();
            if let Some(signature) = ethereal_owner_ty_int_index_signature(
                engine,
                syn_expr_idx,
                owner_ty,
                custom_ty_path,
                owner_ty_arguments,
                index_ty,
            )
            .into_result_option()?
            {
                JustOk(signature)
            } else {
                Nothing
            }
        }
    }
}

/// this is an optimization,
/// handles common cases in a quick way
/// option means confident or not
fn prelude_ethereal_owner_ty_index_signature(
    engine: &mut impl FluffyTermEngine,
    expr_idx: SynExprIdx,
    prelude_ty_path: PreludeTypePath,
    owner_ty_arguments: &[EtherealTerm],
    index_ty: FluffyTerm,
) -> FluffyTermMaybeResult<FluffyIndexSignature> {
    let db = engine.db();
    match prelude_ty_path {
        PreludeTypePath::Basic(_) => todo!(),
        PreludeTypePath::Num(_) => todo!(),
        PreludeTypePath::Indirection(_) => Nothing,
        PreludeTypePath::Nat => todo!(),
        PreludeTypePath::Lifetime => todo!(),
        PreludeTypePath::Module => todo!(),
        PreludeTypePath::Trait => todo!(),
        PreludeTypePath::List | PreludeTypePath::Slice | PreludeTypePath::CyclicSliceLeashed => {
            if owner_ty_arguments.len() != 1 {
                todo!()
            }
            let element_ty = owner_ty_arguments[0];
            list_like_index_signature(engine, expr_idx, element_ty.into(), index_ty)
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
