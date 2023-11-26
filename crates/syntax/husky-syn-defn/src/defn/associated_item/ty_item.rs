mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum TypeItemSynNodeDefn {
    AssociatedFn(TypeAssociatedFnSynNodeDefn),
    MethodFn(TypeMethodFnSynNodeDefn),
    AssociatedType(TypeAssociatedTypeSynNodeDefn),
    AssociatedVal(TypeAssociatedValSynNodeDefn),
    MemoizedField(TypeMemoizedFieldSynNodeDefn),
}

impl TypeItemSynNodeDefn {
    pub fn syn_node_path(self, db: &::salsa::Db) -> TypeItemSynNodePath {
        match self {
            TypeItemSynNodeDefn::AssociatedFn(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TypeItemSynNodeDefn::MethodFn(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TypeItemSynNodeDefn::AssociatedType(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TypeItemSynNodeDefn::AssociatedVal(syn_node_defn) => syn_node_defn.syn_node_path(db),
            TypeItemSynNodeDefn::MemoizedField(syn_node_defn) => syn_node_defn.syn_node_path(db),
        }
    }

    pub fn syn_node_decl(self, db: &::salsa::Db) -> TypeItemSynNodeDecl {
        match self {
            TypeItemSynNodeDefn::AssociatedFn(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TypeItemSynNodeDefn::MethodFn(syn_node_defn) => syn_node_defn.syn_node_decl(db).into(),
            TypeItemSynNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TypeItemSynNodeDefn::AssociatedVal(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
            TypeItemSynNodeDefn::MemoizedField(syn_node_defn) => {
                syn_node_defn.syn_node_decl(db).into()
            }
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &::salsa::Db,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            TypeItemSynNodeDefn::AssociatedFn(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            TypeItemSynNodeDefn::MethodFn(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            TypeItemSynNodeDefn::AssociatedType(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            TypeItemSynNodeDefn::AssociatedVal(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
            TypeItemSynNodeDefn::MemoizedField(syn_node_defn) => {
                syn_node_defn.body_with_syn_expr_region(db)
            }
        }
    }
}

impl HasSynNodeDefn for TypeItemSynNodePath {
    type SynNodeDefn = TypeItemSynNodeDefn;

    fn syn_node_defn(self, db: &::salsa::Db) -> Self::SynNodeDefn {
        ty_item_syn_node_defn(db, self)
    }
}

#[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_item_syn_node_defn(
    db: &::salsa::Db,
    syn_node_path: TypeItemSynNodePath,
) -> TypeItemSynNodeDefn {
    match syn_node_path.syn_node_decl(db) {
        TypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => {
            TypeAssociatedFnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeItemSynNodeDecl::MethodFn(syn_node_decl) => {
            TypeMethodFnSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
        TypeItemSynNodeDecl::AssociatedType(_) => todo!(),
        TypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
        TypeItemSynNodeDecl::MemoizedField(syn_node_decl) => {
            TypeMemoizedFieldSynNodeDefn::new(db, syn_node_path, syn_node_decl).into()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDefnDb, jar = SynDefnJar)]
#[enum_class::from_variants]
pub enum TypeItemSynDefn {
    AssociatedFn(TypeAssociatedFnSynDefn),
    MethodFn(TypeMethodFnSynDefn),
    AssociatedType(TypeAssociatedTypeSynDefn),
    AssociatedVal(TypeAssociatedValSynDefn),
    MemoizedField(TypeMemoizedFieldSynDefn),
}

impl TypeItemSynDefn {
    pub fn path(self, db: &::salsa::Db) -> TypeItemPath {
        match self {
            TypeItemSynDefn::AssociatedFn(defn) => defn.path(db),
            TypeItemSynDefn::MethodFn(defn) => defn.path(db),
            TypeItemSynDefn::AssociatedType(defn) => defn.path(db),
            TypeItemSynDefn::AssociatedVal(defn) => defn.path(db),
            TypeItemSynDefn::MemoizedField(defn) => defn.path(db),
        }
    }

    pub fn decl(self, db: &::salsa::Db) -> TypeItemSynDecl {
        match self {
            TypeItemSynDefn::AssociatedFn(defn) => defn.decl(db).into(),
            TypeItemSynDefn::MethodFn(defn) => defn.decl(db).into(),
            TypeItemSynDefn::AssociatedType(defn) => defn.decl(db).into(),
            TypeItemSynDefn::AssociatedVal(defn) => defn.decl(db).into(),
            TypeItemSynDefn::MemoizedField(defn) => defn.decl(db).into(),
        }
    }

    pub fn body_with_syn_expr_region(
        self,
        db: &::salsa::Db,
    ) -> Option<(SynExprIdx, SynExprRegion)> {
        match self {
            TypeItemSynDefn::AssociatedFn(defn) => defn.body_with_syn_expr_region(db).into(),
            TypeItemSynDefn::MethodFn(defn) => defn.body_with_syn_expr_region(db).into(),
            TypeItemSynDefn::AssociatedType(defn) => defn.body_with_syn_expr_region(db).into(),
            TypeItemSynDefn::AssociatedVal(defn) => defn.body_with_syn_expr_region(db).into(),
            TypeItemSynDefn::MemoizedField(defn) => defn.body_with_syn_expr_region(db).into(),
        }
    }
}

impl HasSynDefn for TypeItemPath {
    type SynDefn = TypeItemSynDefn;

    fn syn_defn(self, db: &::salsa::Db) -> SynDefnResult<Self::SynDefn> {
        ty_item_syn_defn(db, self)
    }
}

// #[salsa::tracked(jar = SynDefnJar)]
pub(crate) fn ty_item_syn_defn(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> SynDefnResult<TypeItemSynDefn> {
    let decl = path.syn_decl(db)?;
    Ok(match decl {
        TypeItemSynDecl::AssociatedFn(decl) => TypeAssociatedFnSynDefn::new(db, path, decl)?.into(),
        TypeItemSynDecl::MethodFn(decl) => TypeMethodFnSynDefn::new(db, path, decl)?.into(),
        TypeItemSynDecl::AssociatedType(_) => todo!(),
        TypeItemSynDecl::AssociatedVal(_) => todo!(),
        TypeItemSynDecl::MemoizedField(decl) => {
            TypeMemoizedFieldSynDefn::new(db, path, decl)?.into()
        }
    })
}
