mod trai;
mod ty;
mod ty_as_trai;

pub use self::trai::*;
pub use self::ty::*;
pub use self::ty_as_trai::*;

use husky_entity_taxonomy::AssociatedItemKind;
use husky_print_utils::p;
use husky_word::IdentPairMap;

use crate::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct AssociatedItem {
    #[id]
    pub id: AssociatedItemId,
    pub path: Option<AssociatedItemPath>,
    pub impl_block: ImplBlock,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub associated_item_kind: AssociatedItemKind,
    pub accessibility: Visibility,
    pub is_generic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct AssociatedItemId {
    impl_block_id: ImplBlockId,
    ident: Ident,
}

impl AssociatedItemId {
    pub fn module_path(self) -> ModulePath {
        self.impl_block_id.module()
    }

    pub fn impl_block_id(&self) -> ImplBlockId {
        self.impl_block_id
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }
}

impl AssociatedItem {
    pub fn new_impl_associated_item(
        db: &dyn EntityTreeDb,
        impl_block: ImplBlock,
        ast_idx: AstIdx,
        ident: Ident,
        associated_item_kind: AssociatedItemKind,
        accessibility: Visibility,
        is_generic: bool,
    ) -> Self {
        let id = AssociatedItemId {
            impl_block_id: impl_block.id(db),
            ident,
        };
        let path: Option<AssociatedItemPath> = match associated_item_kind {
            AssociatedItemKind::TraitItem(_) => todo!(),
            AssociatedItemKind::TypeItem(ty_item_kind) => match impl_block {
                ImplBlock::Type(impl_block) => {
                    Some(TypeItemPath::new(db, impl_block.ty(db), ident, ty_item_kind).into())
                }
                ImplBlock::TypeAsTrait(_) => unreachable!(),
                ImplBlock::IllFormed(_) => None,
            },
            AssociatedItemKind::TypeAsTraitItem(ty_as_trai_item_kind) => match impl_block {
                ImplBlock::TypeAsTrait(impl_block) => Some(
                    TypeAsTraitItemPath::new(
                        db,
                        impl_block.ty(db),
                        impl_block.trai(db),
                        ident,
                        ty_as_trai_item_kind,
                    )
                    .into(),
                ),
                ImplBlock::Type(_) => unreachable!(),
                ImplBlock::IllFormed(_) => None,
            },
        };
        Self::new(
            db,
            id,
            path,
            impl_block,
            ast_idx,
            ident,
            associated_item_kind,
            accessibility,
            is_generic,
        )
    }

    pub fn module_path(&self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).impl_block_id.module()
    }
}

impl AsVecMapEntry for AssociatedItem {
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

pub fn impl_block_associated_items(
    db: &dyn EntityTreeDb,
    impl_block: ImplBlock,
) -> &[(Ident, AssociatedItem)] {
    match impl_block {
        ImplBlock::Type(impl_block) => ty_impl_block_items(db, impl_block),
        ImplBlock::TypeAsTrait(impl_block) => ty_as_trai_impl_block_items(db, impl_block),
        ImplBlock::IllFormed(_) => &[],
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_impl_block_items(
    db: &dyn EntityTreeDb,
    impl_block: TypeImplBlock,
) -> IdentPairMap<AssociatedItem> {
    impl_block_associated_items_aux(
        db,
        impl_block.into(),
        impl_block.module_path(db),
        impl_block.body(db),
    )
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_as_trai_impl_block_items(
    db: &dyn EntityTreeDb,
    impl_block: TypeAsTraitImplBlock,
) -> IdentPairMap<AssociatedItem> {
    impl_block_associated_items_aux(
        db,
        impl_block.into(),
        impl_block.module(db),
        impl_block.body(db),
    )
}

pub(crate) fn impl_block_associated_items_aux(
    db: &dyn EntityTreeDb,
    impl_block: ImplBlock,
    module_path: ModulePath,
    body: AstIdxRange,
) -> IdentPairMap<AssociatedItem> {
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    body.into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Defn {
                    accessibility,
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
                        AssociatedItem::new_impl_associated_item(
                            db,
                            impl_block,
                            ast_idx,
                            ident_token.ident(),
                            associated_item_kind,
                            *accessibility,
                            *is_generic,
                        ),
                    ))
                }
                Ast::Err { .. } => None,
                _ => {
                    p!(impl_block.debug(db));
                    p!(ast.debug(db));
                    unreachable!()
                }
            }
        })
        .collect()
}
