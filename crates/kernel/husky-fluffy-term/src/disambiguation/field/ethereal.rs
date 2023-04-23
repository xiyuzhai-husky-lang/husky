use super::*;

pub(super) fn ethereal_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTerm,
    ident: Ident,
) -> FluffyTermResult<Option<&FluffyFieldDisambiguation>> {
    // divide into cases for memoization
    let disambiguation = match ty_term {
        EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
            ethereal_ty_ontology_path_ty_field_disambiguation(db, ty_path, ident)
        }
        EtherealTerm::Application(ty_term) => {
            ethereal_term_application_ty_field_disambiguation(db, ty_term, ident)
        }
        _ => return Ok(None),
    };
    match disambiguation {
        Ok(disambiguation) => Ok(disambiguation.as_ref()),
        Err(e) => Err(*e),
    }
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn ethereal_ty_ontology_path_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_path: TypePath,
    ident: Ident,
) -> FluffyTermResult<Option<FluffyFieldDisambiguation>> {
    ethereal_ty_field_disambiguation_aux(db, ty_path, &[], ident, smallvec![])
}

#[salsa::tracked(jar = FluffyTermJar, return_ref)]
pub(crate) fn ethereal_term_application_ty_field_disambiguation(
    db: &dyn FluffyTermDb,
    ty_term: EtherealTermApplication,
    ident: Ident,
) -> FluffyTermResult<Option<FluffyFieldDisambiguation>> {
    let application_expansion = ty_term.application_expansion(db);
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => ethereal_ty_field_disambiguation_aux(
            db,
            ty_path,
            application_expansion.arguments(db),
            ident,
            smallvec![],
        ),
        TermFunctionReduced::Trait(_) | TermFunctionReduced::Other(_) => Ok(None),
    }
}

fn ethereal_ty_field_disambiguation_aux<'a>(
    db: &'a dyn FluffyTermDb,
    ty_path: TypePath,
    arguments: &'a [EtherealTerm],
    ident: Ident,
    mut indirections: SmallVec<[FluffyFieldIndirection; 2]>,
) -> FluffyTermResult<Option<FluffyFieldDisambiguation>> {
    todo!()
}
