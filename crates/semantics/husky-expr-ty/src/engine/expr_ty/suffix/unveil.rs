use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_unveil_expr_ty(&mut self, opd: ExprIdx) -> ExprTypeResult<FluffyTerm> {
        let Some(ref unveiler) = self.unveiler else {
            Err(DerivedExprTypeError::UnableToInferReturnTypeForUnveiling)?
        };
        match unveiler {
            Unveiler::Unique(_) => todo!(),
        }
        // Err(OriginalExprTypeError::CannotUnveil)?
    }
}

pub(crate) enum Unveiler {
    Unique(EtherealTerm),
}

impl Unveiler {
    pub(crate) fn new(db: &dyn ExprTypeDb, return_ty: EtherealTerm) -> Self {
        // let templates = return_ty.unveil_ethereal_signature_templates(db);
        todo!()
    }
}
