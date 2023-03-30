mod feature;
mod r#fn;
mod gn;
mod type_alias;
mod value;

pub use feature::*;
pub use gn::*;
use husky_entity_taxonomy::{EntityKind, FormKind};
pub use r#fn::*;

pub use type_alias::*;
pub use value::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum FormDecl {
    Fn(FnDecl),
    Feature(FeatureDecl),
    Gn(GnDecl),
    Value(ValueDecl),
}

impl FormDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            FormDecl::Fn(decl) => decl.ast_idx(db),
            FormDecl::Feature(decl) => decl.ast_idx(db),
            FormDecl::Gn(decl) => decl.ast_idx(db),
            FormDecl::Value(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            FormDecl::Fn(decl) => decl.implicit_parameters(db),
            FormDecl::Feature(_decl) => Ok(&[]),
            FormDecl::Gn(decl) => decl.implicit_parameters(db),
            FormDecl::Value(decl) => decl.implicit_parameters(db),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            FormDecl::Fn(decl) => decl.expr_region(db),
            FormDecl::Feature(decl) => decl.expr_region(db),
            FormDecl::Gn(decl) => decl.expr_region(db),
            FormDecl::Value(decl) => decl.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> FormPath {
        match self {
            FormDecl::Fn(decl) => decl.path(db),
            FormDecl::Feature(decl) => decl.path(db),
            FormDecl::Gn(decl) => decl.path(db),
            FormDecl::Value(decl) => decl.path(db),
        }
    }
}

impl HasDecl for FormPath {
    type Decl = FormDecl;

    fn decl<'a>(self, db: &'a dyn DeclDb) -> DeclResultRef<'a, Self::Decl> {
        todo!()
    }
}

#[salsa::tracked(jar = DeclJar, return_ref)]
pub(crate) fn form_decl(db: &dyn DeclDb, path: FormPath) -> DeclResult<FormDecl> {
    let parser = DeclParseContext::new(db, path.module_path(db))?;
    parser.parse_form_decl(path)
}

impl<'a> DeclParseContext<'a> {
    fn parse_form_decl(&self, path: FormPath) -> DeclResult<FormDecl> {
        let ident = path.ident(self.db());
        let ast_idx: AstIdx = self.resolve_module_item_symbol(path).ast_idx(self.db());
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,

                entity_kind,

                saved_stream_state,
                ..
            } => self.parse_form_decl_aux(
                ast_idx,
                path,
                entity_kind,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_form_decl_aux(
        &self,
        ast_idx: AstIdx,
        path: FormPath,
        _entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        _body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> Result<FormDecl, DeclError> {
        match path.form_kind(self.db()) {
            FormKind::Feature => {
                self.parse_feature_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Fn => self.parse_fn_decl(ast_idx, token_group_idx, saved_stream_state, path),
            FormKind::Value => todo!(),
            FormKind::TypeAlias => {
                todo!()
            }
            FormKind::Gn => self.parse_gn_decl(ast_idx, token_group_idx, saved_stream_state, path),
        }
    }
}
