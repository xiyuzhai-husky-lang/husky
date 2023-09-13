mod r#fn;
mod gn;
mod ti;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ti::*;
pub use self::val::*;

use super::*;
use husky_entity_taxonomy::{EntityKind, FugitiveKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum FugitiveSynNodeDecl {
    Fn(FnSynNodeDecl),
    Val(ValSynNodeDecl),
    Gn(GnSynNodeDecl),
}

impl FugitiveSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> FugitiveSynNodePath {
        match self {
            FugitiveSynNodeDecl::Fn(decl) => decl.syn_node_path(db),
            FugitiveSynNodeDecl::Val(decl) => decl.syn_node_path(db),
            FugitiveSynNodeDecl::Gn(decl) => decl.syn_node_path(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            FugitiveSynNodeDecl::Fn(decl) => decl.syn_expr_region(db),
            FugitiveSynNodeDecl::Val(decl) => decl.syn_expr_region(db),
            FugitiveSynNodeDecl::Gn(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        match self {
            FugitiveSynNodeDecl::Fn(syn_node_decl) => syn_node_decl.errors(db),
            FugitiveSynNodeDecl::Val(syn_node_decl) => syn_node_decl.errors(db),
            FugitiveSynNodeDecl::Gn(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasSynNodeDecl for FugitiveSynNodePath {
    type NodeDecl = FugitiveSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        fugitive_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn fugitive_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: FugitiveSynNodePath,
) -> FugitiveSynNodeDecl {
    DeclParser::new(db, syn_node_path).parse_fugitive_syn_node_decl()
}

impl<'a> DeclParser<'a, FugitiveSynNodePath> {
    fn parse_fugitive_syn_node_decl(&self) -> FugitiveSynNodeDecl {
        match self.syn_node_path().fugitive_kind(self.db()) {
            FugitiveKind::Val => self.parse_val_node_decl().into(),
            FugitiveKind::Fn => self.parse_fn_node_decl().into(),
            FugitiveKind::AliasType => {
                todo!()
            }
            FugitiveKind::Gn => self.parse_gn_node_decl().into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum FugitiveSynDecl {
    Fn(FnSynDecl),
    Val(ValSynDecl),
    Gn(GnSynDecl),
    // todo: AliasType
}

impl FugitiveSynDecl {
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: FugitivePath,
        syn_node_decl: FugitiveSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            FugitiveSynNodeDecl::Fn(syn_node_decl) => {
                FnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::Val(syn_node_decl) => {
                ValSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::Gn(syn_node_decl) => {
                GnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }

    pub fn template_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        match self {
            FugitiveSynDecl::Fn(decl) => decl.template_parameters(db),
            FugitiveSynDecl::Val(_decl) => &[],
            FugitiveSynDecl::Gn(decl) => decl.template_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            FugitiveSynDecl::Fn(decl) => decl.syn_expr_region(db),
            FugitiveSynDecl::Val(decl) => decl.syn_expr_region(db),
            FugitiveSynDecl::Gn(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn path(self, db: &dyn SynDeclDb) -> FugitivePath {
        match self {
            FugitiveSynDecl::Fn(decl) => decl.path(db),
            FugitiveSynDecl::Val(decl) => decl.path(db),
            FugitiveSynDecl::Gn(decl) => decl.path(db),
        }
    }
}

impl HasSynDecl for FugitivePath {
    type Decl = FugitiveSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        fugitive_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn fugitive_syn_decl(
    db: &dyn SynDeclDb,
    path: FugitivePath,
) -> DeclResult<FugitiveSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    FugitiveSynDecl::from_node_decl(db, path, syn_node_decl)
}
