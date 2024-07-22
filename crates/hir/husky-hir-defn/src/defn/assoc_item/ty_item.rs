pub mod assoc_ritchie;
pub mod assoc_ty;
pub mod assoc_val;
pub mod memo;
pub mod method_ritchie;

use self::assoc_ritchie::*;
use self::assoc_ty::*;
use self::assoc_val::*;
use self::memo::*;
use self::method_ritchie::*;
use super::*;
use husky_entity_path::path::assoc_item::ty_item::TypeItemPath;
use husky_hir_decl::decl::TypeItemHirDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemHirDefn {
    AssocRitchie(TypeAssocRitchieHirDefn),
    MethodFn(TypeMethodRitchieHirDefn),
    AssocType(TypeAssocTypeHirDefn),
    AssocVal(TypeAssocValHirDefn),
    MemoizedField(TypeMemoizedFieldHirDefn),
}

impl From<TypeItemHirDefn> for HirDefn {
    fn from(hir_defn: TypeItemHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TypeItemHirDefn {
    pub fn path(self, db: &::salsa::Db) -> TypeItemPath {
        match self {
            TypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn.path(db),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.path(db),
            TypeItemHirDefn::AssocType(hir_defn) => hir_defn.path(db),
            TypeItemHirDefn::AssocVal(hir_defn) => hir_defn.path(db),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> TypeItemHirDecl {
        match self {
            TypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::AssocType(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::AssocVal(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            TypeItemHirDefn::AssocRitchie(hir_defn) => {
                hir_defn.hir_eager_expr_region(db).map(Into::into)
            }
            TypeItemHirDefn::MethodFn(hir_defn) => {
                hir_defn.hir_eager_expr_region(db).map(Into::into)
            }
            TypeItemHirDefn::AssocType(_hir_defn) => todo!(),
            //  hir_defn.hir_expr_region(db).into(),
            TypeItemHirDefn::AssocVal(_hir_defn) => todo!(),
            // Some(hir_defn.hir_expr_region(db).into()),
            TypeItemHirDefn::MemoizedField(hir_defn) => {
                hir_defn.hir_eager_expr_region(db).map(Into::into)
            }
        }
    }

    pub fn hir_expr_body_and_region(self, db: &::salsa::Db) -> Option<(HirExprIdx, HirExprRegion)> {
        match self {
            TypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn
                .eager_body_with_hir_eager_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn
                .eager_body_with_hir_eager_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
            TypeItemHirDefn::AssocType(_hir_defn) => todo!(),
            TypeItemHirDefn::AssocVal(_hir_defn) => todo!(),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn
                .eager_body_with_hir_eager_expr_region(db)
                .map(|(body, region)| (body.into(), region.into())),
        }
    }

    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        match self {
            TypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn.deps(db),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.deps(db),
            TypeItemHirDefn::AssocType(hir_defn) => hir_defn.deps(db),
            TypeItemHirDefn::AssocVal(hir_defn) => hir_defn.deps(db),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.deps(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            TypeItemHirDefn::AssocRitchie(hir_defn) => hir_defn.version_stamp(db),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.version_stamp(db),
            TypeItemHirDefn::AssocType(hir_defn) => hir_defn.version_stamp(db),
            TypeItemHirDefn::AssocVal(hir_defn) => hir_defn.version_stamp(db),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for TypeItemPath {
    type HirDefn = TypeItemHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        ty_item_hir_defn(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn ty_item_hir_defn(db: &::salsa::Db, path: TypeItemPath) -> Option<TypeItemHirDefn> {
    match path.hir_decl(db)? {
        TypeItemHirDecl::AssocRitchie(hir_decl) => {
            Some(TypeAssocRitchieHirDefn::new(db, path, hir_decl).into())
        }
        TypeItemHirDecl::MethodFn(hir_decl) => {
            Some(TypeMethodRitchieHirDefn::new(db, path, hir_decl).into())
        }
        TypeItemHirDecl::AssocType(_) => todo!(),
        TypeItemHirDecl::AssocVal(_) => todo!(),
        TypeItemHirDecl::MemoizedField(hir_decl) => {
            Some(TypeMemoizedFieldHirDefn::new(db, path, hir_decl).into())
        }
    }
}
