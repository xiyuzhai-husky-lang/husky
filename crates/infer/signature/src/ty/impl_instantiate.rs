use super::*;
use check_utils::should_eq;

impl TySignature {
    pub fn instantiate(&self, dst_generics: &[GenericArgument]) -> Self {
        should_eq!(self.generics.len(), dst_generics.len());
        Self {
            generics: todo!(),
            traits: self
                .traits
                .iter()
                .map(|t| t.instantiate(&self.generics, dst_generics))
                .collect(),
            members: todo!(),
            kind: todo!(),
        }
    }
}
