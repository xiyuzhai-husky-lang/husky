mod r#fn;
mod gn;
mod ti;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ti::*;
pub use self::val::*;

use super::*;
use husky_item_taxonomy::{EntityKind, FugitiveKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
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

    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            FugitiveSynNodeDecl::Fn(decl) => decl.ast_idx(db),
            FugitiveSynNodeDecl::Val(decl) => decl.ast_idx(db),
            FugitiveSynNodeDecl::Gn(decl) => decl.ast_idx(db),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            FugitiveSynNodeDecl::Fn(decl) => decl.syn_expr_region(db),
            FugitiveSynNodeDecl::Val(decl) => decl.syn_expr_region(db),
            FugitiveSynNodeDecl::Gn(decl) => decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        match self {
            FugitiveSynNodeDecl::Fn(syn_node_decl) => syn_node_decl.errors(db),
            FugitiveSynNodeDecl::Val(syn_node_decl) => syn_node_decl.errors(db),
            FugitiveSynNodeDecl::Gn(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasNodeDecl for FugitiveSynNodePath {
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
    let parser = DeclParser::new(db, syn_node_path.module_path(db));
    parser.parse_fugitive_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_fugitive_syn_node_decl(
        &self,
        syn_node_path: FugitiveSynNodePath,
    ) -> FugitiveSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                saved_stream_state,
                ..
            } => self.parse_fugitive_syn_node_decl_aux(
                syn_node_path,
                ast_idx,
                token_group_idx,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_fugitive_syn_node_decl_aux(
        &self,
        syn_node_path: FugitiveSynNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> FugitiveSynNodeDecl {
        let db = self.db();
        match syn_node_path.fugitive_kind(db) {
            FugitiveKind::Val => self
                .parse_val_node_decl(syn_node_path, ast_idx, token_group_idx, saved_stream_state)
                .into(),
            FugitiveKind::Fn => self
                .parse_fn_node_decl(syn_node_path, ast_idx, token_group_idx, saved_stream_state)
                .into(),
            FugitiveKind::AliasType => {
                todo!()
            }
            FugitiveKind::Gn => self
                .parse_gn_node_decl(syn_node_path, ast_idx, token_group_idx, saved_stream_state)
                .into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SynDeclDb)]
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

    pub fn generic_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [GenericParameterDecl] {
        match self {
            FugitiveSynDecl::Fn(decl) => decl.generic_parameters(db),
            FugitiveSynDecl::Val(_decl) => &[],
            FugitiveSynDecl::Gn(decl) => decl.generic_parameters(db),
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
