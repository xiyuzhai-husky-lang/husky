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
use husky_ast::*;
use husky_coword::{Ident, IdentPairMap};
use husky_entity_taxonomy::TypeItemKind;
use vec_like::VecMapGetEntry;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeItemNodeDecl {
    AssociatedFn(TypeAssociatedFnNodeDecl),
    MethodFn(TypeMethodFnNodeDecl),
    AssociatedType(TypeAssociatedTypeNodeDecl),
    AssociatedVal(TypeAssociatedValNodeDecl),
    MemoizedField(TypeMemoizedFieldNodeDecl),
}

impl From<TypeItemNodeDecl> for SynNodeDecl {
    fn from(decl: TypeItemNodeDecl) -> Self {
        SynNodeDecl::AssociatedItem(decl.into())
    }
}

impl TypeItemNodeDecl {
    pub fn node_path(self, db: &dyn DeclDb) -> TypeItemSynNodePath {
        match self {
            TypeItemNodeDecl::AssociatedFn(node_decl) => node_decl.node_path(db),
            TypeItemNodeDecl::MethodFn(node_decl) => node_decl.node_path(db),
            TypeItemNodeDecl::AssociatedType(_) => todo!(),
            TypeItemNodeDecl::AssociatedVal(_) => todo!(),
            TypeItemNodeDecl::MemoizedField(node_decl) => node_decl.node_path(db),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeItemNodeDecl::AssociatedFn(node_decl) => node_decl.ast_idx(db),
            TypeItemNodeDecl::MethodFn(node_decl) => node_decl.ast_idx(db),
            TypeItemNodeDecl::AssociatedType(node_decl) => node_decl.ast_idx(db),
            TypeItemNodeDecl::AssociatedVal(node_decl) => node_decl.ast_idx(db),
            TypeItemNodeDecl::MemoizedField(node_decl) => node_decl.ast_idx(db),
        }
    }

