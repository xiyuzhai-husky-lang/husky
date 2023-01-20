use crate::*;
use husky_entity_path::{EntityPath, ModuleItemPath};

pub trait TypeDb: salsa::DbWithJar<TypeJar> + SignatureDb {
    fn entity_ty(&self, entity_path: EntityPath) -> TypeResult<Term>;
}

impl<Db> TypeDb for Db
where
    Db: salsa::DbWithJar<TypeJar> + SignatureDb,
{
    fn entity_ty(&self, entity_path: EntityPath) -> TypeResult<Term> {
        match entity_path {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(path) => match path {
                ModuleItemPath::Type(path) => {
                    let signature = self.ty_signature(path);
                    todo!()
                }
                ModuleItemPath::Trait(_) => todo!(),
                ModuleItemPath::Form(_) => todo!(),
            },
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        }
    }
}
