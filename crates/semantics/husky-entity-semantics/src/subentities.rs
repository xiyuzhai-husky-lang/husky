use crate::*;
use avec::Avec;

impl EntityDefnVariant {
    pub fn subentities(&self) -> Avec<EntityDefn> {
        match self {
            EntityDefnVariant::Module {
                ref module_items, ..
            } => module_items.clone(),
            EntityDefnVariant::Feature { .. }
            | EntityDefnVariant::TyField { .. }
            | EntityDefnVariant::Func { .. }
            | EntityDefnVariant::Proc { .. }
            | EntityDefnVariant::Method { .. }
            | EntityDefnVariant::Function { .. }
            | EntityDefnVariant::Input { .. } => Default::default(),
            EntityDefnVariant::Ty { members, .. } => members.clone(),
            EntityDefnVariant::EnumVariant {
                enum_variant_defn_variant: ref variant,
                ..
            } => match variant {
                EnumVariantDefnVariant::Constant => Default::default(),
            },
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => Arc::new(Vec::new()),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::Trait { ref members, .. } => Arc::new(members.data().to_vec()),
            EntityDefnVariant::Any => Arc::new(Vec::new()),
        }
    }
}
