use crate::*;
use avec::Avec;

impl EntityDefnVariant {
    pub fn subentities(&self) -> Avec<EntityDefn> {
        match self {
            EntityDefnVariant::Main(_) => todo!(),
            EntityDefnVariant::Module {} => todo!(),
            EntityDefnVariant::Feature { ty, lazy_stmts } => todo!(),
            EntityDefnVariant::Pattern {} => todo!(),
            EntityDefnVariant::TypeField { .. }
            | EntityDefnVariant::TypeMethod { .. }
            | EntityDefnVariant::TraitMethod { .. }
            | EntityDefnVariant::Func { .. }
            | EntityDefnVariant::Proc { .. } => Arc::new(Vec::new()),
            EntityDefnVariant::Type {
                type_members,
                variants,
                kind,
                trait_impls,
                members,
            } => members.clone(),
            EntityDefnVariant::EnumVariant { .. } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TraitMethodImpl { .. } => todo!(),
        }
    }
}
