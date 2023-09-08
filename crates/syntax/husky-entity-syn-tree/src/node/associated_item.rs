mod ill_formed_item;
mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::ill_formed_item::*;
pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use crate::*;
use husky_coword::IdentPairMap;
use husky_entity_taxonomy::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
#[enum_class::from_variants]
pub enum AssociatedItemSynNodePath {
    TypeItem(TypeItemSynNodePath),
    TraitItem(TraitItemSynNodePath),
    TraitForTypeItem(TraitForTypeItemSynNodePath),
    IllFormedItem(IllFormedItemSynNodePath),
}

impl AssociatedItemSynNodePath {
    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<AssociatedItemPath> {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.path(db).map(Into::into)
            }
            AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => None,
        }
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> AssociatedItemSynNode {
        todo!()
    }
}

impl<Db> HasModulePath<Db> for AssociatedItemSynNodePath
where
    Db: ?Sized + EntitySynTreeDb,
{
    fn module_path(self, db: &Db) -> ModulePath {
        match self {
            AssociatedItemSynNodePath::TypeItem(syn_node_path) => syn_node_path.module_path(db),
            AssociatedItemSynNodePath::TraitItem(syn_node_path) => syn_node_path.module_path(db),
            AssociatedItemSynNodePath::TraitForTypeItem(syn_node_path) => {
                syn_node_path.module_path(db)
            }
            AssociatedItemSynNodePath::IllFormedItem(syn_node_path) => {
                syn_node_path.module_path(db)
            }
        }
    }
}

impl HasSynNodePath for AssociatedItemPath {
    type SynNodePath = AssociatedItemSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum AssociatedItemSynNode {
    TypeItem(TypeItemSynNode),
}

impl AssociatedItemSynNode {
    pub fn new_impl_associated_item(
        db: &dyn EntitySynTreeDb,
        impl_block: ImplBlockSynNode,
        ast_idx: AstIdx,
        ident: Ident,
        associated_item_kind: AssociatedItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> Self {
        todo!()
        // let id = AssociatedItemPath {
        //     impl_block_path: impl_block.path(db),
        //     ident,
        // };
        // let path: Option<AssociatedItemPath> = match associated_item_kind {
        //     AssociatedItemKind::TraitItem(_) => todo!(),
        //     AssociatedItemKind::TypeItem(ty_item_kind) => match impl_block {
        //         ImplBlock::Type(impl_block) => {
        //             Some(TypeItemPath::new(db, impl_block.ty_path(db), ident, ty_item_kind).into())
        //         }
        //         ImplBlock::TraitForType(_) => unreachable!(),
        //         ImplBlock::IllFormed(_) => None,
        //     },
        //     AssociatedItemKind::TraitForTypeItem(trai_for_ty_item_kind) => match impl_block {
        //         ImplBlock::TraitForType(impl_block) => Some(
        //             TraitForTypeItemPath::new(
        //                 db,
        //                 impl_block.ty_path(db),
        //                 impl_block.trai(db),
        //                 ident,
        //                 trai_for_ty_item_kind,
        //             )
        //             .into(),
        //         ),
        //         ImplBlock::Type(_) => unreachable!(),
        //         ImplBlock::IllFormed(_) => None,
        //     },
        // };
        // Self::new(
        //     db,
        //     id,
        //     path,
        //     impl_block,
        //     ast_idx,
        //     ident,
        //     associated_item_kind,
        //     visibility,
        //     is_generic,
        // )
    }

    pub fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> AssociatedItemSynNodePath {
        match self {
            AssociatedItemSynNode::TypeItem(node) => node.syn_node_path(db).into(),
        }
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        match self {
            AssociatedItemSynNode::TypeItem(node) => node.module_path(db),
        }
    }
}

impl AsVecMapEntry for AssociatedItemSynNode {
    type K = AssociatedItemPath;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        todo!()
    }

    fn key_ref(&self) -> &Self::K {
        todo!()
    }
}

pub(crate) fn calc_impl_block_items(
    db: &dyn EntitySynTreeDb,
    impl_block: ImplBlockSynNode,
    module_path: ModulePath,
    body: ImplBlockItems,
) -> Vec<(Ident, AssociatedItemSynNode)> {
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    body.ast_idx_range()
        .into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Identifiable {
                    visibility_expr,
                    item_kind,
                    ident_token,
                    is_generic,
                    ..
                } => {
                    let associated_item_kind = match item_kind {
                        EntityKind::AssociatedItem {
                            associated_item_kind,
                        } => *associated_item_kind,
                        _ => unreachable!(),
                    };
                    Some((
                        ident_token.ident(),
                        AssociatedItemSynNode::new_impl_associated_item(
                            db,
                            impl_block,
                            ast_idx,
                            ident_token.ident(),
                            associated_item_kind,
                            visibility_expr.visibility(),
                            *is_generic,
                        ),
                    ))
                }
                Ast::Err { .. } => None,
                _ => {
                    let ast_token_idx_range_sheet =
                        db.ast_token_idx_range_sheet(module_path).unwrap();
                    let token_sheet_data = db.token_sheet_data(module_path).unwrap();
                    let ast_range = ast_token_idx_range_sheet[ast_idx];
                    // p!(ast_range);
                    // assert!(token_sheet_data.len() >= ast_range.end().token_idx().raw());
                    // p!(token_sheet_data[ast_range.start().token_idx()].debug(db));
                    // p!(token_sheet_data[ast_range.start().token_idx() + 1].debug(db));
                    // p!(module_path.debug(db), impl_block.debug(db));
                    // p!(ast.debug(db));
                    // p!(token_sheet_data.debug(db));
                    todo!()
                }
            }
        })
        .collect()
}
