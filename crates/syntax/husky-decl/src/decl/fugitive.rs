mod r#fn;
mod gn;
mod ti;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ti::*;
pub use self::val::*;

use crate::*;
use husky_entity_taxonomy::{EntityKind, FugitiveKind};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum FugitiveRawDecl {
    Fn(FnRawDecl),
    Val(ValRawDecl),
    Gn(GnRawDecl),
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

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        fugitive_decl(db, self).as_ref().copied()
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn fugitive_decl(db: &dyn DeclDb, path: FugitivePath) -> DeclResult<FugitiveDecl> {
    let parser = DeclParseContext::new(db, path.module_path(db))?;
    parser.parse_fugitive_decl(path)
}

impl<'a> DeclParseContext<'a> {
    fn parse_fugitive_decl(&self, path: FugitivePath) -> DeclResult<FugitiveDecl> {
        let ast_idx: AstIdx = self.resolve_module_item_ast_idx(path);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                entity_kind,
                saved_stream_state,
                ..
            } => self.parse_fugitive_decl_aux(
                ast_idx,
                path,
                entity_kind,
                token_group_idx,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_fugitive_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: FugitivePath,
        _entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenStreamState,
    ) -> Result<FugitiveDecl, DeclError> {
        match path.form_kind(self.db()) {
            FugitiveKind::Val => {
                self.parse_feature_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FugitiveKind::Fn => {
                self.parse_fn_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FugitiveKind::Type => {
                todo!()
            }
            FugitiveKind::Gn => {
                self.parse_gn_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
        }
    }
}
