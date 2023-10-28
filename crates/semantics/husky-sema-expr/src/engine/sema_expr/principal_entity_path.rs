use super::*;
use either::*;
use husky_ethereal_signature::{
    HasEtherealSignatureTemplate, TypeVariantEtherealSignatureTemplate,
};
use husky_regional_token::IdentRegionalToken;
use maybe_result::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_principal_item_path_expr_ty(
        &mut self,
        path: PrincipalEntityPath,
        expr_ty_expectation: &impl ExpectFluffyTerm,
        ty_path_disambiguation: TypePathDisambiguation,
    ) -> SemaExprTypeResult<FluffyTerm> {
        match path {
            PrincipalEntityPath::Module(_) | PrincipalEntityPath::MajorItem(_) => path
                .ty(self.db, ty_path_disambiguation)
                .map(Into::into)
                .map_err(Into::into),
            PrincipalEntityPath::TypeVariant(path) => {
                self.calc_ty_variant_path_expr_ty(path, expr_ty_expectation)
            }
        }
    }

    fn calc_ty_variant_path_expr_ty(
        &mut self,
        path: TypeVariantPath,
        expr_ty_expectation: &impl ExpectFluffyTerm,
    ) -> SemaExprTypeResult<FluffyTerm> {
        let parent_ty_path = path.parent_ty_path(self.db);
        match path.ethereal_signature_template(self.db)? {
            TypeVariantEtherealSignatureTemplate::Props(_) => todo!(),
            TypeVariantEtherealSignatureTemplate::Unit(_) => {
                match expr_ty_expectation.destination() {
                    Some(destination) => match destination.data(self) {
                        FluffyTermData::TypeOntology { ty_path, .. }
                            if ty_path == parent_ty_path =>
                        {
                            Ok(destination)
                        }
                        _ => Ok(path.ty(self.db)?.into()),
                    },
                    None => Ok(path.ty(self.db)?.into()),
                }
            }
            TypeVariantEtherealSignatureTemplate::Tuple(_) => Ok(path.ty(self.db)?.into()),
        }
    }
}
