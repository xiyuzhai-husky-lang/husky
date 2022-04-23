use crate::*;
use avec::Avec;

impl EntityDefnVariant {
    pub fn subentities(&self) -> Avec<EntityDefn> {
        match self {
            EntityDefnVariant::Main(_) => todo!(),
            EntityDefnVariant::Module {} => todo!(),
            EntityDefnVariant::Feature { .. }
            | EntityDefnVariant::Pattern {}
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
        }
    }
}
