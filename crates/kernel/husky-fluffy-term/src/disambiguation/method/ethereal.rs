use husky_ethereal_signature::*;

use super::*;

pub(super) fn ethereal_ty_method_disambiguation(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTerm,
    ident: Ident,
) -> FluffyTermMaybeResult<&FluffyMethodDisambiguation> {
    // divide into cases for memoization
    match ty_term {
        EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
            ethereal_ty_ontology_path_ty_method_disambiguation(db, ty_path, ident).just_ok_as_ref()
        }
        EtherealTerm::Application(ty_term) => {
            ethereal_term_application_ty_method_disambiguation(db, ty_term, ident).just_ok_as_ref()
        }
        _ => Nothing,
    }
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn ethereal_ty_ontology_path_ty_method_disambiguation(
    db: &dyn FluffyTermDb,
    ty_path: TypePath,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
    ethereal_ty_method_disambiguation_aux(db, ty_path, &[], ident, smallvec![])
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn ethereal_term_application_ty_method_disambiguation(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTermApplication,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
    let application_expansion = ty_term.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => ethereal_ty_method_disambiguation_aux(
            db,
            ty_path,
            application_expansion.arguments(db),
            ident,
            smallvec![],
        ),
        TermFunctionReduced::Trait(_) | TermFunctionReduced::Other(_) => Nothing,
    }
}

fn ethereal_ty_method_disambiguation_aux<'a>(
    db: &'a dyn FluffyTermDb,
    ty_path: TypePath,
    arguments: &'a [EtherealTerm],
    ident: Ident,
    mut indirections: SmallVec<[FluffyIndirection; 2]>,
) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
    match ty_path.refine(db) {
        Right(PreludeTypePath::Borrow(borrow_ty_path)) => match borrow_ty_path {
            PreludeBorrowTypePath::Ref => todo!(),
            PreludeBorrowTypePath::RefMut => todo!(),
            PreludeBorrowTypePath::Leash => {
                indirections.push(FluffyIndirection::Leash);
                if arguments.len() != 1 {
                    todo!()
                }
                return JustOk(
                    ethereal_ty_method_disambiguation(db, arguments[0], ident)?.merge(indirections),
                );
            }
        },
        _ => (),
    }
    if let Some(signature) = method_fluffy_signature(
        db,
        ty_path,
        arguments.iter().copied().map(Into::into),
        &[],
        ident,
    )
    .into_result_option()?
    {
        return JustOk(FluffyMemberDisambiguation {
            indirections,
            ty_path,
            signature,
        });
    };
    if indirections.contains(&FluffyIndirection::Leash) {
        todo!()
    }
    // ad hoc
    // needs to consider `Deref` `DerefMut` `Carrier`
    Nothing
}
