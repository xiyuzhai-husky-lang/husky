use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TraitForTypeItemNodePath {
    maybe_ambiguous_path: MaybeAmbiguousPath<TraitForTypeItemPath>,
}

impl TraitForTypeItemNodePath {
    fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TraitForTypeItemPath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<TraitForTypeItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn impl_block(self, db: &dyn EntityTreeDb) -> TraitForTypeImplBlockNodePath {
        self.maybe_ambiguous_path(db)
            .path
            .impl_block(db)
            .node_path(db)
    }

    pub fn item_kind(self, db: &dyn EntityTreeDb) -> TraitItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> TraitForTypeItemNode {
        trai_for_ty_item_node(db, self)
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
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub item_kind: TraitItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TraitForTypeItemNode {
    #[inline(always)]
    fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        impl_block_node_path: TraitForTypeImplBlockNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TraitItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TraitForTypeItemNodePath, Self) {
        let path = TraitForTypeItemPath::new(db, impl_block_node_path.path(), ident, item_kind);
        let node_path = TraitForTypeItemNodePath::new(db, registry, path);
        (
            node_path,
            Self::new_inner(
                db, node_path, ast_idx, ident, item_kind, visibility, is_generic,
            ),
        )
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn trai_for_ty_item_node(
    db: &dyn EntityTreeDb,
    node_path: TraitForTypeItemNodePath,
) -> TraitForTypeItemNode {
    node_path
        .impl_block(db)
        .items(db)
        .iter()
        .copied()
        .find_map(|(_, node_path1, node)| (node_path1 == node_path).then_some(node))
        .expect("some")
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn trai_for_ty_impl_block_items(
    db: &dyn EntityTreeDb,
    impl_block_node_path: TraitForTypeImplBlockNodePath,
) -> Vec<(Ident, TraitForTypeItemNodePath, TraitForTypeItemNode)> {
    let impl_block_node = impl_block_node_path.node(db);
    let module_path = impl_block_node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    let Some(items) = impl_block_node.items(db) else {
        return vec![]
    };
    let mut registry = EntityNodeRegistry::default();
    items
        .ast_idx_range()
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
                    let item_kind = match entity_kind {
                        EntityKind::AssociatedItem {
                            associated_item_kind: AssociatedItemKind::TraitForTypeItem(ty_item_kind),
                        } => *ty_item_kind,
                        _ => unreachable!(),
                    };
                    let (node_path, node) = TraitForTypeItemNode::new(
                        db,
                        &mut registry,
                        impl_block_node_path,
                        ast_idx,
                        ident_token.ident(),
                        item_kind,
                        visibility_expr.visibility(),
                        *is_generic,
                    );
                    Some((ident_token.ident(), node_path, node))
                }
                Ast::Err { .. } => None,
                _ => unreachable!(),
            }
        })
        .collect()
}
