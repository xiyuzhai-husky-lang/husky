use super::*;
use husky_entity_tree::helpers::TraitInUseItemsTable;
use husky_place::{PlaceIdx, PlaceRegistry};

pub trait FlyTermEngine<'a>: Sized {
    fn db(&self) -> &'a ::salsa::Db;
    fn trai_in_use_items_table(&self) -> TraitInUseItemsTable<'a>;
    fn fly_term_region(&self) -> &FlyTermRegion;
    fn item_path_menu(&self) -> &'a ItemPathMenu;
    fn term_menu(&self) -> &'a EthTermMenu;
    fn expr_region_data(&self) -> &'a SynExprRegionData;

    fn fly_terms(&self) -> &FlyTerms {
        self.fly_term_region().terms()
    }

    fn path(&self) -> String {
        format!("{:?}", self.expr_region_data().path().debug(self.db()))
    }
}

pub trait FlyTermEngineMut<'a>: FlyTermEngine<'a> {
    fn stack_location_registry_mut(&mut self) -> &mut PlaceRegistry;
    fn fly_term_region_mut(&mut self) -> &mut FlyTermRegion;
    fn fly_terms_mut(&mut self) -> &mut FlyTerms {
        self.fly_term_region_mut().terms_mut()
    }

    fn issue_new_stack_location_idx(&mut self) -> PlaceIdx {
        self.stack_location_registry_mut().issue_new()
    }

    fn new_hole(&mut self, src: impl Into<HoleSource>, hole_kind: HoleKind) -> FlyTerm {
        HolTerm::new_hole(self, src, hole_kind).into()
    }

    fn synthesize_function_application_expr_ty(
        &mut self,
        variance: Variance,
        parameter_hvar: Option<FlyHvar>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
        argument_ty: FlyTerm,
        shift: i8,
    ) -> FlyTermResult<FlyTerm> {
        debug_assert!(shift >= 0);
        if shift == 0 {
            return Ok(return_ty);
        }
        debug_assert_eq!(parameter_ty.place(), None);
        debug_assert_eq!(return_ty.place(), None);
        match (
            parameter_hvar.map(|hvar| hvar.base_resolved(self)),
            parameter_ty.base_resolved(self),
            return_ty.base_resolved(self),
            argument_ty.base_resolved(self),
        ) {
            (
                None,
                FlyTermBase::Eth(parameter_ty),
                FlyTermBase::Eth(return_ty),
                FlyTermBase::Eth(argument_ty),
            ) => {
                return Ok(EthTerm::synthesize_function_application_expr_ty(
                    self.db(),
                    variance,
                    None,
                    parameter_ty,
                    return_ty,
                    argument_ty,
                    shift,
                )?
                .into())
            }
            _ => (),
        }
        if parameter_hvar.is_some() {
            todo!()
        }
        match argument_ty.data(self) {
            FlyTermData::Curry {
                curry_kind: argument_curry_kind,
                variance: argument_variance,
                parameter_hvar: argument_parameter_variable,
                parameter_ty: argument_parameter_ty,
                return_ty: argument_return_ty,
                ..
            } => {
                if argument_parameter_variable.is_some() {
                    todo!()
                }
                let expr_ty = self.synthesize_function_application_expr_ty(
                    variance,
                    parameter_hvar,
                    parameter_ty,
                    return_ty,
                    argument_return_ty,
                    shift - 1,
                );
                todo!()
                // FlyTerm::new_curry()
            }
            _ => unreachable!(),
        }
    }

    fn add_expectation(
        &mut self,
        src: ExpectationSource,
        expectee: FlyTerm,
        expectation: impl Into<Expectation>,
    ) -> (FlyTermExpectationIdx, FlyTerm) {
        let db = self.db();
        self.fly_term_region_mut()
            .add_expectation(src, expectee, expectation, db)
    }
}
