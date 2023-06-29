use crate::*;
use husky_print_utils::p;
use vec_like::VecPairMap;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntityTreeSheet {
    module_path: ModulePath,
    major_entity_node_table: MajorEntityNodeTable,
    entity_symbol_table: EntitySymbolTable,
    // todo: split this into ty impl block and trai for ty impl block
    impl_block_node_table: VecPairMap<ImplBlockNodePath, ImplBlockNode>,
    once_use_rules: OnceUseRules,
    use_all_rules: UseAllModuleSymbolsRules,
    errors: Vec<EntityTreeError>,
}

impl vec_like::AsVecMapEntry for EntityTreeSheet {
    type K = ModulePath;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.module_path
    }

    fn key_ref(&self) -> &Self::K {
        &self.module_path
    }
}

impl EntityTreeSheet {
    pub(crate) fn new(
        module_path: ModulePath,
        major_entity_node_table: MajorEntityNodeTable,
        entity_symbol_table: EntitySymbolTable,
        once_use_rules: OnceUseRules,
        use_all_rules: UseAllModuleSymbolsRules,
        errors: Vec<EntityTreeError>,
        impl_block_node_table: VecPairMap<ImplBlockNodePath, ImplBlockNode>,
    ) -> Self {
        Self {
            module_path,
            major_entity_node_table,
            entity_symbol_table,
            impl_block_node_table,
            once_use_rules,
            use_all_rules,
            errors,
        }
    }

    pub fn module_symbols<'a>(&'a self) -> EntitySymbolTableRef<'a> {
        self.entity_symbol_table.as_ref()
    }

    pub fn errors(&self) -> &[EntityTreeError] {
        &self.errors
    }

    pub fn once_use_rule_indexed_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (OnceUseRuleIdx, &'a OnceUseRule)> + 'a {
        self.once_use_rules.indexed_iter()
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn major_entity_node_paths<'a>(&'a self) -> impl Iterator<Item = EntityNodePath> + 'a {
        self.major_entity_node_table.node_paths()
    }

    pub fn major_entity_paths<'a>(
        &'a self,
        db: &'a dyn EntityTreeDb,
    ) -> impl Iterator<Item = EntityPath> + 'a {
        self.major_entity_node_table
            .node_paths()
            .filter_map(|node_path| node_path.path(db))
    }

    pub fn major_entity_node(&self, node_path: EntityNodePath) -> Option<EntityNode> {
        self.major_entity_node_table.node(node_path)
    }

    pub(crate) fn ty_impl_block_node(&self, node_path: TypeImplBlockNodePath) -> TypeImplBlockNode {
        self.impl_block_node_table
            .iter()
            .find_map(|(node_path1, node)| {
                (*node_path1 == node_path.into()).then(|| match node {
                    ImplBlockNode::TypeImplBlock(node) => *node,
                    _ => unreachable!(),
                })
            })
            .expect("valid node path")
    }

    pub(crate) fn trai_for_ty_impl_block_node(
        &self,
        db: &dyn EntityTreeDb,
        node_path: TraitForTypeImplBlockNodePath,
    ) -> TraitForTypeImplBlockNode {
        self.impl_block_node_table
            .iter()
            .find_map(|(node_path1, node)| {
                (*node_path1 == node_path.into()).then(|| match node {
                    ImplBlockNode::TraitForTypeImplBlock(node) => *node,
                    _ => unreachable!(),
                })
            })
            .expect("valid node path")
    }

    pub fn impl_block_node_paths<'a>(&'a self) -> impl Iterator<Item = ImplBlockNodePath> + 'a {
        self.impl_block_node_table
            .iter()
            .map(|(node_path, _)| *node_path)
    }

    pub fn all_ty_impl_block_node_paths<'a>(
        &'a self,
    ) -> impl Iterator<Item = TypeImplBlockNodePath> + 'a {
        self.impl_block_node_table
            .iter()
            .copied()
            .filter_map(|(node_path, _)| match node_path {
                ImplBlockNodePath::TypeImplBlock(node_path) => Some(node_path),
                _ => None,
            })
    }

    pub fn all_ty_impl_block_nodes<'a>(&'a self) -> impl Iterator<Item = TypeImplBlockNode> + 'a {
        self.impl_block_node_table
            .iter()
            .copied()
            .filter_map(|(_, impl_block)| match impl_block {
                ImplBlockNode::TypeImplBlock(impl_block) => Some(impl_block),
                _ => None,
            })
    }

    pub fn all_trai_for_ty_impl_block_paths<'a>(
        &'a self,
        db: &'a dyn EntityTreeDb,
    ) -> impl Iterator<Item = TraitForTypeImplBlockPath> + 'a {
        self.impl_block_node_table
            .iter()
            .copied()
            .filter_map(|(node_path, _)| match node_path.path(db)? {
                ImplBlockPath::TraitForTypeImplBlock(path) => Some(path),
                ImplBlockPath::TypeImplBlock(_) => None,
            })
    }

    pub fn all_trai_for_ty_impl_block_nodes<'a>(
        &'a self,
    ) -> impl Iterator<Item = TraitForTypeImplBlockNode> + 'a {
        self.impl_block_node_table
            .iter()
            .copied()
            .filter_map(|(_, impl_block)| match impl_block {
                ImplBlockNode::TypeImplBlock(_) => None,
                ImplBlockNode::TraitForTypeImplBlock(impl_block) => Some(impl_block),
                ImplBlockNode::IllFormedImplBlock(_) => None,
            })
    }

    pub fn all_ill_formed_impl_block_nodes<'a>(
        &'a self,
    ) -> impl Iterator<Item = IllFormedImplBlockNode> + 'a {
        self.impl_block_node_table
            .iter()
            .copied()
            .filter_map(|(_, impl_block)| match impl_block {
                ImplBlockNode::TypeImplBlock(_) => None,
                ImplBlockNode::TraitForTypeImplBlock(_) => None,
                ImplBlockNode::IllFormedImplBlock(impl_block) => Some(impl_block),
            })
    }
}

