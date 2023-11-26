mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum MajorItemSynNodeDecl {
    Type(TypeSynNodeDecl),
    Fugitive(FugitiveSynNodeDecl),
    Trait(TraitSynNodeDecl),
}

impl MajorItemSynNodeDecl {
    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            MajorItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
            MajorItemSynNodeDecl::Fugitive(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db).into()
            }
            MajorItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
        }
    }

    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> ItemSynNodePath {
        match self {
            MajorItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            MajorItemSynNodeDecl::Fugitive(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            MajorItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        match self {
            MajorItemSynNodeDecl::Type(syn_node_decl) => syn_node_decl.errors(db),
            MajorItemSynNodeDecl::Fugitive(syn_node_decl) => syn_node_decl.errors(db),
            MajorItemSynNodeDecl::Trait(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasSynNodeDecl for MajorItemSynNodePath {
    type NodeDecl = MajorItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        match self {
            MajorItemSynNodePath::Trait(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            MajorItemSynNodePath::Type(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            MajorItemSynNodePath::Fugitive(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum MajorItemSynDecl {
    Type(TypeSynDecl),
    Trait(TraitSynDecl),
    Fugitive(FugitiveSynDecl),
}

impl MajorItemSynDecl {
    pub fn template_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [TemplateSynParameterData] {
        match self {
            MajorItemSynDecl::Type(decl) => decl.template_parameters(db),
            MajorItemSynDecl::Fugitive(decl) => decl.template_parameters(db),
            MajorItemSynDecl::Trait(decl) => decl.template_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            MajorItemSynDecl::Type(decl) => decl.syn_expr_region(db),
            MajorItemSynDecl::Fugitive(decl) => decl.syn_expr_region(db),
            MajorItemSynDecl::Trait(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn path(self, db: &dyn SynDeclDb) -> MajorItemPath {
        match self {
            MajorItemSynDecl::Type(decl) => decl.path(db).into(),
            MajorItemSynDecl::Fugitive(decl) => decl.path(db).into(),
            MajorItemSynDecl::Trait(decl) => decl.path(db).into(),
        }
    }
}

impl HasSynDecl for MajorItemPath {
    type Decl = MajorItemSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        match self {
            MajorItemPath::Type(id) => id.syn_decl(db).map(Into::into),
            MajorItemPath::Trait(id) => id.syn_decl(db).map(Into::into),
            MajorItemPath::Fugitive(id) => id.syn_decl(db).map(Into::into),
        }
    }
}
