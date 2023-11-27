use super::*;

use husky_entity_kind::TypeKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
pub struct TypeVariantSynNodePath(ItemSynNodePathId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVariantSynNodePathData {
    pub parent_ty_node_path: TypeSynNodePath,
    maybe_ambiguous_path: MaybeAmbiguousPath<TypeVariantPath>,
}

impl TypeVariantSynNodePath {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        parent_ty_node_path: TypeSynNodePath,
        ty_variant_path: TypeVariantPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::TypeVariant(TypeVariantSynNodePathData {
                parent_ty_node_path,
                maybe_ambiguous_path: registry.issue_maybe_ambiguous_path(ty_variant_path),
            }),
        ))
    }

    pub fn path(self, db: &::salsa::Db) -> Option<TypeVariantPath> {
        Some(match self.0.path(db)? {
            ItemPath::TypeVariant(_, path) => path,
            _ => unreachable!(),
        })
    }

    pub fn data(self, db: &::salsa::Db) -> TypeVariantSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::TypeVariant(data) => data,
            _ => unreachable!(),
        }
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a TypeVariantSynNode {
        self.data(db)
            .parent_ty_node_path
            .ty_variant_syn_nodes(db)
            .iter()
            .find_map(|&(_, syn_node_path, ref node)| (self == syn_node_path).then_some(node))
            .unwrap()
    }
}

impl TypeVariantSynNodePathData {
    pub fn path(self, db: &::salsa::Db) -> Option<TypeVariantPath> {
        self.maybe_ambiguous_path.unambiguous_path()
    }
}

// impl HasModulePath<Db> for TypeVariantSynNodePath
// where
//      + EntitySynTreeDb,
// {
//     fn module_path(self, db: &::salsa::Db,) -> ModulePath {
//         let db = entity_syn_tree_db(db);
//         self.maybe_ambiguous_path(db).path.module_path(db)
//     }
// }

impl TypeSynNodePath {
    fn ty_variant_syn_nodes<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [(Ident, TypeVariantSynNodePath, TypeVariantSynNode)] {
        ty_variant_syn_nodes(db, self)
    }
}

impl HasSynNodePath for TypeVariantPath {
    type SynNodePath = TypeVariantSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TypeVariantSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::TypeVariant(TypeVariantSynNodePathData {
                parent_ty_node_path: self.parent_ty_path(db).syn_node_path(db),
                maybe_ambiguous_path: MaybeAmbiguousPath::from_path(self),
            }),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct TypeVariantSynNode {
    pub(crate) syn_node_path: TypeVariantSynNodePath,
    pub(crate) ast_idx: AstIdx,
    pub(crate) ident_token: IdentToken,
}

impl TypeVariantSynNode {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        ty_node_path: TypeSynNodePath,
        ty_variant_path: TypeVariantPath,
        ast_idx: AstIdx,
        ident_token: IdentToken,
    ) -> (TypeVariantSynNodePath, Self) {
        let syn_node_path =
            TypeVariantSynNodePath::new(db, registry, ty_node_path, ty_variant_path);
        (
            syn_node_path,
            TypeVariantSynNode {
                syn_node_path,
                ast_idx,
                ident_token,
            },
        )
    }
}

#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn ty_variant_syn_nodes(
    db: &::salsa::Db,
    ty_node_path: TypeSynNodePath,
) -> Vec<(Ident, TypeVariantSynNodePath, TypeVariantSynNode)> {
    let module_path: ModulePath = ty_node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path);
    match ty_node_path.ty_kind(db) {
        TypeKind::Enum | TypeKind::Inductive => (),
        _ => return vec![],
    }
    let mut registry = ItemSynNodePathRegistry::default();
    let Ast::Identifiable {
        block: DefnBlock::Type { variants, .. },
        ..
    } = ast_sheet[ty_node_path.syn_node(db).ast_idx]
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
                variant_path,
                ident_token,
                ..
            } => {
                let ident = ident_token.ident();
                let (syn_node_path, node) = TypeVariantSynNode::new(
                    db,
                    &mut registry,
                    ty_node_path,
                    *variant_path,
                    ast_idx,
                    *ident_token,
                );
                (ident, syn_node_path, node)
            }
            _ => unreachable!(),
        })
        .collect()
}

pub trait HasTypeVariantPaths: Copy {
    fn ty_variant_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [(Ident, TypeVariantPath)];
}

impl HasTypeVariantPaths for TypePath {
    fn ty_variant_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [(Ident, TypeVariantPath)] {
        ty_variant_paths(db, self)
    }
}

/// guaranteed that each ident is unique
#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub(crate) fn ty_variant_paths(db: &::salsa::Db, path: TypePath) -> Vec<(Ident, TypeVariantPath)> {
    path.syn_node_path(db)
        .ty_variant_syn_nodes(db)
        .iter()
        .filter_map(|&(ident, variant_node_path, _)| Some((ident, variant_node_path.path(db)?)))
        .collect()
}
