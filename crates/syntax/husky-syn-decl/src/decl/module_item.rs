mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum ModuleItemSynNodeDecl {
    Type(TypeSynNodeDecl),
    Fugitive(FugitiveSynNodeDecl),
    Trait(TraitSynNodeDecl),
}

impl ModuleItemSynNodeDecl {
    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            ModuleItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.ast_idx(db),
            ModuleItemSynNodeDecl::Fugitive(syn_node_decl) => syn_node_decl.ast_idx(db),
            ModuleItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.ast_idx(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            ModuleItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
            ModuleItemSynNodeDecl::Fugitive(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db).into()
            }
            ModuleItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
        }
    }

    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> EntitySynNodePath {
        match self {
            ModuleItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ModuleItemSynNodeDecl::Fugitive(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            ModuleItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        match self {
            ModuleItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.errors(db),
            ModuleItemSynNodeDecl::Fugitive(syn_node_decl) => syn_node_decl.errors(db),
            ModuleItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasNodeDecl for ModuleItemSynNodePath {
    type NodeDecl = ModuleItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
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
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum ModuleItemSynDecl {
    Type(TypeSynDecl),
    Trait(TraitSynDecl),
    Fugitive(FugitiveSynDecl),
}

impl ModuleItemSynDecl {
    pub fn generic_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [GenericParameterDecl] {
        match self {
            ModuleItemSynDecl::Type(decl) => decl.generic_parameters(db),
            ModuleItemSynDecl::Fugitive(decl) => decl.generic_parameters(db),
            ModuleItemSynDecl::Trait(decl) => decl.generic_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            ModuleItemSynDecl::Type(decl) => decl.syn_expr_region(db).into(),
            ModuleItemSynDecl::Fugitive(decl) => decl.syn_expr_region(db).into(),
            ModuleItemSynDecl::Trait(decl) => decl.syn_expr_region(db).into(),
        }
    }

    pub fn path(self, db: &dyn SynDeclDb) -> ModuleItemPath {
        match self {
            ModuleItemSynDecl::Type(decl) => decl.path(db).into(),
            ModuleItemSynDecl::Fugitive(decl) => decl.path(db).into(),
            ModuleItemSynDecl::Trait(decl) => decl.path(db).into(),
        }
    }
}

impl HasSynDecl for ModuleItemPath {
    type Decl = ModuleItemSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        match self {
            ModuleItemPath::Type(id) => id.syn_decl(db).map(Into::into),
            ModuleItemPath::Trait(id) => id.syn_decl(db).map(Into::into),
            ModuleItemPath::Fugitive(id) => id.syn_decl(db).map(Into::into),
        }
    }
}
