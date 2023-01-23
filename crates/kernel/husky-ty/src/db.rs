use crate::*;
use husky_entity_path::{EntityPath, ModuleItemPath};

pub trait TypeDb: salsa::DbWithJar<TypeJar> + SignatureDb {
    fn entity_ty(&self, entity_path: EntityPath) -> TypeResultRef<Term>;
}

impl<Db> TypeDb for Db
where
    Db: salsa::DbWithJar<TypeJar> + SignatureDb,
{
    fn entity_ty(&self, entity_path: EntityPath) -> TypeResultRef<Term> {
        let term_menu = self
            .term_menu(entity_path.toolchain(self))
            .as_ref()
            .unwrap();
        match entity_path {
            EntityPath::Module(_) => todo!(),
            EntityPath::ModuleItem(path) => match path {
                ModuleItemPath::Type(path) => ty_entity_ty(self, path).as_ref().map(|t| *t),
                ModuleItemPath::Trait(path) => trai_entity_ty(self, path).as_ref().map(|t| *t),
                ModuleItemPath::Form(path) => match path.form_kind(self) {
                    FormKind::Feature => todo!(),
                    FormKind::Function => todo!(),
                    FormKind::Value => todo!(),
                    FormKind::TypeAlias => todo!(),
                },
            },
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn ty_entity_ty(db: &dyn TypeDb, ty_path: TypePath) -> TypeResult<Term> {
    let term_menu = db.term_menu(ty_path.toolchain(db)).as_ref().unwrap();
    let signature = match db.ty_signature(ty_path) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    Ok(curry_from_implicit_parameter_tys(
        db,
        signature.implicit_parameters(db),
        term_menu.ty0(),
    ))
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn trai_entity_ty(db: &dyn TypeDb, trai_path: TraitPath) -> TypeResult<Term> {
    let term_menu = db.term_menu(trai_path.toolchain(db)).as_ref().unwrap();
    let signature = match db.trai_signature(trai_path) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    Ok(curry_from_implicit_parameter_tys(
        db,
        signature.implicit_parameters(db),
        term_menu.ty0(),
    ))
}

fn curry_from_implicit_parameter_tys(
    db: &dyn TypeDb,
    implicit_parameters: &[ImplicitParameterSignature],
    mut term: Term,
) -> Term {
    for implicit_parameter in implicit_parameters.iter().rev() {
        term = TermCurry::new(db, implicit_parameter.ty(), term).into()
    }
    term
}
