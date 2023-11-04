mod r#fn;
mod gn;
mod ti;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ti::*;
pub use self::val::*;

use super::*;
use husky_entity_kind::{EntityKind, FugitiveKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum FugitiveSynNodeDecl {
    FunctionFn(FnSynNodeDecl),
    Val(ValSynNodeDecl),
    FunctionGn(GnSynNodeDecl),
}

impl FugitiveSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> FugitiveSynNodePath {
        match self {
            FugitiveSynNodeDecl::FunctionFn(decl) => decl.syn_node_path(db),
            FugitiveSynNodeDecl::Val(decl) => decl.syn_node_path(db),
            FugitiveSynNodeDecl::FunctionGn(decl) => decl.syn_node_path(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            FugitiveSynNodeDecl::FunctionFn(decl) => decl.syn_expr_region(db),
            FugitiveSynNodeDecl::Val(decl) => decl.syn_expr_region(db),
            FugitiveSynNodeDecl::FunctionGn(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        match self {
            FugitiveSynNodeDecl::FunctionFn(syn_node_decl) => syn_node_decl.errors(db),
            FugitiveSynNodeDecl::Val(syn_node_decl) => syn_node_decl.errors(db),
            FugitiveSynNodeDecl::FunctionGn(syn_node_decl) => syn_node_decl.errors(db),
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
            FugitiveKind::FunctionFn => self.parse_fn_node_decl().into(),
            FugitiveKind::AliasType => {
                todo!()
            }
            FugitiveKind::FunctionGn => self.parse_gn_node_decl().into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum FugitiveSynDecl {
    FunctionFn(FunctionFnSynDecl),
    Val(ValSynDecl),
    FunctionGn(FunctionGnSynDecl),
    // todo: AliasType
}

impl FugitiveSynDecl {
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: FugitivePath,
        syn_node_decl: FugitiveSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            FugitiveSynNodeDecl::FunctionFn(syn_node_decl) => {
                FunctionFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::Val(syn_node_decl) => {
                ValSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::FunctionGn(syn_node_decl) => {
                FunctionGnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }

    pub fn template_parameters<'a>(
        self,
        db: &'a dyn SynDeclDb,
    ) -> &'a [SynTemplateParameterSyndicate] {
        match self {
            FugitiveSynDecl::FunctionFn(decl) => decl.template_parameters(db),
            FugitiveSynDecl::Val(_decl) => &[],
            FugitiveSynDecl::FunctionGn(decl) => decl.template_parameters(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            FugitiveSynDecl::FunctionFn(decl) => decl.syn_expr_region(db),
            FugitiveSynDecl::Val(decl) => decl.syn_expr_region(db),
            FugitiveSynDecl::FunctionGn(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn path(self, db: &dyn SynDeclDb) -> FugitivePath {
        match self {
            FugitiveSynDecl::FunctionFn(decl) => decl.path(db),
            FugitiveSynDecl::Val(decl) => decl.path(db),
            FugitiveSynDecl::FunctionGn(decl) => decl.path(db),
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
