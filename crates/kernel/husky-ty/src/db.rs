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
        let term_menu = self
            .term_menu(entity_path.toolchain(self))
            .as_ref()
            .unwrap();
        let (implicit_parameters, mut term) = match entity_path {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(path) => match path {
                ModuleItemPath::Type(path) => {
                    let signature = self.ty_signature(path);
                    (signature.implicit_parameters(self), term_menu.ty0())
                }
                ModuleItemPath::Trait(path) => {
                    let signature = self.trai_signature(path);
                    (
                        signature.implicit_parameters(self).as_ref(),
                        term_menu.trai(),
                    )
                }
                ModuleItemPath::Form(path) => match path.form_kind(self) {
                    FormKind::Feature => todo!(),
                    FormKind::Function => todo!(),
                    FormKind::Value => todo!(),
                    FormKind::TypeAlias => todo!(),
                },
            },
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        };
        for implicit_parameter in implicit_parameters.iter().rev() {
            match implicit_parameter.ty() {
                Success(ty) => term = TermCurry::new(self, ty, term).into(),
                Failure(_) => todo!(),
                Abort(_) => todo!(),
            }
        }
        Ok(term)
    }
}
