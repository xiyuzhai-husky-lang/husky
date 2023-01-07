mod form;
mod trai;
mod trai_item;
mod ty;
mod ty_item;
mod variant;

pub use form::*;
pub use trai::*;
pub use trai_item::*;
pub use ty::*;
pub use ty_item::*;
pub use variant::*;

use crate::*;
use husky_ast::AstIdx;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Defn {
    Type(TypeDefn),
    Trait(TraitDefn),
    Form(FormDefn),
    Variant(VariantDefn),
    TypeItem(TypeItemDefn),
    TraitItem(TraitItemDefn),
}

impl Defn {
    pub fn decl(self, db: &dyn DefnDb) -> Decl {
        match self {
            Defn::Type(defn) => defn.decl(db).into(),
            Defn::Trait(defn) => defn.decl(db).into(),
            Defn::Form(defn) => defn.decl(db).into(),
            Defn::TypeItem(defn) => defn.decl(db).into(),
            Defn::TraitItem(defn) => defn.decl(db).into(),
            Defn::Variant(defn) => defn.decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DefnDb) -> AstIdx {
        self.decl(db).ast_idx(db)
    }

    pub fn implicit_parameters(self, db: &dyn DefnDb) -> &[ImplicitParameterDecl] {
        self.decl(db).implicit_parameters(db)
    }
    pub fn expr_sheet(self, db: &dyn DefnDb) -> Option<ExprSheet> {
        match self {
            Defn::Type(_) | Defn::Trait(_) => None,
            Defn::Form(defn) => Some(defn.expr_sheet(db).into()),
            Defn::TypeItem(defn) => Some(defn.expr_sheet(db).into()),
            Defn::TraitItem(defn) => Some(defn.expr_sheet(db).into()),
            Defn::Variant(defn) => None,
        }
    }
}

impl From<TraitItemDefn> for Defn {
    fn from(v: TraitItemDefn) -> Self {
        Self::TraitItem(v)
    }
}

impl From<TypeItemDefn> for Defn {
    fn from(v: TypeItemDefn) -> Self {
        Self::TypeItem(v)
    }
}

impl From<FormDefn> for Defn {
    fn from(v: FormDefn) -> Self {
        Self::Form(v)
    }
}

impl From<TraitDefn> for Defn {
    fn from(v: TraitDefn) -> Self {
        Self::Trait(v)
    }
}

impl From<TypeDefn> for Defn {
    fn from(v: TypeDefn) -> Self {
        Self::Type(v)
    }
}

impl Defn {
    pub fn path(self, db: &dyn DefnDb) -> EntityPath {
        match self {
            Defn::Type(defn) => defn.path(db).into(),
            Defn::Trait(defn) => defn.path(db).into(),
            Defn::Form(defn) => defn.path(db).into(),
            Defn::TypeItem(defn) => defn.path(db).into(),
            Defn::TraitItem(defn) => defn.path(db).into(),
            Defn::Variant(defn) => defn.path(db).into(),
        }
    }
}

impl<Db: DefnDb + ?Sized> salsa::DebugWithDb<Db> for Defn {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<DefnJar>>::as_jar_db(db);
        match self {
            Defn::Type(decl) => f
                .debug_tuple("Type")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Defn::Trait(decl) => f
                .debug_tuple("Trait")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Defn::Form(decl) => f
                .debug_tuple("Form")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Defn::Variant(decl) => f
                .debug_tuple("Variant")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Defn::TypeItem(decl) => f
                .debug_tuple("TypeItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Defn::TraitItem(decl) => f
                .debug_tuple("TraitItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
