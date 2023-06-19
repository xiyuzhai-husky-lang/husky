mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ModuleItemNodeDecl {
    Type(TypeNodeDecl),
    Fugitive(FugitiveNodeDecl),
    Trait(TraitNodeDecl),
}

impl ModuleItemNodeDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ModuleItemNodeDecl::Type(decl) => decl.ast_idx(db),
            ModuleItemNodeDecl::Fugitive(decl) => decl.ast_idx(db),
            ModuleItemNodeDecl::Trait(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            ModuleItemNodeDecl::Type(decl) => decl.implicit_parameters(db),
            ModuleItemNodeDecl::Fugitive(decl) => decl.implicit_parameters(db),
            ModuleItemNodeDecl::Trait(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            ModuleItemNodeDecl::Type(decl) => decl.expr_region(db).into(),
            ModuleItemNodeDecl::Fugitive(decl) => decl.expr_region(db).into(),
            ModuleItemNodeDecl::Trait(decl) => decl.expr_region(db).into(),
        }
    }

    pub fn node_path(self, db: &dyn DeclDb) -> EntityNodePath {
        match self {
            ModuleItemNodeDecl::Type(decl) => decl.node_path(db).into(),
            ModuleItemNodeDecl::Fugitive(decl) => decl.node_path(db).into(),
            ModuleItemNodeDecl::Trait(decl) => decl.node_path(db).into(),
        }
    }
}

impl HasNodeDecl for ModuleItemNodePath {
    type NodeDecl = ModuleItemNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        match self {
            ModuleItemNodePath::Trait(node_path) => node_path.node_decl(db).into(),
            ModuleItemNodePath::Type(node_path) => node_path.node_decl(db).into(),
            ModuleItemNodePath::Fugitive(node_path) => node_path.node_decl(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ModuleItemDecl {
    Type(TypeDecl),
    Trait(TraitDecl),
    Fugitive(FugitiveDecl),
}

impl ModuleItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ModuleItemDecl::Type(decl) => decl.ast_idx(db),
            ModuleItemDecl::Fugitive(decl) => decl.ast_idx(db),
            ModuleItemDecl::Trait(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            ModuleItemDecl::Type(decl) => decl.implicit_parameters(db),
            ModuleItemDecl::Fugitive(decl) => decl.implicit_parameters(db),
            ModuleItemDecl::Trait(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            ModuleItemDecl::Type(decl) => decl.expr_region(db).into(),
            ModuleItemDecl::Fugitive(decl) => decl.expr_region(db).into(),
            ModuleItemDecl::Trait(decl) => decl.expr_region(db).into(),
        }
    }

    pub fn node_path(self, db: &dyn DeclDb) -> EntityNodePath {
        match self {
            ModuleItemDecl::Type(decl) => decl.node_path(db).into(),
            ModuleItemDecl::Fugitive(decl) => decl.node_path(db).into(),
            ModuleItemDecl::Trait(decl) => decl.node_path(db).into(),
        }
    }
}

impl HasDecl for ModuleItemPath {
    type Decl = ModuleItemDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        match self {
            ModuleItemPath::Type(id) => id.decl(db).map(Into::into),
            ModuleItemPath::Trait(id) => id.decl(db).map(Into::into),
            ModuleItemPath::Fugitive(id) => id.decl(db).map(Into::into),
        }
    }
}
