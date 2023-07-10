use super::*;
use husky_coword::IdentPairMap;
use husky_entity_taxonomy::TypeKind;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeVariantNodePath {
    pub parent_ty_node_path: TypeNodePath,
    pub maybe_ambiguous_path: MaybeAmbiguousPath<TypeVariantPath>,
}

impl TypeVariantNodePath {
    fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        ty_node_path: TypeNodePath,
        ty_variant_path: TypeVariantPath,
    ) -> Self {
        Self::new_inner(
            db,
            ty_node_path,
            registry.issue_maybe_ambiguous_path(ty_variant_path),
        )
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> TypeVariantNode {
        ty_variant_node(db, self)
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<TypeVariantPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }
}

impl TypeNodePath {
    fn ty_variant_nodes<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> &'a [(Ident, TypeVariantNodePath, TypeVariantNode)] {
        ty_variant_nodes(db, self)
    }
}

impl HasNodePath for TypeVariantPath {
    type NodePath = TypeVariantNodePath;

    fn node_path(self, db: &dyn EntityTreeDb) -> Self::NodePath {
        TypeVariantNodePath::new_inner(
            db,
            self.parent_ty_path(db).node_path(db),
            MaybeAmbiguousPath::from_path(self),
        )
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeVariantNode {
    #[id]
    pub node_path: TypeVariantNodePath,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

impl TypeVariantNode {
    fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        ty_node_path: TypeNodePath,
        ty_variant_path: TypeVariantPath,
        ast_idx: AstIdx,
        ident_token: IdentToken,
    ) -> (TypeVariantNodePath, Self) {
        let node_path = TypeVariantNodePath::new(db, registry, ty_node_path, ty_variant_path);
        (
            node_path,
            Self::new_inner(db, node_path, ast_idx, ident_token),
        )
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_variant_nodes(
    db: &dyn EntityTreeDb,
    ty_node_path: TypeNodePath,
) -> Vec<(Ident, TypeVariantNodePath, TypeVariantNode)> {
    let module_path = ty_node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path).expect("valid module");
    match ty_node_path.ty_kind(db) {
        TypeKind::Enum | TypeKind::Inductive => (),
        _ => return vec![],
    }
    let mut registry = EntityNodeRegistry::default();
    let Ast::Defn {
        block: DefnBlock::Type { variants, .. },
        ..
    } = ast_sheet[ty_node_path.node(db).ast_idx(db)]
    else {
        unreachable!()
    };
    let Some(variants) = variants else {
        return vec![];
    };
    ast_sheet
        .indexed_iter(variants.ast_idx_range())
        .map(|(ast_idx, variant_ast)| match variant_ast {
            Ast::TypeVariant {
                token_group_idx,
                variant_path,
                ident_token,
                ..
            } => {
                let ident = ident_token.ident();
                let (node_path, node) = TypeVariantNode::new(
                    db,
                    &mut registry,
                    ty_node_path,
                    *variant_path,
                    ast_idx,
                    *ident_token,
                );
                (ident, node_path, node)
            }
            _ => unreachable!(),
        })
        .collect()
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn ty_variant_node(
    db: &dyn EntityTreeDb,
    node_path: TypeVariantNodePath,
) -> TypeVariantNode {
    node_path
        .parent_ty_node_path(db)
        .ty_variant_nodes(db)
        .iter()
        .copied()
        .find_map(|(_, node_path1, node)| (node_path == node_path1).then_some(node))
        .unwrap()
}
pub trait HasTypeVariantPaths: Copy {
    fn ty_variant_paths<'a>(self, db: &'a dyn EntityTreeDb) -> &'a [(Ident, TypeVariantPath)];
}

impl HasTypeVariantPaths for TypePath {
    fn ty_variant_paths<'a>(self, db: &'a dyn EntityTreeDb) -> &'a [(Ident, TypeVariantPath)] {
        ty_variant_paths(db, self)
    }
}

/// guaranteed that each ident is unique
#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_variant_paths(
    db: &dyn EntityTreeDb,
    path: TypePath,
) -> Vec<(Ident, TypeVariantPath)> {
    path.node_path(db)
        .ty_variant_nodes(db)
        .iter()
        .copied()
        .filter_map(|(ident, variant_node_path, _)| Some((ident, variant_node_path.path(db)?)))
        .collect()
}
