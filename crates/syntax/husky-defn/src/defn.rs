mod associated_item;
mod fugitive;
mod impl_block;
mod trai;
mod ty;
mod variant;

pub use self::associated_item::*;
pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;
pub use self::variant::*;

use crate::*;
use husky_ast::AstIdx;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum Defn {
    Type(TypeDefn),
    Trait(TraitDefn),
    Fugitive(FugitiveDefn),
    Variant(VariantDefn),
    ImplBlock(ImplBlockDecl),
    AssociatedItem(AssociatedItemDefn),
}

impl Defn {
    pub fn decl(self, db: &dyn DefnDb) -> Decl {
        match self {
            Defn::Type(defn) => defn.decl(db).into(),
            Defn::Trait(defn) => defn.decl(db).into(),
            Defn::Fugitive(defn) => defn.decl(db).into(),
            Defn::Variant(defn) => defn.decl(db).into(),
            Defn::ImplBlock(decl) => decl.into(),
            Defn::AssociatedItem(defn) => defn.decl(db).into(),
        }
    }

    pub fn ast_idx(self, db: &dyn DefnDb) -> AstIdx {
        self.decl(db).ast_idx(db)
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DefnDb) -> &'a [ImplicitParameterDeclPattern] {
        self.decl(db).implicit_parameters(db)
    }

    pub fn expr_region(self, db: &dyn DefnDb) -> Option<ExprRegion> {
        match self {
            Defn::Type(_) | Defn::Trait(_) => None,
            Defn::Fugitive(defn) => Some(defn.expr_region(db)),
            Defn::AssociatedItem(defn) => defn.expr_region(db),
            Defn::Variant(_defn) => None,
            Defn::ImplBlock(_) => None,
        }
    }
}

impl Defn {
    pub fn path(self, db: &dyn DefnDb) -> Option<EntityPath> {
        match self {
            Defn::Type(defn) => Some(defn.path(db).into()),
            Defn::Trait(defn) => Some(defn.path(db).into()),
            Defn::Fugitive(defn) => Some(defn.path(db).into()),
            Defn::AssociatedItem(defn) => defn.path(db).map(|path| path.into()),
            Defn::Variant(defn) => Some(defn.path(db).into()),
            Defn::ImplBlock(_) => None,
        }
    }
}

pub trait HasDefn: Copy {
    type Defn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn;
}

impl HasDefn for Decl {
    type Defn = Defn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        match self {
            Decl::Type(decl) => decl.defn(db).into(),
            Decl::Form(decl) => decl.defn(db).into(),
            Decl::Trait(decl) => decl.defn(db).into(),
            Decl::ImplBlock(decl) => decl.defn(db).into(),
            Decl::AssociatedItem(decl) => decl.defn(db).into(),
            Decl::TypeVariant(_) => todo!(),
        }
    }
}
