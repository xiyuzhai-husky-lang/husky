use husky_ethereal_signature::{
    HasEtherealSignatureTemplate, TypeVariantEtherealSignatureTemplate,
};

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_principal_entity_path_expr_ty(
        &mut self,
        path: Option<PrincipalEntityPath>,
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        let disambiguation = expr_ty_expectation.disambiguate_ty_path(self);
        let path = path.ok_or(DerivedExprTypeError::EntityPathError)?;
        let ty_result = match path {
            PrincipalEntityPath::Module(_) | PrincipalEntityPath::ModuleItem(_) => {
                Ok(path.ty(self.db, disambiguation)?.into())
            }
            PrincipalEntityPath::TypeVariant(path) => {
                self.calc_ty_variant_path_expr_ty(path, expr_ty_expectation)
            }
        };
        Ok((disambiguation.into(), ty_result))
    }

    fn calc_ty_variant_path_expr_ty(
        &mut self,
        path: TypeVariantPath,
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> ExprTypeResult<FluffyTerm> {
        Ok(match path.ethereal_signature_template(self.db)? {
            TypeVariantEtherealSignatureTemplate::Props(_) => todo!(),
            TypeVariantEtherealSignatureTemplate::Unit(_) => todo!(),
            TypeVariantEtherealSignatureTemplate::Tuple(_) => todo!(),
        })
    }
}
