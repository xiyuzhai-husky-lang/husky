use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_suffix_expr_ty(
        &mut self,
        opd: ExprIdx,
        opr: SuffixOpr,
        final_destination: FinalDestination,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        match opr {
            SuffixOpr::Incr => Ok((ExprDisambiguation::Trivial, self.calc_incr_expr_ty(opd))),
            SuffixOpr::Decr => Ok((ExprDisambiguation::Trivial, self.calc_decr_expr_ty(opd))),
            SuffixOpr::UnveilOrComposeWithOption => self.calc_ambiguous_suffix_expr_ty(
                opd,
                final_destination,
                (
                    UnveilOrComposeWithOptionExprDisambiguation::Unveil,
                    Self::calc_unveil_expr_ty,
                ),
                (
                    UnveilOrComposeWithOptionExprDisambiguation::ComposeWithOption,
                    Self::calc_compose_with_option_expr_ty,
                ),
            ),
            SuffixOpr::UnwrapOrComposeWithNot => self.calc_ambiguous_suffix_expr_ty(
                opd,
                final_destination,
                (
                    UnwrapOrComposeWithNotExprDisambiguation::Unwrap,
                    Self::calc_unveil_expr_ty,
                ),
                (
                    UnwrapOrComposeWithNotExprDisambiguation::ComposeWithNot,
                    Self::calc_compose_with_option_expr_ty,
                ),
            ),
        }
    }

    fn calc_incr_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        let opd_ty = self
            .infer_new_expr_ty(opd, ExpectAnyOriginal)
            .ok_or(DerivedExprTypeError::SuffixOperandTypeNotInferred)?;
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
        Ok(self.term_menu.unit_ty_ontology().into())
    }

    fn calc_decr_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        let opd_ty = self
            .infer_new_expr_ty(opd, ExpectAnyOriginal)
            .ok_or(DerivedExprTypeError::SuffixOperandTypeNotInferred)?;
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
        Ok(self.term_menu.unit_ty_ontology().into())
    }

    fn calc_ambiguous_suffix_expr_ty<D: Into<ExprDisambiguation>>(
        &mut self,
        opd: ExprIdx,
        final_destination: FinalDestination,
        (left_disambiguation, left_f): (D, fn(&mut Self, ExprIdx) -> ExprTypeResult<FluffyTerm>),
        (right_disambiguation, right_f): (D, fn(&mut Self, ExprIdx) -> ExprTypeResult<FluffyTerm>),
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        match self.infer_new_expr_ty(
            opd,
            ExpectAnyTowardsFinalDestination::new(final_destination),
        ) {
            Some(_) => todo!(),
            None => Err(DerivedExprTypeError::UnableToInferSuffixOperandType.into()),
        }
    }

    fn calc_unveil_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        todo!()
    }

    fn calc_compose_with_option_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        let opd_ty = self
            .infer_new_expr_ty(
                opd,
                ExpectEqsFunctionType::new(FinalDestination::TypeOntology),
            )
            .ok_or(DerivedExprTypeError::SuffixOperandTypeNotInferred)?;
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                path,
                refined_path,
                arguments,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::PlaceTypeOntology {
                place,
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                base_ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::PlaceHole {
                place,
                hole_kind,
                hole,
            } => todo!(),
        }
    }
}
