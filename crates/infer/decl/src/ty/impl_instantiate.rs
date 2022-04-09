use super::*;
use check_utils::should_eq;

impl TyDecl {
    pub fn instantiate(&self, db: &dyn DeclQueryGroup, dst_generics: &[GenericArgument]) -> Self {
        should_eq!(self.generic_placeholders.len(), dst_generics.len());
        let instantiator = Instantiator {
            db: db.upcast(),
            generic_placeholders: &self.generic_placeholders,
            dst_generics,
        };
        todo!()
        // let kind = match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         methods: ref field_routines,
        //     } => todo!(),
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => todo!(),
        //     TyKind::Vec { element_ty } => TyKind::Vec {
        //         element_ty: instantiator.instantiate_entity_route(element_ty).as_scope(),
        //     },
        // };
        // Self {
        //     this_type: instantiator
        //         .instantiate_entity_route(self.this_type)
        //         .as_scope(),
        //     generic_placeholders: Default::default(),
        //     traits: self
        //         .traits
        //         .iter()
        //         .map(|t| instantiator.instantiate_entity_route(*t).as_scope())
        //         .collect(),
        //     members: self
        //         .members
        //         .iter()
        //         .map(|(ident, signature)| (*ident, signature.instantiate(&instantiator)))
        //         .collect(),
        //     kind,
        // }
    }
}
