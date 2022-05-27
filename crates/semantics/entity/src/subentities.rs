use crate::*;
use avec::Avec;

impl EntityDefnVariant {
    pub fn subentities(&self) -> Avec<EntityDefn> {
        match self {
            EntityDefnVariant::Main(_) => todo!(),
            EntityDefnVariant::Module { ref module_items } => module_items.clone(),
            EntityDefnVariant::Feature { .. }
            | EntityDefnVariant::TypeField { .. }
            | EntityDefnVariant::Func { .. }
            | EntityDefnVariant::Proc { .. }
            | EntityDefnVariant::Method { .. } => Arc::new(Vec::new()),
            EntityDefnVariant::Type { members, .. } => members.clone(),
            EntityDefnVariant::EnumVariant { ref variant, .. } => match variant {
                EnumVariantDefnVariant::Constant => Default::default(),
            },
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => Arc::new(Vec::new()),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::Trait { ref members, .. } => Arc::new(members.data().to_vec()),
        }
    }
}
