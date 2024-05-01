use super::*;
use husky_eth_signature::signature::{
    assoc_item::ty_item::memo_field::HasTypeMemoizedFieldEtherealSignature,
    major_item::ty::HasPropsFieldEtherealSignature,
};
use husky_eth_term::term::application::{EthApplication, TermFunctionReduced};

pub(super) fn ethereal_ty_field_dispatch(
    db: &::salsa::Db,
    ty_term: EthTerm,
    ident: Ident,
    indirections: FlyIndirections,
) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
    // divide into cases for memoization
    match ty_term {
        EthTerm::EntityPath(ItemPathTerm::TypeOntology(ty_path)) => {
            ethereal_ty_ontology_path_ty_field_dispatch(db, ty_path, ident, indirections)
        }
        EthTerm::Application(ty_term) => {
            ethereal_term_application_ty_field_dispatch(db, ty_term, ident, indirections)
        }
        _ => Nothing,
    }
}

pub(crate) fn ethereal_ty_ontology_path_ty_field_dispatch(
    db: &::salsa::Db,
    ty_path: TypePath,
    ident: Ident,
    indirections: FlyIndirections,
) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
    ethereal_ty_field_dispatch_aux(db, ty_path, &[], ident, indirections)
}

pub(crate) fn ethereal_term_application_ty_field_dispatch(
    db: &::salsa::Db,
    ty_term: EthApplication,
    ident: Ident,
    indirections: FlyIndirections,
) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
    let application_expansion = ty_term.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => ethereal_ty_field_dispatch_aux(
            db,
            ty_path,
            application_expansion.arguments(db),
            ident,
            indirections,
        ),
        TermFunctionReduced::Trait(_) | TermFunctionReduced::Other(_) => Nothing,
    }
}

fn ethereal_ty_field_dispatch_aux<'a>(
    db: &'a ::salsa::Db,
    ty_path: TypePath,
    arguments: &'a [EthTerm],
    ident: Ident,
    mut indirections: FlyIndirections,
) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
    match ty_path.refine(db) {
        Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)) => {
            match prelude_indirection_ty_path {
                PreludeIndirectionTypePath::Ref => todo!(),
                PreludeIndirectionTypePath::RefMut => todo!(),
                PreludeIndirectionTypePath::Leash => {
                    indirections.add(FlyIndirection::Leash);
                    if arguments.len() != 1 {
                        todo!()
                    }
                    return ethereal_ty_field_dispatch(db, arguments[0], ident, indirections);
                }
                PreludeIndirectionTypePath::At => todo!(),
            }
        }
        _ => (),
    }
    if let Some(regular_field_ethereal_signature) = ty_path
        .props_field_ethereal_signature(db, arguments, ident)
        .into_result_option()?
    {
        return JustOk(FlyFieldDyanmicDispatch {
            indirections,
            ty_path,
            signature: regular_field_ethereal_signature.into(),
        });
    };

    if let Some(memo_field_ethereal_signature) = ty_path
        .ty_memo_field_ethereal_signature(db, arguments, ident)
        .into_result_option()?
    {
        return JustOk(FlyFieldDyanmicDispatch {
            indirections,
            ty_path,
            signature: memo_field_ethereal_signature.into(),
        });
    }
    // todo!("trai for ty memoized field");
    // ad hoc
    // needs to consider `Deref` `DerefMut` `Carrier`
    Nothing
}
