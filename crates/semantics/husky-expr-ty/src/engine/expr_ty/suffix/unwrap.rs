use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_unwrap_expr_ty(&mut self, opd_ty: FluffyTerm) -> ExprTypeResult<FluffyTerm> {
        match opd_ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path,
                refined_ty_path,
                arguments,
                ty_ethereal_term,
            } => match refined_ty_path {
                Left(PreludeTypePath::Option | PreludeTypePath::Result) => Ok(arguments[0]),
                _ => Err(OriginalExprTypeError::CannotUnwrap)?,
            },
            FluffyTermData::TypeOntologyAtPlace {
                ty_path,
                refined_ty_path,
                ty_arguments: arguments,
                base_ty_ethereal_term,
                place,
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::HoleAtPlace {
                hole_kind,
                hole,
                place,
            } => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FluffyTermData::Symbol { term, ty } => todo!(),
            FluffyTermData::SymbolAtPlace { term, place } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