    pub fn generic_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            TypeItemNodeDecl::AssociatedFn(_) => todo!(),
            TypeItemNodeDecl::MethodFn(_) => todo!(),
            TypeItemNodeDecl::AssociatedType(_) => todo!(),
            TypeItemNodeDecl::AssociatedVal(_) => todo!(),
            TypeItemNodeDecl::MemoizedField(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> SynExprRegion {
        match self {
            TypeItemNodeDecl::AssociatedFn(node_decl) => node_decl.expr_region(db),
            TypeItemNodeDecl::MethodFn(node_decl) => node_decl.expr_region(db),
            TypeItemNodeDecl::AssociatedType(node_decl) => node_decl.expr_region(db),
            TypeItemNodeDecl::AssociatedVal(node_decl) => node_decl.expr_region(db),
            TypeItemNodeDecl::MemoizedField(node_decl) => node_decl.expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        match self {
            TypeItemNodeDecl::AssociatedFn(node_decl) => node_decl.errors(db),
            TypeItemNodeDecl::MethodFn(node_decl) => node_decl.errors(db),
            TypeItemNodeDecl::AssociatedType(node_decl) => node_decl.errors(db),
            TypeItemNodeDecl::AssociatedVal(node_decl) => node_decl.errors(db),
            TypeItemNodeDecl::MemoizedField(node_decl) => node_decl.errors(db),
        }
    }
}

impl HasNodeDecl for TypeItemSynNodePath {
    type NodeDecl = TypeItemNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        ty_item_node_decl(db, self)
    }
}

impl HasNodeDecl for TypeItemNode {
    type NodeDecl = TypeItemNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        todo!()
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_item_node_decl(
    db: &dyn DeclDb,
    node_path: TypeItemSynNodePath,
) -> TypeItemNodeDecl {
    let module_path = node_path.module_path(db);
    let ctx = DeclParser::new(db, module_path);
    ctx.parse_ty_item_node_decl(node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_ty_item_node_decl(&self, node_path: TypeItemSynNodePath) -> TypeItemNodeDecl {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                entity_kind:
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TypeItem(item_kind),
                    },
                saved_stream_state,
                ..
            } => self.parse_ty_item_node_decl_aux(
                node_path,
                node,
                ast_idx,
                token_group_idx,
                item_kind,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_ty_item_node_decl_aux(
        &self,
        node_path: TypeItemSynNodePath,
        node: TypeItemNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        ty_item_kind: TypeItemKind,
        saved_stream_state: TokenStreamState,
    ) -> TypeItemNodeDecl {
        match ty_item_kind {
            TypeItemKind::MethodFn => self
                .parse_ty_method_node_decl(
                    node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into(),
            TypeItemKind::AssociatedFn => self
                .parse_ty_associated_fn_node_decl(
                    node_path,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into(),
            TypeItemKind::MemoizedField => self
                .parse_ty_memo_decl(ast_idx, token_group_idx, node, saved_stream_state)
                .into(),
            TypeItemKind::AssociatedVal => todo!(),
            TypeItemKind::AssociatedType => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeItemDecl {
    AssociatedFn(TypeAssociatedFnDecl),
    MethodFn(TypeMethodFnDecl),
    AssociatedType(TypeAssociatedTypeDecl),
    AssociatedVal(TypeAssociatedValDecl),
    MemoizedField(TypeMemoizedFieldDecl),
}

impl From<TypeItemDecl> for Decl {
    fn from(decl: TypeItemDecl) -> Self {
        Decl::AssociatedItem(decl.into())
    }
}

impl TypeItemDecl {
    pub fn path(self, db: &dyn DeclDb) -> TypeItemPath {
        match self {
            TypeItemDecl::AssociatedFn(decl) => decl.path(db),
            TypeItemDecl::MethodFn(decl) => decl.path(db),
            TypeItemDecl::AssociatedType(_) => todo!(),
            TypeItemDecl::AssociatedVal(_) => todo!(),
            TypeItemDecl::MemoizedField(decl) => decl.path(db),
        }
    }

    pub fn generic_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            TypeItemDecl::AssociatedFn(_) => todo!(),
            TypeItemDecl::MethodFn(_) => todo!(),
            TypeItemDecl::AssociatedType(_) => todo!(),
            TypeItemDecl::AssociatedVal(_) => todo!(),
            TypeItemDecl::MemoizedField(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> SynExprRegion {
        match self {
            TypeItemDecl::AssociatedFn(decl) => decl.expr_region(db),
            TypeItemDecl::MethodFn(decl) => decl.expr_region(db),
            TypeItemDecl::AssociatedType(decl) => decl.expr_region(db),
            TypeItemDecl::AssociatedVal(decl) => decl.expr_region(db),
            TypeItemDecl::MemoizedField(decl) => decl.expr_region(db),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TypeItemDecls {
    AssociatedFn(SmallVecImpl<TypeAssociatedFnDecl>),
    MethodFn(SmallVecImpl<TypeMethodFnDecl>),
    MethodFunction(/* adhoc */),
    AssociatedType(SmallVecImpl<TypeAssociatedTypeDecl>),
    AssociatedVal(SmallVecImpl<TypeAssociatedValDecl>),
    MemoizedField(SmallVecImpl<TypeMemoizedFieldDecl>),
}

impl HasDecl for TypeItemPath {
    type Decl = TypeItemDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        ty_item_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_item_decl(db: &dyn DeclDb, path: TypeItemPath) -> DeclResult<TypeItemDecl> {
    match path.node_path(db).node_decl(db) {
        TypeItemNodeDecl::AssociatedFn(node_decl) => {
            TypeAssociatedFnDecl::from_node_decl(db, path, node_decl).map(Into::into)
        }
        TypeItemNodeDecl::MethodFn(node_decl) => {
            TypeMethodFnDecl::from_node_decl(db, path, node_decl).map(Into::into)
        }
        TypeItemNodeDecl::AssociatedType(_) => todo!(),
        TypeItemNodeDecl::AssociatedVal(_) => todo!(),
        TypeItemNodeDecl::MemoizedField(node_decl) => {
            TypeMemoizedFieldDecl::from_node_decl(db, path, node_decl).map(Into::into)
        }
    }
}

// impl HasDecl for TypeItemNodePath {
//     type Decl = TypeItemDecl;

//     fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
//         todo!()
//         // Err(&DeclError::Original(OriginalDeclError::Deprecated))
//         // todo!("deprecated")
//         // self.parent_ty(db)
//         //     .item_decls(db)
//         //     .map_err(|_| todo!())?
//         //     .get_entry(self.ident(db))
//         //     .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
//         //     .1
//         //     .map_err(|_| todo!())
//     }
// }

// impl HasDecl for TypeItemNode {
//     type Decl = TypeItemDecl;

//     fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
//         todo!()
//         // Err(&DeclError::Original(OriginalDeclError::Deprecated))
//         // todo!("deprecated")
//         // self.parent_ty(db)
//         //     .item_decls(db)
//         //     .map_err(|_| todo!())?
//         //     .get_entry(self.ident(db))
//         //     .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
//         //     .1
//         //     .map_err(|_| todo!())
//     }
// }

// impl HasItemDecls for TypeItemPath {
//     type ItemDecls = TypeItemDecls;

//     fn item_decls<'a>(self, db: &'a dyn DeclDb) -> DeclResult<'a, &'a Self::ItemDecls> {
//         todo!()
//         // self.ty_path(db)
//         //     .item_decls_map(db)
//         //     .map_err(|_| todo!())?
//         //     .get_entry(self.ident(db))
//         //     .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
//         //     .1
//         //     .as_ref()
//         //     .map_err(|_| todo!())
//     }
// }

// #[salsa::tracked(jar = SynDeclJar, return_ref)]
// pub(crate) fn ty_item_decls_map(
//     db: &dyn DeclDb,
//     path: TypePath,
// ) -> EntityTreeBundleResult<IdentPairMap<Result<TypeItemDecls, ()>>> {
//     let mut map = IdentPairMap::default();
//     for (ident, path) in path.item_paths(db)?.iter().copied() {
//         let ty_item_kind = path.item_kind(db);
//         let result = map.get_value_mut_or_insert_with(ident, || {
//             Ok(match ty_item_kind {
//                 TypeItemKind::MethodFn => TypeItemDecls::MethodFn(Default::default()),
//                 TypeItemKind::AssociatedFn => TypeItemDecls::AssociatedFn(Default::default()),
//                 TypeItemKind::AssociatedVal => TypeItemDecls::AssociatedVal(Default::default()),
//                 TypeItemKind::AssociatedType => TypeItemDecls::AssociatedType(Default::default()),
//                 TypeItemKind::MemoizedField => TypeItemDecls::MemoizedField(Default::default()),
//             })
//         });
//         let Ok(decl) = path.decl(db) else {
//             *result = Err(());
//             continue
//         };
//         match result {
//             Ok(decls) => match (decls, decl) {
//                 (TypeItemDecls::AssociatedFn(decls), TypeItemDecl::AssociatedFn(decl)) => {
//                     decls.push(decl)
//                 }
//                 (TypeItemDecls::MethodFn(decls), TypeItemDecl::MethodFn(decl)) => decls.push(decl),
//                 (TypeItemDecls::AssociatedType(decls), TypeItemDecl::AssociatedType(decl)) => {
//                     decls.push(decl)
//                 }
//                 (TypeItemDecls::AssociatedVal(decls), TypeItemDecl::AssociatedVal(decl)) => {
//                     decls.push(decl)
//                 }
//                 (TypeItemDecls::MemoizedField(decls), TypeItemDecl::MemoizedField(decl)) => {
//                     decls.push(decl)
//                 }
//                 _ => *result = Err(()), // error because of inconsistent type item kind
//             },
//             Err(_) => continue,
//         }
//     }
//     Ok(map)
// }

// impl HasItemDeclsMap for TypePath {
//     type ItemDecls = TypeItemDecls;

//     fn item_decls_map<'a>(
//         self,
//         db: &'a dyn DeclDb,
//     ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<Self::ItemDecls, ()>)]> {
//         match ty_item_decls_map(db, self) {
//             Ok(ty_item_decls_map) => Ok(ty_item_decls_map),
//             Err(e) => Err(e),
//         }
//     }
// }
