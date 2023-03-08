use husky_entity_taxonomy::AssociatedItemKind;
use husky_print_utils::p;
use husky_word::IdentPairMap;

use crate::*;

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct AssociatedItem {
    #[id]
    pub id: AssociatedItemId,
    pub path: Option<AssociatedItemPath>,
    pub im: Impl,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub associated_item_kind: AssociatedItemKind,
    pub accessibility: Accessibility,
    pub is_generic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct AssociatedItemId {
    impl_id: ImplId,
    ident: Ident,
}

impl AssociatedItemId {
    pub fn module_path(self) -> ModulePath {
        self.impl_id.module_path()
    }
}

impl AssociatedItem {
    pub fn new_impl_associated_item(
        db: &dyn EntityTreeDb,
        im: Impl,
        ast_idx: AstIdx,
        ident: Ident,
        associated_item_kind: AssociatedItemKind,
        accessibility: Accessibility,
        is_generic: bool,
    ) -> Self {
        let id = AssociatedItemId {
            impl_id: im.id(db),
            ident,
        };
        let path: Option<AssociatedItemPath> = match associated_item_kind {
            AssociatedItemKind::TraitItem(_) => todo!(),
            AssociatedItemKind::TypeItem(ty_item_kind) => match im.kind(db) {
                ImplKind::Type { ty } => {
                    Some(TypeItemPath::new(db, ty, ident, ty_item_kind).into())
                }
                ImplKind::Err => None,
                _ => unreachable!(),
            },
            AssociatedItemKind::TypeAsTraitItem(ty_as_trai_item_kind) => match im.kind(db) {
                ImplKind::TypeAsTrait { ty, trai } => {
                    Some(TypeAsTraitItemPath::new(db, ty, trai, ident, ty_as_trai_item_kind).into())
                }
                ImplKind::Err => None,
                _ => {
                    p!(im.kind(db));
                    unreachable!()
                }
            },
        };
        Self::new(
            db,
            id,
            path,
            im,
            ast_idx,
            ident,
            associated_item_kind,
            accessibility,
            is_generic,
        )
    }

    pub fn module_path(&self, db: &dyn EntityTreeDb) -> ModulePath {
        self.id(db).impl_id.module_path()
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

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn impl_associated_items(
    db: &dyn EntityTreeDb,
    im: Impl,
) -> IdentPairMap<AssociatedItem> {
    let body = im.body(db);
    let ast_sheet = db.ast_sheet(im.module_path(db)).unwrap();
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
                            im,
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
                    p!(im.debug(db));
                    p!(ast);
                    unreachable!()
                }
            }
        })
        .collect()
}
