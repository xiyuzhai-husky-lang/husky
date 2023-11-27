use crate::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct SynNodeDeclSheet {
    #[return_ref]
    pub decls: Vec<(ItemSynNodePath, ItemSynNodeDecl)>,
}

pub trait HasSynNodeDeclSheet: Copy {
    fn syn_node_decl_sheet(self, db: &::salsa::Db) -> SynNodeDeclSheet;
}

impl HasSynNodeDeclSheet for ModulePath {
    fn syn_node_decl_sheet(self, db: &::salsa::Db) -> SynNodeDeclSheet {
        syn_node_decl_sheet(db, self)
    }
}

// useful for diagnostics and testing
#[salsa::tracked(jar = SynDeclJar)]
pub fn syn_node_decl_sheet(db: &::salsa::Db, path: ModulePath) -> SynNodeDeclSheet {
    let item_tree_sheet = db.item_syn_tree_sheet(path);
    let mut decls: Vec<(ItemSynNodePath, ItemSynNodeDecl)> = Default::default();
    for syn_node_path in item_tree_sheet.major_item_syn_node_paths() {
        decls.push((syn_node_path, syn_node_path.syn_node_decl(db)))
    }
    // todo: handle trait items
    for impl_block_syn_node_path in item_tree_sheet.impl_block_syn_node_paths() {
        decls.push((
            impl_block_syn_node_path.into(),
            impl_block_syn_node_path.syn_node_decl(db).into(),
        ));
        match impl_block_syn_node_path {
            ImplBlockSynNodePath::TypeImplBlock(impl_block_syn_node_path) => {
                for item_syn_node_path in impl_block_syn_node_path.item_syn_node_paths(db) {
                    decls.push((
                        item_syn_node_path.into(),
                        item_syn_node_path.syn_node_decl(db).into(),
                    ))
                }
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(impl_block_syn_node_path) => {
                for item_syn_node_path in impl_block_syn_node_path.item_syn_node_paths(db) {
                    decls.push((
                        item_syn_node_path.into(),
                        item_syn_node_path.syn_node_decl(db).into(),
                    ))
                }
            }
            ImplBlockSynNodePath::IllFormedImplBlock(impl_block_syn_node_path) => {
                for item_syn_node_path in impl_block_syn_node_path
                    .item_syn_node_paths(db)
                    .iter()
                    .copied()
                {
                    todo!()
                    // decls.push((
                    //     item_syn_node_path.into(),
                    //     item_syn_node_path.syn_node_decl(db).into(),
                    // ))
                }
            }
        }
    }
    SynNodeDeclSheet::new(db, decls)
}

#[test]
fn syn_node_decl_sheet_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db(
        |db, module_path| db.syn_node_decl_sheet(module_path),
        &AstTestConfig::new("syn_node_decl_sheet"),
    );
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct SynDeclSheet {
    #[return_ref]
    pub decls: Vec<(ItemPath, SynDecl)>,
}

// only useful for testing purposes
#[salsa::tracked(jar = SynDeclJar)]
pub fn syn_decl_sheet(db: &::salsa::Db, path: ModulePath) -> SynDeclSheet {
    // get decls through item paths
    let item_tree_sheet = db.item_syn_tree_sheet(path);
    let mut decls: Vec<(ItemPath, SynDecl)> = Default::default();
    for syn_node_path in item_tree_sheet.major_item_syn_node_paths() {
        if let Some(path) = syn_node_path.path(db)
            && let Ok(decl) = path.syn_decl(db)
        {
            decls.push((path, decl))
        }
    }
    // todo: trait item
    for syn_node_path in item_tree_sheet.impl_block_syn_node_paths() {
        if let Some(path) = syn_node_path.path(db)
            && let Ok(decl) = path.syn_decl(db)
        {
            decls.push((path.into(), decl.into()));
            match path {
                ImplBlockPath::TypeImplBlock(path) => {
                    for syn_node_path in path.syn_node_path(db).item_syn_node_paths(db) {
                        if let Some(path) = syn_node_path.path(db)
                            && let Ok(decl) = path.syn_decl(db)
                        {
                            decls.push((path.into(), decl.into()))
                        }
                    }
                }
                ImplBlockPath::TraitForTypeImplBlock(path) => {
                    for syn_node_path in path.syn_node_path(db).item_syn_node_paths(db) {
                        if let Some(path) = syn_node_path.path(db)
                            && let Ok(decl) = path.syn_decl(db)
                        {
                            decls.push((path.into(), decl.into()))
                        }
                    }
                }
            }
        }
    }
    SynDeclSheet::new(db, decls)
}

#[test]
fn syn_decl_sheet_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db(
        |db, module_path| db.syn_decl_sheet(module_path),
        &AstTestConfig::new("syn_decl_sheet"),
    );
}
