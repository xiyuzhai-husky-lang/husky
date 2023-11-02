use super::*;

// include submodules, major items, associated items
#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub fn module_item_syn_node_paths(
    db: &dyn EntitySynTreeDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<ItemSynNodePath>> {
    let mut node_paths: Vec<ItemSynNodePath> = Default::default();
    let item_tree_sheet = db.item_syn_tree_sheet(module_path)?;
    let mut push = |syn_node_path| {
        node_paths.push(syn_node_path);
        for &(attr_syn_node_path, _) in syn_node_path.attr_syn_nodes(db) {
            node_paths.push(attr_syn_node_path.into())
        }
    };
    for syn_node_path in item_tree_sheet.major_item_syn_node_paths() {
        push(syn_node_path);
        // ignore this for now because I'm lazy
        // match syn_node_path {
        //     ItemSynNodePath::MajorItem(MajorItemSynNodePath::Trait(trai_node_path)) => {
        //         for trai_item_syn_node_path in trai_node_path.item_node_paths(db) {
        //             node_paths.push(trai_item_syn_node_path.into())
        //         }
        //     }
        //     _ => (),
        // }
    }
    // todo: trait item
    for impl_block_syn_node_path in item_tree_sheet.impl_block_syn_node_paths() {
        push(impl_block_syn_node_path.into());
        match impl_block_syn_node_path {
            ImplBlockSynNodePath::TypeImplBlock(impl_block_syn_node_path) => {
                for syn_node_path in impl_block_syn_node_path.item_syn_node_paths(db) {
                    push(syn_node_path.into())
                }
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(impl_block_syn_node_path) => {
                for syn_node_path in impl_block_syn_node_path.item_syn_node_paths(db) {
                    push(syn_node_path.into())
                }
            }
            ImplBlockSynNodePath::IllFormedImplBlock(impl_block_syn_node_path) => {
                for syn_node_path in impl_block_syn_node_path
                    .item_syn_node_paths(db)
                    .iter()
                    .copied()
                {
                    push(syn_node_path.into())
                }
            }
        }
    }
    Ok(node_paths)
}

// include submodules, module items, associated items
// todo: type variants
// todo: trait item
#[salsa::tracked(jar = EntitySynTreeJar, return_ref)]
pub fn module_item_paths(
    db: &dyn EntitySynTreeDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<ItemPath>> {
    let mut paths: Vec<ItemPath> = Default::default();
    let item_tree_sheet = db.item_syn_tree_sheet(module_path)?;
    for syn_node_path in item_tree_sheet.major_item_syn_node_paths() {
        if let Some(path) = syn_node_path.path(db) {
            paths.push(path)
        }
    }
    for syn_node_path in item_tree_sheet.impl_block_syn_node_paths() {
        if let Some(path) = syn_node_path.path(db) {
            paths.push(path.into());
            match path {
                ImplBlockPath::TypeImplBlock(path) => {
                    for syn_node_path in path.syn_node_path(db).item_syn_node_paths(db) {
                        if let Some(path) = syn_node_path.path(db) {
                            paths.push(path.into())
                        }
                    }
                }
                ImplBlockPath::TraitForTypeImplBlock(path) => {
                    for syn_node_path in path.syn_node_path(db).item_syn_node_paths(db) {
                        if let Some(path) = syn_node_path.path(db) {
                            paths.push(path.into())
                        }
                    }
                }
            }
        }
    }
    Ok(paths)
}

#[test]
fn module_item_paths_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        |db, module_path| module_item_paths(db, module_path),
        &AstTestConfig::new("module_item_paths"),
    )
}