pub trait HasEntityTreeSheet: Copy {
    fn entity_tree_sheet<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeResult<&'a EntityTreeSheet>;
}

impl HasEntityTreeSheet for ModulePath {
    fn entity_tree_sheet<'a>(
        self,
        db: &'a dyn EntityTreeDb,
    ) -> EntityTreeResult<&'a EntityTreeSheet> {
        entity_tree_sheet(db, self)
    }
}

pub(crate) fn entity_tree_sheet(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<&EntityTreeSheet> {
    let crate_path = module_path.crate_path(db);
    let entity_tree_bundle = entity_tree_crate_bundle(db, crate_path)
        .as_ref()
        .map_err(|e| e.clone())?;
    entity_tree_bundle
        .get_sheet(module_path)
        .ok_or_else(|| DerivedEntityTreeError::InvalidModulePath(module_path).into())
}

#[test]
fn entity_tree_sheet_works() {
    DB::default().ast_expect_test_debug_with_db("entity_tree_sheet", |db, module_path| {
        entity_tree_sheet(db, module_path)
    })
}

// include submodules, module items, associated items
#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub fn module_entity_node_paths(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<EntityNodePath>> {
    let mut node_paths: Vec<EntityNodePath> = Default::default();
    let entity_tree_sheet = db.entity_tree_sheet(module_path)?;
    for node_path in entity_tree_sheet.major_entity_node_paths() {
        node_paths.push(node_path)
    }
    // todo: trait item
    for node_path in entity_tree_sheet.impl_block_node_paths() {
        match node_path {
            ImplBlockNodePath::TypeImplBlock(node_path) => {
                for node_path in node_path.item_node_paths(db) {
                    node_paths.push(node_path.into())
                }
            }
            ImplBlockNodePath::TraitForTypeImplBlock(node_path) => {
                for node_path in node_path.item_node_paths(db) {
                    node_paths.push(node_path.into())
                }
            }
            ImplBlockNodePath::IllFormedImplBlock(_) => todo!(),
        }
    }
    Ok(node_paths)
}

// include submodules, module items, associated items
#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub fn module_entity_paths(
    db: &dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<Vec<EntityPath>> {
    let mut paths: Vec<EntityPath> = Default::default();
    let entity_tree_sheet = db.entity_tree_sheet(module_path)?;
    for node_path in entity_tree_sheet.major_entity_node_paths() {
        if let Some(path) = node_path.path(db) {
            paths.push(path)
        }
    }
    // todo: trait item
    for node_path in entity_tree_sheet.impl_block_node_paths() {
        if let Some(path) = node_path.path(db) {
            paths.push(path.into());
            match path {
                ImplBlockPath::TypeImplBlock(path) => {
                    for node_path in path.node_path(db).item_node_paths(db) {
                        if let Some(path) = node_path.path(db) {
                            paths.push(path.into())
                        }
                    }
                }
                ImplBlockPath::TraitForTypeImplBlock(path) => {
                    for node_path in path.node_path(db).item_node_paths(db) {
                        if let Some(path) = node_path.path(db) {
                            paths.push(path.into())
                        }
                    }
                }
            }
        }
    }
    Ok(paths)
}
