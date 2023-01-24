use crate::*;

pub trait TypeDb: salsa::DbWithJar<TypeJar> + SignatureDb {
    fn entity_ty(&self, entipath: EntityPath) -> TypeResultRef<Term>;
}

impl<Db> TypeDb for Db
where
    Db: salsa::DbWithJar<TypeJar> + SignatureDb,
{
    fn entity_ty(&self, entipath: EntityPath) -> TypeResultRef<Term> {
        let term_menu = self.term_menu(entipath.toolchain(self)).as_ref().unwrap();
        match entipath {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(path) => match path {
                ModuleItemPath::Type(path) => ty_entity_ty(self, path).as_ref().map(|t| *t),
                ModuleItemPath::Trait(path) => trai_entity_ty(self, path).as_ref().map(|t| *t),
                ModuleItemPath::Form(path) => form_entity_ty(self, path).as_ref().map(|t| *t),
            },
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        }
    }
}
