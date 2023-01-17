mod associated_item;
mod form;
mod trai;
mod ty;
mod variant;

pub use associated_item::*;
pub use form::*;
pub use trai::*;
pub use ty::*;
pub use variant::*;

use crate::*;
use husky_ast::AstIdx;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Defn {
    Type(TypeDefn),
    Trait(TraitDefn),
    Form(FormDefn),
    Variant(VariantDefn),
    ImplBlock(ImplBlockDecl),
    AssociatedItem(AssociatedItemDefn),
}

impl From<AssociatedItemDefn> for Defn {
    fn from(v: AssociatedItemDefn) -> Self {
        Self::AssociatedItem(v)
    }
}

impl Defn {
    pub fn decl(self, db: &dyn DefnDb) -> Decl {
        match self {
            Defn::Type(defn) => defn.decl(db).into(),
            Defn::Trait(defn) => defn.decl(db).into(),
            Defn::Form(defn) => defn.decl(db).into(),
            Defn::Variant(defn) => defn.decl(db).into(),
            Defn::ImplBlock(decl) => decl.into(),
            Defn::AssociatedItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DefnDb) -> AstIdx {
        self.decl(db).ast_idx(db)
    }

    pub fn implicit_parameters(self, db: &dyn DefnDb) -> &[ImplicitParameterDecl] {
        self.decl(db).implicit_parameters(db)
    }
    pub fn expr_page(self, db: &dyn DefnDb) -> Option<ExprPage> {
        match self {
            Defn::Type(_) | Defn::Trait(_) => None,
            Defn::Form(defn) => Some(defn.expr_page(db)),
            Defn::AssociatedItem(defn) => defn.expr_page(db),
            Defn::Variant(defn) => None,
            Defn::ImplBlock(_) => None,
        }
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
    pub fn path(self, db: &dyn DefnDb) -> Option<EntityPath> {
        match self {
            Defn::Type(defn) => Some(defn.path(db).into()),
            Defn::Trait(defn) => Some(defn.path(db).into()),
            Defn::Form(defn) => Some(defn.path(db).into()),
            Defn::AssociatedItem(defn) => defn.path(db).map(|path| path.into()),
            Defn::Variant(defn) => Some(defn.path(db).into()),
            Defn::ImplBlock(_) => None,
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
            Defn::ImplBlock(decl) => f
                .debug_tuple("ImplBlock")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            Defn::AssociatedItem(decl) => f
                .debug_tuple("AssociatedItem")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
