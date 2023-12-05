//! coercing `@leashed T` to `~T`
//!
//!
use super::*;

impl ExpectCoersion {
    pub(super) fn resolve_place_to_prelude_indirection(
        &self,
        db: &::salsa::Db,
        terms: &FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
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
        let expected_base_ty_data = self.ty_expected.base_ty_data_inner(db, terms) else {
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
                    // todo: check place
                    try_finalize_coersion(
                        state.expectee(),
                        expected_ty_arguments[0],
                        Coersion::PlaceToLeash,
                        db,
                        terms,
                        state,
                    )
                }
                PreludeIndirectionTypePath::At => todo!(),
            },
            _ => AltNone,
        }
    }
}
