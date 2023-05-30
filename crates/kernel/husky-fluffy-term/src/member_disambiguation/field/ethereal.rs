use husky_ethereal_signature::{
    HasRegularFieldEtherealSignature, HasTypeMemoizedFieldEtherealSignature,
    HasTypeMethodEtherealSignatureTemplates,
};

use super::*;

pub(super) fn ethereal_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTerm,
    ident: Ident,
) -> FluffyTermMaybeResult<&FluffyFieldDisambiguation> {
    // divide into cases for memoization
    match ty_term {
        EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
            ethereal_ty_ontology_path_ty_field_disambiguation(db, ty_path, ident).just_ok_as_ref()
        }
        EtherealTerm::Application(ty_term) => {
            ethereal_term_application_ty_field_disambiguation(db, ty_term, ident).just_ok_as_ref()
        }
        _ => Nothing,
    }
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn ethereal_ty_ontology_path_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_path: TypePath,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
    ethereal_ty_field_disambiguation_aux(db, ty_path, &[], ident, smallvec![])
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn ethereal_term_application_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTermApplication,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
    let application_expansion = ty_term.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => ethereal_ty_field_disambiguation_aux(
            db,
            ty_path,
            application_expansion.arguments(db),
            ident,
            smallvec![],
        ),
        TermFunctionReduced::Trait(_) | TermFunctionReduced::Other(_) => Nothing,
    }
}

fn ethereal_ty_field_disambiguation_aux<'a>(
    db: &'a dyn FluffyTermDb,
    ty_path: TypePath,
    arguments: &'a [EtherealTerm],
    ident: Ident,
    mut indirections: SmallVec<[FluffyIndirection; 2]>,
) -> FluffyTermMaybeResult<FluffyFieldDisambiguation> {
    match ty_path.refine(db) {
        Left(PreludeTypePath::Borrow(borrow_ty_path)) => match borrow_ty_path {
            PreludeBorrowTypePath::Ref => todo!(),
            PreludeBorrowTypePath::RefMut => todo!(),
            PreludeBorrowTypePath::Leash => {
                indirections.push(FluffyIndirection::Leash);
                if arguments.len() != 1 {
                    todo!()
                }
                return JustOk(
                    ethereal_ty_field_disambiguation(db, arguments[0], ident)?.merge(indirections),
                );
            }
        },
        _ => (),
    }
    if let Some(regular_field_ethereal_signature) = ty_path
        .regular_field_ethereal_signature(db, arguments, ident)
        .into_result_option()?
    {
        return JustOk(FluffyFieldDisambiguation {
            indirections,
            ty_path,
            signature: regular_field_ethereal_signature.into(),
        });
    };

    if let Some(memoized_field_ethereal_signature) = ty_path
        .ty_memoized_field_ethereal_signature(db, arguments, ident)
        .into_result_option()?
    {
        return JustOk(FluffyFieldDisambiguation {
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
