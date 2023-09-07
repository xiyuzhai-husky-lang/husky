use super::*;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TraitForTypeItemSynNodePath {
    maybe_ambiguous_path: MaybeAmbiguousPath<TraitForTypeItemPath>,
}

impl TraitForTypeItemSynNodePath {
    fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemSynNodePathRegistry,
        path: TraitForTypeItemPath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<TraitForTypeItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn impl_block(self, db: &dyn EntitySynTreeDb) -> TraitForTypeImplBlockSynNodePath {
        self.maybe_ambiguous_path(db)
            .path
            .impl_block(db)
            .syn_node_path(db)
    }

    pub fn item_kind(self, db: &dyn EntitySynTreeDb) -> TraitItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> TraitForTypeItemSynNode {
        trai_for_ty_item_syn_node(db, self)
    }
}

impl From<TraitForTypeItemSynNodePath> for ItemSynNodePath {
    fn from(id: TraitForTypeItemSynNodePath) -> Self {
        ItemSynNodePath::AssociatedItem(id.into())
    }
}

impl HasSynNodePath for TraitForTypeItemPath {
    type SynNodePath = TraitForTypeItemSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        TraitForTypeItemSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TraitForTypeItemSynNode {
    #[id]
    pub syn_node_path: TraitForTypeItemSynNodePath,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub item_kind: TraitItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TraitForTypeItemSynNode {
    #[inline(always)]
    fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut ItemSynNodePathRegistry,
        impl_block_syn_node_path: TraitForTypeImplBlockSynNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TraitItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TraitForTypeItemSynNodePath, Self) {
        let path = TraitForTypeItemPath::new(db, impl_block_syn_node_path.path(), ident, item_kind);
        let syn_node_path = TraitForTypeItemSynNodePath::new(db, registry, path);
        (
            syn_node_path,
            Self::new_inner(
                db,
                syn_node_path,
                ast_idx,
                ident,
                item_kind,
                visibility,
                is_generic,
            ),
        )
    }
}

#[salsa::tracked(jar = EntitySynTreeJar)]
pub(crate) fn trai_for_ty_item_syn_node(
    db: &dyn EntitySynTreeDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> TraitForTypeItemSynNode {
    syn_node_path
        .impl_block(db)
        .associated_items(db)
        .iter()
        .copied()
        .find_map(|(_, node_path1, node)| (node_path1 == syn_node_path).then_some(node))
        .expect("some")
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn trai_for_ty_impl_block_items(
    db: &dyn EntitySynTreeDb,
    impl_block_syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> Vec<(Ident, TraitForTypeItemSynNodePath, TraitForTypeItemSynNode)> {
    let impl_block_syn_node = impl_block_syn_node_path.node(db);
    let module_path = impl_block_syn_node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    let Some(items) = impl_block_syn_node.items(db) else {
        return vec![];
    };
    let mut registry = ItemSynNodePathRegistry::default();
    items
        .ast_idx_range()
        .into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Defn {
                    visibility_expr,
                    item_kind,
                    ident_token,
                    is_generic,
                    ..
                } => {
                    let item_kind = match item_kind {
                        EntityKind::AssociatedItem {
                            associated_item_kind: AssociatedItemKind::TraitForTypeItem(ty_item_kind),
                        } => *ty_item_kind,
                        _ => unreachable!(),
                    };
                    let (syn_node_path, node) = TraitForTypeItemSynNode::new(
                        db,
                        &mut registry,
                        impl_block_syn_node_path,
                        ast_idx,
                        ident_token.ident(),
                        item_kind,
                        visibility_expr.visibility(),
                        *is_generic,
                    );
                    Some((ident_token.ident(), syn_node_path, node))
                }
                Ast::Err { .. } => None,
                _ => unreachable!(),
            }
        })
        .collect()
}
