use super::*;
use check_utils::should_eq;

impl TySignature {
    pub fn instantiate(
        &self,
        db: &dyn InferSignatureQueryGroup,
        dst_generics: &[GenericArgument],
    ) -> Self {
        should_eq!(self.generic_placeholders.len(), dst_generics.len());
        let instantiator = Instantiator {
            db: db.upcast(),
            src_generic_placeholders: &self.generic_placeholders,
            dst_generics,
        };
        let kind = match self.kind {
            TySignatureKind::Struct {
                ref memb_vars,
                ref memb_routines,
            } => todo!(),
            TySignatureKind::Enum { ref variants } => todo!(),
            TySignatureKind::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
            TySignatureKind::Vec { element_ty } => TySignatureKind::Vec {
                element_ty: instantiator.instantiate_scope(element_ty).as_scope(),
            },
        };
        Self {
            generic_placeholders: Default::default(),
            traits: self
                .traits
                .iter()
                .map(|t| instantiator.instantiate_scope(*t).as_scope())
                .collect(),
            members: self
                .members
                .iter()
                .map(|(ident, signature)| (*ident, signature.instantiate(&instantiator)))
                .collect(),
            kind,
        }
    }
}
