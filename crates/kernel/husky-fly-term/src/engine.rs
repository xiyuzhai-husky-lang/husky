use super::*;
use husky_entity_path::menu::ItemPathMenu;
use husky_entity_tree::helpers::AvailableTraitItemsTable;
use husky_eth_signature::context::{EthSignatureBuilderContext, EthSignatureBuilderContextItd};
use husky_eth_term::term::symbolic_variable::EthSymbolicVariable;
use husky_place::{place::idx::PlaceIdx, PlaceInfo, PlaceRegistry};
use vec_like::SmallVecSet;

pub trait FlyTermEngine<'db>: Sized {
    fn db(&self) -> &'db ::salsa::Db;
    fn available_trai_items_table(&self) -> AvailableTraitItemsTable<'db>;
    fn fly_term_region(&self) -> &FlyTermRegion;
    fn item_path_menu(&self) -> &'db ItemPathMenu;
    fn term_menu(&self) -> &'db EthTermMenu;
    fn syn_expr_region_data(&self) -> &'db SynExprRegionData;
    fn context_itd(&self) -> EthSignatureBuilderContextItd;
    fn context(&self) -> &'db EthSignatureBuilderContext {
        self.context_itd().context(self.db())
    }

    fn fly_terms(&self) -> &FlyTerms {
        self.fly_term_region().terms()
    }

    fn path(&self) -> String {
        format!("{:?}", self.syn_expr_region_data().path().debug(self.db()))
    }

    fn obvious_trais_map(
        &self,
    ) -> &[(
        EthSymbolicVariable,
        Result<SmallVecSet<EthTerm, 2>, EthTermError>,
    )];

    /// final
    fn symbolic_variable_obvious_trais(
        &self,
        svar: EthSymbolicVariable,
    ) -> EthTermResult<&[EthTerm]> {
        use vec_like::VecMapGetEntry;

        match self.obvious_trais_map().get_entry(svar) {
            Some((_, symbolic_variable_obvious_trais)) => match *symbolic_variable_obvious_trais {
                Ok(ref symbolic_variable_obvious_trais) => Ok(symbolic_variable_obvious_trais),
                Err(e) => Err(e),
            },
            None => Ok(&[]),
        }
    }
}

pub trait FlyTermEngineMut<'a>: FlyTermEngine<'a> {
    fn place_registry_mut(&mut self) -> &mut PlaceRegistry;
    fn fly_term_region_mut(&mut self) -> &mut FlyTermRegion;
    fn fly_terms_mut(&mut self) -> &mut FlyTerms {
        self.fly_term_region_mut().terms_mut()
    }

    fn issue_new_place_idx(&mut self, place_data: PlaceInfo) -> PlaceIdx {
        self.place_registry_mut().issue_new(place_data)
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
        debug_assert_eq!(parameter_ty.quary(), None);
        debug_assert_eq!(return_ty.quary(), None);
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
                parameter_hvar: argument_parameter_variable,
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
        expectation: impl ExpectFlyTerm,
    ) -> (FlyTermExpectationIdx, FlyTerm) {
        let db = self.db();
        self.fly_term_region_mut()
            .add_expectation(src, expectee, expectation, db)
    }
}
