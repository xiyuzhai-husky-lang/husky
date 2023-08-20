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
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TypeItemSynNodeDecl {
    AssociatedFn(TypeAssociatedFnSynNodeDecl),
    MethodFn(TypeMethodFnSynNodeDecl),
    AssociatedType(TypeAssociatedTypeSynNodeDecl),
    AssociatedVal(TypeAssociatedValSynNodeDecl),
    MemoizedField(TypeMemoizedFieldSynNodeDecl),
}

impl From<TypeItemSynNodeDecl> for SynNodeDecl {
    fn from(decl: TypeItemSynNodeDecl) -> Self {
        SynNodeDecl::AssociatedItem(decl.into())
    }
}

impl TypeItemSynNodeDecl {
    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> TypeItemSynNodePath {
        match self {
            TypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.syn_node_path(db),
            TypeItemSynNodeDecl::AssociatedType(_) => todo!(),
            TypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
            TypeItemSynNodeDecl::MemoizedField(syn_node_decl) => syn_node_decl.syn_node_path(db),
        }
    }

    pub fn ast_idx(self, db: &dyn SynDeclDb) -> AstIdx {
        match self {
            TypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeItemSynNodeDecl::AssociatedType(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeItemSynNodeDecl::AssociatedVal(syn_node_decl) => syn_node_decl.ast_idx(db),
            TypeItemSynNodeDecl::MemoizedField(syn_node_decl) => syn_node_decl.ast_idx(db),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a dyn SynDeclDb) -> &'a [TemplateParameterDecl] {
        match self {
            TypeItemSynNodeDecl::AssociatedFn(_) => todo!(),
            TypeItemSynNodeDecl::MethodFn(_) => todo!(),
            TypeItemSynNodeDecl::AssociatedType(_) => todo!(),
            TypeItemSynNodeDecl::AssociatedVal(_) => todo!(),
            TypeItemSynNodeDecl::MemoizedField(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TypeItemSynNodeDecl::AssociatedFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeItemSynNodeDecl::MethodFn(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeItemSynNodeDecl::AssociatedType(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeItemSynNodeDecl::AssociatedVal(syn_node_decl) => syn_node_decl.syn_expr_region(db),
            TypeItemSynNodeDecl::MemoizedField(syn_node_decl) => syn_node_decl.syn_expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
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

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        ty_item_syn_node_decl(db, self)
    }
}

impl HasSynNodeDecl for TypeItemSynNode {
    type NodeDecl = TypeItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        todo!()
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_item_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: TypeItemSynNodePath,
) -> TypeItemSynNodeDecl {
    let module_path = syn_node_path.module_path(db);
    let ctx = DeclParser::new(db, module_path);
    ctx.parse_ty_item_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_ty_item_syn_node_decl(
        &self,
        syn_node_path: TypeItemSynNodePath,
    ) -> TypeItemSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                item_kind:
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TypeItem(item_kind),
                    },
                saved_stream_state,
                ..
            } => self.parse_ty_item_syn_node_decl_aux(
                syn_node_path,
                node,
                ast_idx,
                token_group_idx,
                item_kind,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    fn parse_ty_item_syn_node_decl_aux(
        &self,
        syn_node_path: TypeItemSynNodePath,
        node: TypeItemSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        ty_item_kind: TypeItemKind,
        saved_stream_state: TokenStreamState,
    ) -> TypeItemSynNodeDecl {
        match ty_item_kind {
            TypeItemKind::MethodFn => self
                .parse_ty_method_node_decl(
                    syn_node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into(),
            TypeItemKind::AssociatedFn => self
                .parse_ty_associated_fn_node_decl(
                    syn_node_path,
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
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum TypeItemSynDecl {
    AssociatedFn(TypeAssociatedFnSynDecl),
    MethodFn(TypeMethodFnSynDecl),
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
    pub fn path(self, db: &dyn SynDeclDb) -> TypeItemPath {
        match self {
            TypeItemSynDecl::AssociatedFn(decl) => decl.path(db),
            TypeItemSynDecl::MethodFn(decl) => decl.path(db),
            TypeItemSynDecl::AssociatedType(_) => todo!(),
            TypeItemSynDecl::AssociatedVal(_) => todo!(),
            TypeItemSynDecl::MemoizedField(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, _db: &'a dyn SynDeclDb) -> &'a [TemplateParameterDecl] {
        match self {
            TypeItemSynDecl::AssociatedFn(_) => todo!(),
            TypeItemSynDecl::MethodFn(_) => todo!(),
            TypeItemSynDecl::AssociatedType(_) => todo!(),
            TypeItemSynDecl::AssociatedVal(_) => todo!(),
            TypeItemSynDecl::MemoizedField(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            TypeItemSynDecl::AssociatedFn(decl) => decl.syn_expr_region(db),
            TypeItemSynDecl::MethodFn(decl) => decl.syn_expr_region(db),
            TypeItemSynDecl::AssociatedType(decl) => decl.syn_expr_region(db),
            TypeItemSynDecl::AssociatedVal(decl) => decl.syn_expr_region(db),
            TypeItemSynDecl::MemoizedField(decl) => decl.syn_expr_region(db),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
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

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        ty_item_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_item_syn_decl(
    db: &dyn SynDeclDb,
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

// impl HasDecl for TypeItemNodePath {
//     type Decl = TypeItemDecl;

//     fn decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
//         todo!()
//         // Err(&DeclError::Original(OriginalDeclError::Deprecated))
//         // todo!("deprecated")
//         // self.parent_ty(db)
//         //     .item_syn_decls(db)
//         //     .map_err(|_| todo!())?
//         //     .get_entry(self.ident(db))
//         //     .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
//         //     .1
//         //     .map_err(|_| todo!())
//     }
// }

// impl HasDecl for TypeItemNode {
//     type Decl = TypeItemDecl;

//     fn decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
//         todo!()
//         // Err(&DeclError::Original(OriginalDeclError::Deprecated))
//         // todo!("deprecated")
//         // self.parent_ty(db)
//         //     .item_syn_decls(db)
//         //     .map_err(|_| todo!())?
//         //     .get_entry(self.ident(db))
//         //     .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
//         //     .1
//         //     .map_err(|_| todo!())
//     }
// }

// impl HasItemDecls for TypeItemPath {
//     type ItemDecls = TypeItemDecls;

//     fn item_syn_decls<'a>(self, db: &'a dyn SynDeclDb) -> DeclResult<'a, &'a Self::ItemDecls> {
//         todo!()
//         // self.ty_path(db)
//         //     .item_syn_decls_map(db)
//         //     .map_err(|_| todo!())?
//         //     .get_entry(self.ident(db))
//         //     .ok_or(&DeclError::Original(OriginalDeclError::NoSuchItem))?
//         //     .1
//         //     .as_ref()
//         //     .map_err(|_| todo!())
//     }
// }

// #[salsa::tracked(jar = SynDeclJar, return_ref)]
// pub(crate) fn ty_item_syn_decls_map(
//     db: &dyn SynDeclDb,
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

//     fn item_syn_decls_map<'a>(
//         self,
//         db: &'a dyn SynDeclDb,
//     ) -> EntityTreeBundleResultRef<'a, &'a [(Ident, Result<Self::ItemDecls, ()>)]> {
//         match ty_item_syn_decls_map(db, self) {
//             Ok(ty_item_syn_decls_map) => Ok(ty_item_syn_decls_map),
//             Err(e) => Err(e),
//         }
//     }
// }
