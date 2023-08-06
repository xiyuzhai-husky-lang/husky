//! coercing `@leashed T` to `~T`
//!
//!
use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_place_to_prelude_indirection(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<ExpectationEffect> {
        // todo: check contract
        // match self.contract {
        //     Contract::None => todo!(),
        //     Contract::Move => todo!(),
        //     Contract::Borrow => todo!(),
        //     Contract::BorrowMut => todo!(),
        //     Contract::Const => todo!(),
        //     Contract::Leash => todo!(),
        // }
        // todo: assert that is None
        // let (None, expected_base_ty_data) = self.ty_expected.ty_data_inner(db, terms) else {
        let (expected_place, expected_base_ty_data) = self.ty_expected.ty_data_inner(db, terms)
        else {
            unreachable!(
                "place should be merged with contract already, self.ty_expected = {}",
                self.ty_expected.show(db, terms)
            )
        };
        match expected_base_ty_data {
            FluffyBaseTypeData::TypeOntology {
                ty_path,
                refined_ty_path: Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)),
                ty_arguments: expected_ty_arguments,
                ty_ethereal_term,
            } => match prelude_indirection_ty_path {
                PreludeIndirectionTypePath::Ref => todo!(),
                PreludeIndirectionTypePath::RefMut => todo!(),
                PreludeIndirectionTypePath::Leash => {
                    debug_assert_eq!(expected_ty_arguments.len(), 1);
                    let (dst_place, dst) = expected_ty_arguments[0].ty_data_inner(db, terms);
                    // todo: check place
                    resolve_aux(
                        state.expectee(),
                        expected_ty_arguments[0],
                        |_,_|/* ad hoc */ Some(Coersion::PlaceToLeash),
                        db,
                        terms,
                        state,
                    )
                }
            },
            _ => AltNone,
        }
    }
}
// FluffyTermData::Literal(_) => todo!(),
// FluffyTermData::TypeOntology {
//     ty_path,
//     refined_ty_path: Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)),
//     arguments,
//     ty_ethereal_term,
// } => match prelude_indirection_ty_path {
//     PreludeIndirectionTypePath::Ref => todo!(),
//     PreludeIndirectionTypePath::RefMut => todo!(),
//     PreludeIndirectionTypePath::Leash => todo!(),
// },
// FluffyTermData::TypeOntologyAtPlace { .. } => unreachable!(),
// _ => todo!(),

// FluffyTermData::TypeOntologyAtPlace {
//     ty_path: src_ty_path,
//     refined_ty_path: Left(PreludeTypePath::Indirection(src_indirection_ty_path)),
//     ty_arguments: src_ty_arguments,
//     ..
// } => match src_indirection_ty_path {
//     PreludeIndirectionTypePath::Ref => todo!(),
//     PreludeIndirectionTypePath::RefMut => todo!(),
//     PreludeIndirectionTypePath::Leash => {
//         debug_assert_eq!(src_ty_arguments.len(), 1);
//         match src_ty_arguments[0].data_inner(db, fluffy_terms) {
//             FluffyTermData::Literal(_) => todo!(),
//             FluffyTermData::TypeOntology {
//                 ty_path,
//                 refined_ty_path,
//                 arguments,
//                 ty_ethereal_term,
//             } => todo!(),
//             // self.resolve_convertible_to_ty_ontology_aux(
//             //     db,
//             //     state,
//             //     fluffy_terms,
//             //     FluffyTermData::TypeOntologyAtPlace {
//             //         ty_path,
//             //         refined_ty_path,
//             //         ty_arguments: arguments,
//             //         base_ty_ethereal_term: ty_ethereal_term,
//             //         place: Place::Leashed,
//             //     },
//             //     dst_path,
//             //     dst_refined_ty_path,
//             //     dst_ty_arguments,
//             // ),
//             FluffyTermData::TypeOntologyAtPlace {
//                 ty_path,
//                 refined_ty_path,
//                 ty_arguments: arguments,
//                 base_ty_ethereal_term,
//                 place,
//             } => todo!(),
//             FluffyTermData::Curry {
//                 curry_kind,
//                 variance,
//                 parameter_variable,
//                 parameter_ty,
//                 return_ty,
//                 ty_ethereal_term,
//             } => todo!(),
//             FluffyTermData::Hole(_, _) => todo!(),
//             FluffyTermData::HoleAtPlace {
//                 hole_kind,
//                 hole,
//                 place,
//             } => todo!(),
//             FluffyTermData::Category(_) => todo!(),
//             FluffyTermData::Ritchie {
//                 ritchie_kind,
//                 parameter_contracted_tys,
//                 return_ty,
//             } => todo!(),
//             FluffyTermData::Symbol { term, ty } => todo!(),
//             FluffyTermData::SymbolAtPlace { term, place } => todo!(),
//             FluffyTermData::Variable { ty } => todo!(),
//             FluffyTermData::TypeVariant { path } => todo!(),
//         }
//     }
// },
// FluffyTermData::TypeOntologyAtPlace {
//     ty_path: src_path,
//     refined_ty_path: src_refined_ty_path,
//     ty_arguments: src_ty_arguments,
//     ..
// } => {
//     // at this stage, we should already rule out the case dst_ty_path is prelude indirection
//     // todo: consider `Deref` and `DerefMut`
//     state.set_err(
//         OriginalFluffyTermExpectationError::TypePathMismatchForCoersion {
//             contract: todo!(),
//             ty_expected: todo!(),
//             expectee: todo!(),
//             expected_path: dst_path,
//             expectee_path: src_path,
//         },
//         smallvec![],
//     )
// }
