mod r#fn;
mod gn;
mod ty_alias;
mod val;

pub use self::gn::*;
pub use self::r#fn::*;
pub use self::ty_alias::*;
pub use self::val::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum FugitiveHirDecl {
    FunctionFn(FnFugitiveHirDecl),
    Val(ValFugitiveHirDecl),
    FunctionGn(GnFugitiveHirDecl),
    TypeAlias(TypeAliasFugitiveHirDecl),
}

impl FugitiveHirDecl {
    pub fn template_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [HirTemplateParameter] {
        match self {
            FugitiveHirDecl::FunctionFn(decl) => decl.template_parameters(db),
            FugitiveHirDecl::Val(_decl) => &[],
            FugitiveHirDecl::FunctionGn(decl) => decl.template_parameters(db),
            FugitiveHirDecl::TypeAlias(_) => todo!(),
        }
    }

    // pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
    //     match self {
    //         FugitiveHirDecl::Fn(decl) => decl.hir_expr_region(db).into(),
    //         FugitiveHirDecl::Val(decl) => decl.hir_expr_region(db).into(),
    //         FugitiveHirDecl::Gn(decl) => decl.hir_expr_region(db).into(),
    //     }
    // }

    pub fn path(self, db: &dyn HirDeclDb) -> FugitivePath {
        match self {
            FugitiveHirDecl::FunctionFn(decl) => decl.path(db),
            FugitiveHirDecl::Val(decl) => decl.path(db),
            FugitiveHirDecl::FunctionGn(decl) => decl.path(db),
            FugitiveHirDecl::TypeAlias(decl) => decl.path(db),
        }
    }
}

impl HasHirDecl for FugitivePath {
    type HirDecl = FugitiveHirDecl;

    fn hir_decl_with_source_map(
        self,
        db: &dyn HirDeclDb,
    ) -> Option<(Self::HirDecl, Self::HirExprSourceMap)> {
        fugitive_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn fugitive_hir_decl(
    db: &dyn HirDeclDb,
    path: FugitivePath,
) -> Option<(FugitiveHirDecl, HirExprSourceMap)> {
    match path
        .ethereal_signature_template(db)
        .expect("no errors for hir stage")
    {
        FugitiveEtherealSignatureTemplate::FunctionFn(ethereal_signature_template) => {
            Some(FnFugitiveHirDecl::from_ethereal(path, ethereal_signature_template, db).into())
        }
        FugitiveEtherealSignatureTemplate::FunctionGn(ethereal_signature_template) => {
            Some(GnFugitiveHirDecl::from_ethereal(path, ethereal_signature_template, db).into())
        }
        FugitiveEtherealSignatureTemplate::TypeAlias(ethereal_signature_template) => Some(
            TypeAliasFugitiveHirDecl::from_ethereal(path, ethereal_signature_template, db).into(),
        ),
        FugitiveEtherealSignatureTemplate::Val(ethereal_signature_template) => {
            Some(ValFugitiveHirDecl::from_ethereal(path, ethereal_signature_template, db).into())
        }
    }
}
