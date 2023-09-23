use husky_ethereal_signature::{
    HasPropsFieldEtherealSignature, HasTypeMemoizedFieldEtherealSignature,
};

use super::*;

pub(super) fn ethereal_ty_field_dispatch(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTerm,
    ident: Ident,
    indirections: FluffyTermDynamicDispatchIndirections,
) -> FluffyTermMaybeResult<FluffyFieldDispatch> {
    // divide into cases for memoization
    match ty_term {
        EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
            ethereal_ty_ontology_path_ty_field_dispatch(db, ty_path, ident, indirections)
        }
        EtherealTerm::Application(ty_term) => {
            ethereal_term_application_ty_field_dispatch(db, ty_term, ident, indirections)
        }
        _ => Nothing,
    }
}

pub(crate) fn ethereal_ty_ontology_path_ty_field_dispatch(
    db: &dyn FluffyTermDb,
    ty_path: TypePath,
    ident: Ident,
    indirections: FluffyTermDynamicDispatchIndirections,
) -> FluffyTermMaybeResult<FluffyFieldDispatch> {
    ethereal_ty_field_dispatch_aux(db, ty_path, &[], ident, indirections)
}

pub(crate) fn ethereal_term_application_ty_field_dispatch(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTermApplication,
    ident: Ident,
    indirections: FluffyTermDynamicDispatchIndirections,
) -> FluffyTermMaybeResult<FluffyFieldDispatch> {
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
    db: &'a dyn FluffyTermDb,
    ty_path: TypePath,
    arguments: &'a [EtherealTerm],
    ident: Ident,
    mut indirections: FluffyTermDynamicDispatchIndirections,
) -> FluffyTermMaybeResult<FluffyFieldDispatch> {
    match ty_path.refine(db) {
        Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)) => {
            match prelude_indirection_ty_path {
                PreludeIndirectionTypePath::Ref => todo!(),
                PreludeIndirectionTypePath::RefMut => todo!(),
                PreludeIndirectionTypePath::Leash => {
                    indirections.add(FluffyTermDynamicDispatchIndirection::Leash);
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
        .regular_field_ethereal_signature(db, arguments, ident)
        .into_result_option()?
    {
        return JustOk(FluffyFieldDispatch {
            indirections,
            ty_path,
            signature: regular_field_ethereal_signature.into(),
        });
    };

    if let Some(memoized_field_ethereal_signature) = ty_path
        .ty_memoized_field_ethereal_signature(db, arguments, ident)
        .into_result_option()?
    {
        return JustOk(FluffyFieldDispatch {
            indirections,
            ty_path,
            signature: memoized_field_ethereal_signature.into(),
        });
    }
    // todo!("trai for ty memoized field");
    // ad hoc
    // needs to consider `Deref` `DerefMut` `Carrier`
    Nothing
}
