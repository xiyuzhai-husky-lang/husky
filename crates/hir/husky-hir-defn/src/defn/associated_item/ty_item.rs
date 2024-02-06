mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

use husky_hir_decl::decl::TypeItemHirDecl;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TypeItemHirDefn {
    AssocFn(TypeAssocFnHirDefn),
    MethodFn(TypeMethodFnHirDefn),
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
            TypeItemHirDefn::AssocFn(hir_defn) => hir_defn.path(db),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.path(db),
            TypeItemHirDefn::AssocType(hir_defn) => hir_defn.path(db),
            TypeItemHirDefn::AssocVal(hir_defn) => hir_defn.path(db),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> TypeItemHirDecl {
        match self {
            TypeItemHirDefn::AssocFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::AssocType(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::AssocVal(hir_defn) => hir_defn.hir_decl(db).into(),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            TypeItemHirDefn::AssocFn(hir_defn) => {
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

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            TypeItemHirDefn::AssocFn(hir_defn) => hir_defn.dependencies(db),
            TypeItemHirDefn::MethodFn(hir_defn) => hir_defn.dependencies(db),
            TypeItemHirDefn::AssocType(hir_defn) => hir_defn.dependencies(db),
            TypeItemHirDefn::AssocVal(hir_defn) => hir_defn.dependencies(db),
            TypeItemHirDefn::MemoizedField(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            TypeItemHirDefn::AssocFn(hir_defn) => hir_defn.version_stamp(db),
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

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn ty_item_hir_defn(db: &::salsa::Db, path: TypeItemPath) -> Option<TypeItemHirDefn> {
    match path.hir_decl(db)? {
        TypeItemHirDecl::AssocFn(hir_decl) => {
            Some(TypeAssocFnHirDefn::new(db, path, hir_decl).into())
        }
        TypeItemHirDecl::MethodFn(hir_decl) => {
            Some(TypeMethodFnHirDefn::new(db, path, hir_decl).into())
        }
        TypeItemHirDecl::AssocType(_) => todo!(),
        TypeItemHirDecl::AssocVal(_) => todo!(),
        TypeItemHirDecl::MemoizedField(hir_decl) => {
            Some(TypeMemoizedFieldHirDefn::new(db, path, hir_decl).into())
        }
    }
}
