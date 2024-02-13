mod r#fn;
mod gn;
mod ty_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ty_alias::*;
pub use self::val::*;

use super::*;
use husky_entity_kind::MajorFugitiveKind;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum FugitiveSynNodeDecl {
    Fn(MajorFnSynNodeDecl),
    Val(MajorValSynNodeDecl),
    Gn(MajorGnSynNodeDecl),
    TypeAlias(TypeAliasSynNodeDecl),
}

impl FugitiveSynNodeDecl {
    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            FugitiveSynNodeDecl::Fn(slf) => slf.syn_expr_region(db),
            FugitiveSynNodeDecl::Val(slf) => slf.syn_expr_region(db),
            FugitiveSynNodeDecl::Gn(slf) => slf.syn_expr_region(db),
            FugitiveSynNodeDecl::TypeAlias(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            FugitiveSynNodeDecl::Fn(slf) => slf.errors(db),
            FugitiveSynNodeDecl::Val(slf) => slf.errors(db),
            FugitiveSynNodeDecl::Gn(slf) => slf.errors(db),
            FugitiveSynNodeDecl::TypeAlias(slf) => slf.errors(db),
        }
    }
}

impl HasSynNodeDecl for FugitiveSynNodePath {
    type NodeDecl = FugitiveSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        fugitive_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn fugitive_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: FugitiveSynNodePath,
) -> FugitiveSynNodeDecl {
    DeclParser::new(db, syn_node_path.into()).parse_fugitive_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_fugitive_syn_node_decl(
        &self,
        syn_node_path: FugitiveSynNodePath,
    ) -> FugitiveSynNodeDecl {
        match syn_node_path.fugitive_kind(self.db()) {
            MajorFugitiveKind::Val => self.parse_val_node_decl(syn_node_path).into(),
            MajorFugitiveKind::FN => self.parse_fn_node_decl(syn_node_path).into(),
            MajorFugitiveKind::TypeAlias => {
                todo!()
            }
            MajorFugitiveKind::GN => self.parse_gn_node_decl(syn_node_path).into(),
            MajorFugitiveKind::Formal => todo!(),
            MajorFugitiveKind::Const => todo!(),
            MajorFugitiveKind::Ritchie(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum FugitiveSynDecl {
    Fn(MajorFnSynDecl),
    Val(MajorValSynDecl),
    FunctionGn(MajorGnSynDecl),
    TypeAlias(TypeAliasSynDecl),
}

impl FugitiveSynDecl {
    fn from_node_decl(
        db: &::salsa::Db,
        path: FugitivePath,
        syn_node_decl: FugitiveSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            FugitiveSynNodeDecl::Fn(syn_node_decl) => {
                MajorFnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::Val(syn_node_decl) => {
                MajorValSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::Gn(syn_node_decl) => {
                MajorGnSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
            FugitiveSynNodeDecl::TypeAlias(syn_node_decl) => {
                TypeAliasSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            FugitiveSynDecl::Fn(decl) => decl.template_parameters(db),
            FugitiveSynDecl::Val(_decl) => &[],
            FugitiveSynDecl::FunctionGn(decl) => decl.template_parameters(db),
            FugitiveSynDecl::TypeAlias(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            FugitiveSynDecl::Fn(slf) => slf.syn_expr_region(db),
            FugitiveSynDecl::Val(slf) => slf.syn_expr_region(db),
            FugitiveSynDecl::FunctionGn(slf) => slf.syn_expr_region(db),
            FugitiveSynDecl::TypeAlias(slf) => slf.syn_expr_region(db),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> FugitivePath {
        match self {
            FugitiveSynDecl::Fn(slf) => slf.path(db),
            FugitiveSynDecl::Val(slf) => slf.path(db),
            FugitiveSynDecl::FunctionGn(slf) => slf.path(db),
            FugitiveSynDecl::TypeAlias(_) => todo!(),
        }
    }
}

impl HasSynDecl for FugitivePath {
    type Decl = FugitiveSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        fugitive_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn fugitive_syn_decl(
    db: &::salsa::Db,
    path: FugitivePath,
) -> DeclResult<FugitiveSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    FugitiveSynDecl::from_node_decl(db, path, syn_node_decl)
}
