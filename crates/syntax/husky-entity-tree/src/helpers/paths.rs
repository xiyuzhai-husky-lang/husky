use super::*;

pub trait HasItemPaths: Copy {
    fn item_paths<'a>(self, _db: &'a ::salsa::Db) -> &'a [ItemPath];
}

impl HasItemPaths for ModulePath {
    fn item_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [ItemPath] {
        module_item_paths(db, self)
    }
}

impl HasItemPaths for CratePath {
    fn item_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [ItemPath] {
        crate_item_paths(db, self)
    }
}

/// include everything defined under a module,
/// submodules, major items, associated items, impl blocks, attrs
#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub fn module_item_syn_node_paths(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<ItemSynNodePath> {
    let mut syn_node_paths: Vec<ItemSynNodePath> = Default::default();
    let item_tree_sheet = db.item_syn_tree_sheet(module_path);
    let mut push_with_attrs = |syn_node_path| {
        syn_node_paths.push(syn_node_path);
        for &(attr_syn_node_path, _) in syn_node_path.attr_syn_nodes(db) {
            syn_node_paths.push(attr_syn_node_path.into())
        }
    };
    for syn_node_path in item_tree_sheet.major_item_syn_node_paths() {
        push_with_attrs(syn_node_path);
        match syn_node_path {
            ItemSynNodePath::MajorItem(MajorItemSynNodePath::Trait(trai_node_path)) => {
                for trai_item_syn_node_path in trai_node_path.item_node_paths(db) {
                    push_with_attrs(trai_item_syn_node_path.into())
                }
            }
            ItemSynNodePath::MajorItem(MajorItemSynNodePath::Type(ty_node_path)) => {
                for ty_variant_syn_node_path in ty_node_path.ty_variant_syn_node_paths(db) {
                    push_with_attrs(ty_variant_syn_node_path.into())
                }
            }
            _ => (),
        }
    }
    for impl_block_syn_node_path in item_tree_sheet.impl_block_syn_node_paths() {
        push_with_attrs(impl_block_syn_node_path.into());
        match impl_block_syn_node_path {
            ImplBlockSynNodePath::TypeImplBlock(impl_block_syn_node_path) => {
                for syn_node_path in impl_block_syn_node_path.item_syn_node_paths(db) {
                    push_with_attrs(syn_node_path.into())
                }
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(impl_block_syn_node_path) => {
                for syn_node_path in impl_block_syn_node_path.item_syn_node_paths(db) {
                    push_with_attrs(syn_node_path.into())
                }
            }
            ImplBlockSynNodePath::IllFormedImplBlock(impl_block_syn_node_path) => {
                for syn_node_path in impl_block_syn_node_path.item_syn_node_paths(db) {
                    push_with_attrs(syn_node_path.into())
                }
            }
        }
    }
    syn_node_paths
}

/// include everything defined under a module,
/// submodules, major items, associated items, impl blocks, attrs
#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub fn module_item_paths(db: &::salsa::Db, module_path: ModulePath) -> Vec<ItemPath> {
    module_item_syn_node_paths(db, module_path)
        .iter()
        .filter_map(|syn_node_path| syn_node_path.unambiguous_item_path(db))
        .collect()
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub fn crate_item_paths(db: &::salsa::Db, crate_path: CratePath) -> Vec<ItemPath> {
    crate_path
        .module_paths(db)
        .iter()
        .flat_map(|module_path| module_path.item_paths(db).iter().copied())
        .collect()
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub fn module_submodule_item_paths(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<SubmoduleItemPath> {
    module_item_paths(db, module_path)
        .iter()
        .copied()
        .filter_map(|item_path| match item_path {
            ItemPath::Submodule(_, submodule_item_path) => Some(submodule_item_path),
            _ => None,
        })
        .collect()
}

#[test]
fn module_item_paths_works() {
    DB::ast_expect_test_debug_with_db(
        |db, module_path| module_item_paths(db, module_path),
        &AstTestConfig::new(
            "module_item_paths",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    )
}

#[test]
fn item_path_id_conversion_works() {
    DB::vfs_plain_test(
        |db, module_path| {
            for &item_path in module_item_paths(db, module_path) {
                assert_eq!(item_path.item_path(db), item_path);
            }
        },
        &AstTestConfig::new(
            "item_path_id_conversion",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    )
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub fn crate_module_paths(db: &::salsa::Db, crate_path: CratePath) -> Vec<ModulePath> {
    let root_module_path = crate_path.root_module_path(db);
    let mut module_paths = vec![];
    collect_module_paths(root_module_path, &mut module_paths, db);
    module_paths
}

pub trait HasModulePaths: is::Is<CratePath> {
    fn module_paths<'a>(self, db: &'a salsa::Db) -> &'a [ModulePath];
}

impl<T> HasModulePaths for T
where
    T: is::Is<CratePath>,
{
    fn module_paths<'a>(self, db: &'a salsa::Db) -> &'a [ModulePath] {
        let slf = self.into();
        crate_module_paths(db, slf)
    }
}

pub fn collect_module_paths(
    module_path: ModulePath,
    module_paths: &mut Vec<ModulePath>,
    db: &::salsa::Db,
) {
    module_paths.push(module_path);
    for item_path in module_item_paths(db, module_path) {
        match item_path {
            ItemPath::Submodule(_, submodule_path) => {
                collect_module_paths(submodule_path.self_module_path(db), module_paths, db)
            }
            _ => (),
        }
    }
}

#[test]
fn crate_module_paths_works() {
    DB::vfs_expect_test_debug_with_db(
        |db, crate_path: CratePath| crate_module_paths(db, crate_path),
        &VfsTestConfig::new(
            "crate_module_paths",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SYNTAX,
        ),
    )
}
