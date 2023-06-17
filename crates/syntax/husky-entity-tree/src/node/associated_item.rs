mod trai;
mod trai_for_ty;
mod ty;

pub use self::trai::*;
pub use self::trai_for_ty::*;
pub use self::ty::*;

use crate::*;
use husky_entity_taxonomy::*;
use husky_word::IdentPairMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum AssociatedItemNodePath {
    TypeItem(TypeItemNodePath),
    TraitItem(TraitItemNodePath),
    TraitForTypeItem(TraitForTypeItemNodePath),
}

impl AssociatedItemNodePath {
    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            AssociatedItemNodePath::TypeItem(id) => id.module_path(db),
            AssociatedItemNodePath::TraitItem(id) => id.module_path(db),
            AssociatedItemNodePath::TraitForTypeItem(id) => id.module_path(db),
        }
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> AssociatedItemNode {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum AssociatedItemNode {
    TypeItem(TypeItemNode),
}

impl AssociatedItemNode {
    pub fn new_impl_associated_item(
        db: &dyn EntityTreeDb,
        impl_block: ImplBlockNode,
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

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        match self {
            AssociatedItemNode::TypeItem(node) => node.module_path(db),
        }
    }
}

impl AsVecMapEntry for AssociatedItemNode {
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

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn trai_for_ty_impl_block_items(
    db: &dyn EntityTreeDb,
    impl_block: TraitForTypeImplBlockNode,
) -> Vec<(Ident, AssociatedItemNode)> {
    let Some(items) = impl_block.items(db) else {
        return Default::default()
    };
    calc_impl_block_items(db, impl_block.into(), impl_block.module_path(db), items)
}

pub(crate) fn calc_impl_block_items(
    db: &dyn EntityTreeDb,
    impl_block: ImplBlockNode,
    module_path: ModulePath,
    body: ImplBlockItems,
) -> Vec<(Ident, AssociatedItemNode)> {
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    body.ast_idx_range()
        .into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Defn {
                    visibility_expr,
                    entity_kind,
                    ident_token,
                    is_generic,
                    ..
                } => {
                    let associated_item_kind = match entity_kind {
                        EntityKind::AssociatedItem {
                            associated_item_kind,
                        } => *associated_item_kind,
                        _ => unreachable!(),
                    };
                    Some((
                        ident_token.ident(),
                        AssociatedItemNode::new_impl_associated_item(
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
