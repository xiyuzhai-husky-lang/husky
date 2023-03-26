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
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum Defn {
    Type(TypeDefn),
    Trait(TraitDefn),
    Form(FormDefn),
    Variant(VariantDefn),
    Impl(ImplBlockDecl),
    AssociatedItem(AssociatedItemDefn),
}

impl Defn {
    pub fn decl(self, db: &dyn DefnDb) -> Decl {
        match self {
            Defn::Type(defn) => defn.decl(db).into(),
            Defn::Trait(defn) => defn.decl(db).into(),
            Defn::Form(defn) => defn.decl(db).into(),
            Defn::Variant(defn) => defn.decl(db).into(),
            Defn::Impl(decl) => decl.into(),
            Defn::AssociatedItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DefnDb) -> AstIdx {
        self.decl(db).ast_idx(db)
    }

    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DefnDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        self.decl(db).implicit_parameters(db)
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            Defn::Type(_) | Defn::Trait(_) => None,
            Defn::Form(defn) => Some(defn.expr_region(db)),
            Defn::AssociatedItem(defn) => defn.expr_region(db),
            Defn::Variant(_defn) => None,
            Defn::Impl(_) => None,
        }
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
            Defn::Impl(_) => None,
        }
    }
}
