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
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum FugitiveNodeDecl {
    Fn(FnNodeDecl),
    Val(ValNodeDecl),
    Gn(GnNodeDecl),
}

impl FugitiveNodeDecl {
    pub fn syn_node_path(self, db: &dyn DeclDb) -> FugitiveSynNodePath {
        match self {
            FugitiveNodeDecl::Fn(decl) => decl.syn_node_path(db),
            FugitiveNodeDecl::Val(decl) => decl.syn_node_path(db),
            FugitiveNodeDecl::Gn(decl) => decl.syn_node_path(db),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            FugitiveNodeDecl::Fn(decl) => decl.ast_idx(db),
            FugitiveNodeDecl::Val(decl) => decl.ast_idx(db),
            FugitiveNodeDecl::Gn(decl) => decl.ast_idx(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> SynExprRegion {
        match self {
            FugitiveNodeDecl::Fn(decl) => decl.expr_region(db),
            FugitiveNodeDecl::Val(decl) => decl.expr_region(db),
            FugitiveNodeDecl::Gn(decl) => decl.expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        match self {
            FugitiveNodeDecl::Fn(node_decl) => node_decl.errors(db),
            FugitiveNodeDecl::Val(node_decl) => node_decl.errors(db),
            FugitiveNodeDecl::Gn(node_decl) => node_decl.errors(db),
        }
    }
}

impl HasNodeDecl for FugitiveSynNodePath {
    type NodeDecl = FugitiveNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        fugitive_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn fugitive_node_decl(
    db: &dyn DeclDb,
    syn_node_path: FugitiveSynNodePath,
) -> FugitiveNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.module_path(db));
    parser.parse_fugitive_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_fugitive_node_decl(&self, syn_node_path: FugitiveSynNodePath) -> FugitiveNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                saved_stream_state,
                ..
            } => self.parse_fugitive_node_decl_aux(
                syn_node_path,
                ast_idx,
                token_group_idx,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_fugitive_node_decl_aux(
        &self,
        syn_node_path: FugitiveSynNodePath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> FugitiveNodeDecl {
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
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum FugitiveDecl {
    Fn(FnDecl),
    Val(ValDecl),
    Gn(GnDecl),
    // todo: AliasType
}

impl FugitiveDecl {
    fn from_node_decl(
        db: &dyn DeclDb,
        path: FugitivePath,
        node_decl: FugitiveNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match node_decl {
            FugitiveNodeDecl::Fn(node_decl) => FnDecl::from_node_decl(db, path, node_decl)?.into(),
            FugitiveNodeDecl::Val(node_decl) => {
                ValDecl::from_node_decl(db, path, node_decl)?.into()
            }
            FugitiveNodeDecl::Gn(node_decl) => GnDecl::from_node_decl(db, path, node_decl)?.into(),
        })
    }

    pub fn generic_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            FugitiveDecl::Fn(decl) => decl.generic_parameters(db),
            FugitiveDecl::Val(_decl) => &[],
            FugitiveDecl::Gn(decl) => decl.generic_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> SynExprRegion {
        match self {
            FugitiveDecl::Fn(decl) => decl.expr_region(db),
            FugitiveDecl::Val(decl) => decl.expr_region(db),
            FugitiveDecl::Gn(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> FugitivePath {
        match self {
            FugitiveDecl::Fn(decl) => decl.path(db),
            FugitiveDecl::Val(decl) => decl.path(db),
            FugitiveDecl::Gn(decl) => decl.path(db),
        }
    }
}

impl HasDecl for FugitivePath {
    type Decl = FugitiveDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        fugitive_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn fugitive_decl(db: &dyn DeclDb, path: FugitivePath) -> DeclResult<FugitiveDecl> {
    let node_decl = path.syn_node_path(db).node_decl(db);
    FugitiveDecl::from_node_decl(db, path, node_decl)
}
