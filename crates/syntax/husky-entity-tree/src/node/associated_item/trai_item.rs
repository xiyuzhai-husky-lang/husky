use super::*;
use smallvec::SmallVec;

#[salsa::interned(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TraitItemSynNodePath {
    maybe_ambiguous_path: MaybeAmbiguousPath<TraitItemPath>,
}

impl TraitItemSynNodePath {
    fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
        path: TraitItemPath,
    ) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &dyn EntitySynTreeDb) -> Option<TraitItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn item_kind(self, db: &dyn EntitySynTreeDb) -> TraitItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub fn node(self, db: &dyn EntitySynTreeDb) -> TraitItemSynNode {
        todo!()
    }
}

impl HasSynNodePath for TraitItemPath {
    type SynNodePath = TraitItemSynNodePath;

    fn syn_node_path(self, db: &dyn EntitySynTreeDb) -> Self::SynNodePath {
        TraitItemSynNodePath::new_inner(db, MaybeAmbiguousPath::from_path(self))
    }
}

#[salsa::tracked(db = EntitySynTreeDb, jar = EntitySynTreeJar, constructor = new_inner)]
pub struct TraitItemSynNode {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub item_kind: TraitItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TraitItemSynNode {
    #[inline(always)]
    fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
        trai_node_path: TraitSynNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TraitItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TraitItemSynNodePath, Self) {
        let trai_item_path = TraitItemPath::new(db, trai_node_path.path(db), ident, item_kind);
        let syn_node_path = TraitItemSynNodePath::new(db, registry, trai_item_path);
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

    pub fn module_path(self, db: &dyn EntitySynTreeDb) -> ModulePath {
        self.syn_node_path(db).module_path(db)
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn trai_item_syn_nodes(
    db: &dyn EntitySynTreeDb,
    trai_node_path: TraitSynNodePath,
) -> SmallVec<
    [(Ident, TraitItemSynNodePath, TraitItemSynNode);
        APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS],
> {
    let trai_node = trai_node_path.node(db);
    let module_path = trai_node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    let DefnBlock::Trait { path, items } = trai_node.block(db) else {
        unreachable!()
    };
    let Some(items) = items else {
        return Default::default()
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
                    let EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TraitItem(item_kind),
                    } = entity_kind else {
                        unreachable!()
                    };
                    let (syn_node_path, node) = TraitItemSynNode::new(
                        db,
                        &mut registry,
                        trai_node_path,
                        ast_idx,
                        ident_token.ident(),
                        *item_kind,
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

pub(crate) const APPROXIMATE_UPPER_BOUND_ON_NUMBER_OF_TRAIT_ITEMS: usize = 4;
