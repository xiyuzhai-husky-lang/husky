pub mod r#const;
pub mod function_ritchie;
pub mod ty_alias;
pub mod val;

pub use self::function_ritchie::*;
pub use self::ty_alias::*;
pub use self::val::*;

use self::r#const::{MajorConstSynDecl, MajorConstSynNodeDecl};
use super::*;
use husky_entity_kind::MajorFormKind;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum FormSynNodeDecl {
    FunctionRitchie(MajorFunctionRitchieSynNodeDecl),
    Val(MajorValSynNodeDecl),
    Const(MajorConstSynNodeDecl),
    TypeAlias(TypeAliasSynNodeDecl),
}

impl FormSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            FormSynNodeDecl::FunctionRitchie(slf) => slf.syn_expr_region(db),
            FormSynNodeDecl::Val(slf) => slf.syn_expr_region(db),
            FormSynNodeDecl::Const(slf) => slf.syn_expr_region(db),
            FormSynNodeDecl::TypeAlias(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            FormSynNodeDecl::FunctionRitchie(slf) => slf.errors(db),
            FormSynNodeDecl::Val(slf) => slf.errors(db),
            FormSynNodeDecl::TypeAlias(slf) => slf.errors(db),
            FormSynNodeDecl::Const(slf) => slf.errors(db),
        }
    }
}

impl HasSynNodeDecl for FormSynNodePath {
    type NodeDecl = FormSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        form_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn form_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: FormSynNodePath,
) -> FormSynNodeDecl {
    DeclParser::new(db, syn_node_path.into()).parse_form_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_form_syn_node_decl(&self, syn_node_path: FormSynNodePath) -> FormSynNodeDecl {
        match syn_node_path.form_kind(self.db()) {
            MajorFormKind::Val => self.parse_val_syn_node_decl(syn_node_path).into(),
            MajorFormKind::Ritchie(ritchie_item_kind) => self
                .parse_ritchie_syn_node_decl(syn_node_path, ritchie_item_kind)
                .into(),
            MajorFormKind::TypeAlias => todo!(),
            MajorFormKind::Formal => todo!(),
            MajorFormKind::Const => self.parse_const_syn_node_decl(syn_node_path).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum FormSynDecl {
    Ritchie(MajorFunctionRitchieSynDecl),
    Val(MajorValSynDecl),
    Const(MajorConstSynDecl),
    TypeAlias(TypeAliasSynDecl),
}

impl FormSynDecl {
    fn from_node_decl(
        db: &::salsa::Db,
        path: MajorFormPath,
        syn_node_decl: FormSynNodeDecl,
    ) -> SynDeclResult<Self> {
        Ok(match syn_node_decl {
            FormSynNodeDecl::FunctionRitchie(syn_node_decl) => {
                MajorFunctionRitchieSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FormSynNodeDecl::Val(syn_node_decl) => {
                MajorValSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FormSynNodeDecl::TypeAlias(syn_node_decl) => {
                TypeAliasSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FormSynNodeDecl::Const(syn_node_decl) => {
                MajorConstSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            FormSynDecl::Ritchie(decl) => decl.template_parameters(db),
            FormSynDecl::Val(_decl) => &[],
            FormSynDecl::TypeAlias(_) => todo!(),
            FormSynDecl::Const(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            FormSynDecl::Ritchie(slf) => slf.syn_expr_region(db),
            FormSynDecl::Val(slf) => slf.syn_expr_region(db),
            FormSynDecl::TypeAlias(slf) => slf.syn_expr_region(db),
            FormSynDecl::Const(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> MajorFormPath {
        match self {
            FormSynDecl::Ritchie(slf) => slf.path(db),
            FormSynDecl::Val(slf) => slf.path(db),
            FormSynDecl::TypeAlias(slf) => slf.path(db),
            FormSynDecl::Const(slf) => slf.path(db),
        }
    }
}

impl HasSynDecl for MajorFormPath {
    type Decl = FormSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> SynDeclResult<Self::Decl> {
        form_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn form_syn_decl(db: &::salsa::Db, path: MajorFormPath) -> SynDeclResult<FormSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    FormSynDecl::from_node_decl(db, path, syn_node_decl)
}
