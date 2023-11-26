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

use husky_entity_kind::TypeItemKind;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum TypeItemSynNodeDecl {
    AssociatedFn(TypeAssociatedFnSynNodeDecl),
    MethodFn(TypeMethodFnSynNodeDecl),
    AssociatedType(TypeAssociatedTypeSynNodeDecl),
    AssociatedVal(TypeAssociatedValSynNodeDecl),
    MemoizedField(TypeMemoizedFieldSynNodeDecl),
}

impl From<TypeItemSynNodeDecl> for ItemSynNodeDecl {
    fn from(decl: TypeItemSynNodeDecl) -> Self {
        ItemSynNodeDecl::AssociatedItem(decl.into())
    }
}

impl TypeItemSynNodeDecl {
    pub fn syn_node_path(self, db: &::salsa::Db) -> TypeItemSynNodePath {
        match self {
            TypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeItemSynNodeDecl::AssociatedType(_) => todo!(),
            TypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
            TypeItemSynNodeDecl::MemoizedField(syn_node_decl) => syn_node_decl.syn_node_path(db),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            TypeItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TypeItemSynNodeDecl::MethodFn(_) => todo!(),
            TypeItemSynNodeDecl::AssociatedType(_) => todo!(),
            TypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
            TypeItemSynNodeDecl::MemoizedField(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeItemSynNodeDecl::AssociatedType(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeItemSynNodeDecl::AssociatedVal(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeItemSynNodeDecl::MemoizedField(syn_node_decl) => syn_node_decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        match self {
            TypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.errors(db),
            TypeItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.errors(db),
            TypeItemSynNodeDecl::AssociatedType(syn_node_decl) => syn_node_decl.errors(db),
            TypeItemSynNodeDecl::AssociatedVal(syn_node_decl) => syn_node_decl.errors(db),
            TypeItemSynNodeDecl::MemoizedField(syn_node_decl) => syn_node_decl.errors(db),
        }
    }
}

impl HasSynNodeDecl for TypeItemSynNodePath {
    type NodeDecl = TypeItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        ty_item_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_item_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: TypeItemSynNodePath,
) -> TypeItemSynNodeDecl {
    let ctx = DeclParser::new(db, syn_node_path);
    ctx.parse_ty_item_syn_node_decl()
}

impl<'a> DeclParser<'a, TypeItemSynNodePath> {
    fn parse_ty_item_syn_node_decl(&self) -> TypeItemSynNodeDecl {
        match self.syn_node_path().item_kind(self.db()) {
            TypeItemKind::MethodFn => self.parse_ty_method_node_decl().into(),
            TypeItemKind::AssociatedFunctionFn => self.parse_ty_associated_fn_node_decl().into(),
            TypeItemKind::MemoizedField => self.parse_ty_memo_decl().into(),
            TypeItemKind::AssociatedVal => todo!(),
            TypeItemKind::AssociatedType => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum TypeItemSynDecl {
    AssociatedFn(TypeAssociatedFnSynDecl),
    MethodFn(TypeMethodFnSynDecl),
    // MethodFunction(TypeMethodFunctionSynDecl),
    AssociatedType(TypeAssociatedTypeSynDecl),
    AssociatedVal(TypeAssociatedValSynDecl),
    MemoizedField(TypeMemoizedFieldSynDecl),
}

impl From<TypeItemSynDecl> for SynDecl {
    fn from(decl: TypeItemSynDecl) -> Self {
        SynDecl::AssociatedItem(decl.into())
    }
}

impl TypeItemSynDecl {
    pub fn path(self, db: &::salsa::Db) -> TypeItemPath {
        match self {
            TypeItemSynDecl::AssociatedFn(decl) => decl.path(db),
            TypeItemSynDecl::MethodFn(decl) => decl.path(db),
            // TypeItemSynDecl::MethodFunction(decl) => decl.path(db),
            TypeItemSynDecl::AssociatedType(_) => todo!(),
            TypeItemSynDecl::AssociatedVal(_) => todo!(),
            TypeItemSynDecl::MemoizedField(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a ::salsa::Db) -> &'a [TemplateSynParameterData] {
        match self {
            TypeItemSynDecl::AssociatedFn(_) => todo!(),
            TypeItemSynDecl::MethodFn(_) => todo!(),
            // TypeItemSynDecl::MethodFunction(decl) => todo!(),
            TypeItemSynDecl::AssociatedType(_) => todo!(),
            TypeItemSynDecl::AssociatedVal(_) => todo!(),
            TypeItemSynDecl::MemoizedField(_) => todo!(),
        }
    }

    pub fn parenate_parameters<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> Option<&'a [ParenateSynParameterData]> {
        match self {
            TypeItemSynDecl::AssociatedFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TypeItemSynDecl::MethodFn(syn_decl) => Some(syn_decl.parenate_parameters(db)),
            TypeItemSynDecl::AssociatedType(_) => None,
            TypeItemSynDecl::AssociatedVal(_) => None,
            TypeItemSynDecl::MemoizedField(_) => None,
        }
    }

    pub fn syn_expr_region(self, db: &::salsa::Db) -> SynExprRegion {
        match self {
            TypeItemSynDecl::AssociatedFn(decl) => decl.syn_expr_region(db),
            TypeItemSynDecl::MethodFn(decl) => decl.syn_expr_region(db),
            // TypeItemSynDecl::MethodFunction(decl) => decl.syn_expr_region(db),
            TypeItemSynDecl::AssociatedType(decl) => decl.syn_expr_region(db),
            TypeItemSynDecl::AssociatedVal(decl) => decl.syn_expr_region(db),
            TypeItemSynDecl::MemoizedField(decl) => decl.syn_expr_region(db),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = SynDeclDb, jar = SynDeclJar)]
#[enum_class::from_variants]
pub enum TypeItemDecls {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnSynDecl>),
    MethodFn(SmallVecImpl<TypeMethodFnSynDecl>),
    MethodFunction(/* adhoc */),
    AssociatedType(SmallVecImpl<TypeAssociatedTypeSynDecl>),
    AssociatedVal(SmallVecImpl<TypeAssociatedValSynDecl>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldSynDecl>),
}

impl HasSynDecl for TypeItemPath {
    type Decl = TypeItemSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        ty_item_syn_decl(db, self)
    }
}

// #[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_item_syn_decl(
    db: &::salsa::Db,
    path: TypeItemPath,
) -> DeclResult<TypeItemSynDecl> {
    match path.syn_node_path(db).syn_node_decl(db) {
        TypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => {
            TypeAssociatedFnSynDecl::from_node_decl(db, path, syn_node_decl).map(Into::into)
        }
        TypeItemSynNodeDecl::MethodFn(syn_node_decl) => {
            TypeMethodFnSynDecl::from_node_decl(db, path, syn_node_decl).map(Into::into)
        }
        TypeItemSynNodeDecl::AssociatedType(_) => todo!(),
        TypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
        TypeItemSynNodeDecl::MemoizedField(syn_node_decl) => {
            TypeMemoizedFieldSynDecl::from_node_decl(db, path, syn_node_decl).map(Into::into)
        }
    }
}
