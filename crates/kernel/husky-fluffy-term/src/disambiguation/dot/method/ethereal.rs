use husky_ethereal_signature::*;

use super::*;

pub(super) fn ethereal_ty_method_disambiguation(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    ty_term: EtherealTerm,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
    // divide into cases for memoization
    match ty_term {
        EtherealTerm::EntityPath(TermEntityPath::TypeOntology(ty_path)) => {
            ethereal_ty_ontology_path_ty_method_disambiguation(engine, expr_idx, ty_path, ident)
        }
        EtherealTerm::Application(ty_term) => {
            ethereal_term_application_ty_method_disambiguation(engine, expr_idx, ty_term, ident)
        }
        _ => Nothing,
    }
}

pub(crate) fn ethereal_ty_ontology_path_ty_method_disambiguation(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    ty_path: TypePath,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
    ethereal_ty_method_disambiguation_aux(engine, expr_idx, ty_path, &[], ident, smallvec![])
}

pub(crate) fn ethereal_term_application_ty_method_disambiguation(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    ty_term: EtherealTermApplication,
    ident: Ident,
) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
    let application_expansion = ty_term.application_expansion(engine.db());
    match application_expansion.function() {
        TermFunctionReduced::TypeOntology(ty_path) => ethereal_ty_method_disambiguation_aux(
            engine,
            expr_idx,
            ty_path,
            application_expansion.arguments(engine.db()),
            ident,
            smallvec![],
        ),
        TermFunctionReduced::Trait(_) | TermFunctionReduced::Other(_) => Nothing,
    }
}

fn ethereal_ty_method_disambiguation_aux(
    engine: &mut impl FluffyTermEngine,
    expr_idx: ExprIdx,
    ty_path: TypePath,
    arguments: &[EtherealTerm],
    ident: Ident,
    mut indirections: SmallVec<[FluffyDotIndirection; 2]>,
) -> FluffyTermMaybeResult<FluffyMethodDisambiguation> {
    match ty_path.refine(engine.db()) {
        Left(PreludeTypePath::Borrow(borrow_ty_path)) => match borrow_ty_path {
            PreludeBorrowTypePath::Ref => todo!(),
            PreludeBorrowTypePath::RefMut => todo!(),
            PreludeBorrowTypePath::Leash => {
                indirections.push(FluffyDotIndirection::Leash);
                if arguments.len() != 1 {
                    p!((&arguments).debug(engine.db()));
                    todo!()
                }
                return JustOk(
                    ethereal_ty_method_disambiguation(engine, expr_idx, arguments[0], ident)?
                        .merge(indirections),
                );
            }
        },
        _ => (),
    }
    if let Some(signature) = ty_method_fluffy_signature(
        engine,
        expr_idx,
        ty_path,
        arguments,
        /* ad hoc */ &[],
        ident,
    )
    .into_result_option()?
    {
        return JustOk(FluffyDotDisambiguation {
            indirections,
            signature,
        });
    };
    if indirections.contains(&FluffyDotIndirection::Leash) {
        todo!()
    }
    // ad hoc
    // needs to consider `Deref` `DerefMut` `Carrier`
    Nothing
}
