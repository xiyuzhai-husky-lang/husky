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
pub enum ModuleItemSynNodeDecl {
    Type(TypeNodeDecl),
    Fugitive(FugitiveNodeDecl),
    Trait(TraitSynNodeDecl),
}

impl ModuleItemSynNodeDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            ModuleItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.ast_idx(db),
            ModuleItemSynNodeDecl::Fugitive(syn_node_decl) => syn_node_decl.ast_idx(db),
            ModuleItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.ast_idx(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> SynExprRegion {
        match self {
            ModuleItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.expr_region(db).into(),
            ModuleItemSynNodeDecl::Fugitive(syn_node_decl) => syn_node_decl.expr_region(db).into(),
            ModuleItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.expr_region(db).into(),
        }
    }

    pub fn syn_node_path(self, db: &dyn DeclDb) -> EntitySynNodePath {
        match self {
            ModuleItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ModuleItemSynNodeDecl::Fugitive(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            ModuleItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
        }
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        match self {
            ModuleItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.errors(db),
            ModuleItemSynNodeDecl::Fugitive(syn_node_decl) => syn_node_decl.errors(db),
            ModuleItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasNodeDecl for ModuleItemSynNodePath {
    type NodeDecl = ModuleItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        match self {
            ModuleItemSynNodePath::Trait(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ModuleItemSynNodePath::Type(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ModuleItemSynNodePath::Fugitive(syn_node_path) => {
                syn_node_path.syn_node_decl(db).into()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum ModuleItemDecl {
    Type(TypeDecl),
    Trait(TraitSynDecl),
    Fugitive(FugitiveDecl),
}

impl ModuleItemDecl {
    pub fn generic_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            ModuleItemDecl::Type(decl) => decl.generic_parameters(db),
            ModuleItemDecl::Fugitive(decl) => decl.generic_parameters(db),
            ModuleItemDecl::Trait(decl) => decl.generic_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> SynExprRegion {
        match self {
            ModuleItemDecl::Type(decl) => decl.expr_region(db).into(),
            ModuleItemDecl::Fugitive(decl) => decl.expr_region(db).into(),
            ModuleItemDecl::Trait(decl) => decl.expr_region(db).into(),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> ModuleItemPath {
        match self {
            ModuleItemDecl::Type(decl) => decl.path(db).into(),
            ModuleItemDecl::Fugitive(decl) => decl.path(db).into(),
            ModuleItemDecl::Trait(decl) => decl.path(db).into(),
        }
    }
}

impl HasDecl for ModuleItemPath {
    type Decl = ModuleItemDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        match self {
            ModuleItemPath::Type(id) => id.decl(db).map(Into::into),
            ModuleItemPath::Trait(id) => id.decl(db).map(Into::into),
            ModuleItemPath::Fugitive(id) => id.decl(db).map(Into::into),
        }
    }
}
