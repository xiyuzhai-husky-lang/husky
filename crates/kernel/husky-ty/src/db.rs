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
                    let implicit_parameters = signature.implicit_parameters(self);
                    let term_menu = self
                        .term_menu(entity_path.toolchain(self))
                        .as_ref()
                        .unwrap();
                    let mut term = term_menu.ty0();
                    for implicit_parameter in implicit_parameters.iter().rev() {
                        todo!()
                    }
                    Ok(term)
                }
                ModuleItemPath::Trait(_) => todo!(),
                ModuleItemPath::Form(_) => todo!(),
            },
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        }
    }
}
