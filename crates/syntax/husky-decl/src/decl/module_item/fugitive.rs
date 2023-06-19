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
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            FugitiveNodeDecl::Fn(decl) => decl.ast_idx(db),
            FugitiveNodeDecl::Val(decl) => decl.ast_idx(db),
            FugitiveNodeDecl::Gn(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            FugitiveNodeDecl::Fn(decl) => decl.implicit_parameters(db),
            FugitiveNodeDecl::Val(_decl) => &[],
            FugitiveNodeDecl::Gn(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            FugitiveNodeDecl::Fn(decl) => decl.expr_region(db),
            FugitiveNodeDecl::Val(decl) => decl.expr_region(db),
            FugitiveNodeDecl::Gn(decl) => decl.expr_region(db),
        }
    }

    pub fn node_path(self, db: &dyn DeclDb) -> FugitiveNodePath {
        match self {
            FugitiveNodeDecl::Fn(decl) => decl.node_path(db),
            FugitiveNodeDecl::Val(decl) => decl.node_path(db),
            FugitiveNodeDecl::Gn(decl) => decl.node_path(db),
        }
    }
}

impl HasNodeDecl for FugitiveNodePath {
    type NodeDecl = FugitiveNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        fugitive_node_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub(crate) fn fugitive_node_decl(db: &dyn DeclDb, node_path: FugitiveNodePath) -> FugitiveNodeDecl {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum FugitiveDecl {
    Fn(FnDecl),
    Val(ValDecl),
    Gn(GnDecl),
}

impl FugitiveDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            FugitiveDecl::Fn(decl) => decl.ast_idx(db),
            FugitiveDecl::Val(decl) => decl.ast_idx(db),
            FugitiveDecl::Gn(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            FugitiveDecl::Fn(decl) => decl.implicit_parameters(db),
            FugitiveDecl::Val(_decl) => &[],
            FugitiveDecl::Gn(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            FugitiveDecl::Fn(decl) => decl.expr_region(db),
            FugitiveDecl::Val(decl) => decl.expr_region(db),
            FugitiveDecl::Gn(decl) => decl.expr_region(db),
        }
    }

    pub fn node_path(self, db: &dyn DeclDb) -> FugitiveNodePath {
        match self {
            FugitiveDecl::Fn(decl) => decl.node_path(db),
            FugitiveDecl::Val(decl) => decl.node_path(db),
            FugitiveDecl::Gn(decl) => decl.node_path(db),
        }
    }
}

impl HasDecl for FugitivePath {
    type Decl = FugitiveDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        self.node_path(db).decl(db)
    }
}

impl HasDecl for FugitiveNodePath {
    type Decl = FugitiveDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        fugitive_decl(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn fugitive_decl(db: &dyn DeclDb, id: FugitiveNodePath) -> DeclResult<FugitiveDecl> {
    let parser = DeclParseContext::new(db, id.module_path(db))?;
    parser.parse_fugitive_decl(id)
}

impl<'a> DeclParseContext<'a> {
    fn parse_fugitive_decl(&self, path: FugitiveNodePath) -> DeclResult<FugitiveDecl> {
        let db = self.db();
        let node = path.node(db);
        let ast_idx: AstIdx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                saved_stream_state,
                ..
            } => self.parse_fugitive_decl_aux(ast_idx, path, token_group_idx, saved_stream_state),
            _ => unreachable!(),
        }
    }

    fn parse_fugitive_decl_aux(
        &self,
        ast_idx: AstIdx,
        id: FugitiveNodePath,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> Result<FugitiveDecl, DeclError> {
        let db = self.db();
        todo!()
        // match id.path(db).form_kind(db) {
        //     FugitiveKind::Val => {
        //         self.parse_feature_decl(ast_idx, token_group_idx, saved_stream_state, id)
        //     }
        //     FugitiveKind::Fn => {
        //         self.parse_fn_decl(ast_idx, token_group_idx, saved_stream_state, id)
        //     }
        //     FugitiveKind::Type => {
        //         todo!()
        //     }
        //     FugitiveKind::Gn => {
        //         self.parse_gn_decl(ast_idx, token_group_idx, saved_stream_state, id)
        //     }
        // }
    }
}
