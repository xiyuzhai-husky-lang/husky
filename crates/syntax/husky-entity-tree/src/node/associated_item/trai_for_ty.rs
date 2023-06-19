use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitForTypeItemNodePath {
    maybe_ambiguous_path: MaybeAmbiguousPath<TraitForTypeItemPath>,
}

impl TraitForTypeItemNodePath {
    pub fn path(self, db: &dyn EntityTreeDb) -> Option<TraitForTypeItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }
}

impl From<TraitForTypeItemNodePath> for EntityNodePath {
    fn from(id: TraitForTypeItemNodePath) -> Self {
        EntityNodePath::AssociatedItem(id.into())
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitForTypeItemNode {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub impl_block: TraitForTypeImplBlockNode,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub kind: TraitItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TraitForTypeItemNode {
    pub fn new(
        db: &dyn EntityTreeDb,
        impl_block: TraitForTypeImplBlockNode,
        ast_idx: AstIdx,
        ident: Ident,
        kind: TraitItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> Self {
        todo!();
        // let id = TraitForTypeItemNodePath::new(db, todo!());
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
        Self::new_inner(
            db,
            todo!(),
            impl_block,
            ast_idx,
            ident,
            kind,
            visibility,
            is_generic,
        )
    }
}
