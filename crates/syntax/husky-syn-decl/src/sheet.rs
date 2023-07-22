use crate::*;
use vec_like::VecPairMap;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar, constructor = new)]
pub struct NodeDeclSheet {
    #[return_ref]
    pub decls: Vec<(EntitySynNodePath, SynNodeDecl)>,
}

pub trait HasNodeDeclSheet: Copy {
    fn node_decl_sheet(self, db: &dyn DeclDb) -> EntitySynTreeResult<NodeDeclSheet>;
}

impl HasNodeDeclSheet for ModulePath {
    fn node_decl_sheet(self, db: &dyn DeclDb) -> EntitySynTreeResult<NodeDeclSheet> {
        node_decl_sheet(db, self)
    }
}

// useful for diagnostics and testing
#[salsa::tracked(jar = SynDeclJar)]
pub fn node_decl_sheet(db: &dyn DeclDb, path: ModulePath) -> EntitySynTreeResult<NodeDeclSheet> {
    let entity_tree_sheet = db.entity_syn_tree_sheet(path)?;
    let mut decls: Vec<(EntitySynNodePath, SynNodeDecl)> = Default::default();
    for syn_node_path in entity_tree_sheet.major_entity_node_paths() {
        decls.push((syn_node_path, syn_node_path.node_decl(db)))
    }
    // todo: handle trait items
    for impl_block_syn_node_path in entity_tree_sheet.impl_block_syn_node_paths() {
        decls.push((impl_block_syn_node_path.into(), impl_block_syn_node_path.node_decl(db).into()));
        match impl_block_syn_node_path {
            ImplBlockSynNodePath::TypeImplBlock(impl_block_syn_node_path) => {
                for item_node_path in impl_block_syn_node_path.item_node_paths(db) {
                    decls.push((item_node_path.into(), item_node_path.node_decl(db).into()))
                }
            }
            ImplBlockSynNodePath::TraitForTypeImplBlock(impl_block_syn_node_path) => {
                for item_node_path in impl_block_syn_node_path.item_node_paths(db) {
                    decls.push((item_node_path.into(), item_node_path.node_decl(db).into()))
                }
            }
            ImplBlockSynNodePath::IllFormedImplBlock(impl_block_syn_node_path) => { 
                for item_node_path in
                    impl_block_syn_node_path.item_node_paths(db).iter().copied()
                {
                    decls.push((
                        item_node_path.into(),
                        item_node_path.node_decl(db).into(),
                    ))
                }
            }
        }
    }
    Ok(NodeDeclSheet::new(db, decls))
}

#[test]
fn node_decl_sheet_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db("node_decl_sheet", DeclDb::node_decl_sheet);
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar, constructor = new)]
pub struct DeclSheet {
    #[return_ref]
    pub decls: Vec<(EntityPath, Decl)>,
}

// only useful for testing purposes
#[salsa::tracked(jar = SynDeclJar)]
pub fn decl_sheet(db: &dyn DeclDb, path: ModulePath) -> EntitySynTreeResult<DeclSheet> {
    // get decls through entity paths
    let entity_tree_sheet = db.entity_syn_tree_sheet(path)?;
    let mut decls: Vec<(EntityPath, Decl)> = Default::default();
    for syn_node_path in entity_tree_sheet.major_entity_node_paths() {
        if let Some(path) = syn_node_path.path(db) && let Ok(decl) = path.decl(db) {
            decls.push((path, decl))
        }
    }
    // todo: trait item
    for syn_node_path in entity_tree_sheet.impl_block_syn_node_paths() {
        if let Some(path) = syn_node_path.path(db) && let Ok(decl) = path.decl(db) {
            decls.push((path.into(), decl.into()));
            match path {
                ImplBlockPath::TypeImplBlock(path) => {
                    for syn_node_path in path.syn_node_path(db).item_node_paths(db) {
                        if let Some(path) = syn_node_path.path(db) && let Ok(decl) = path.decl(db) {
                            decls.push((path.into(), decl.into()))
                        }
                    }
                }
                ImplBlockPath::TraitForTypeImplBlock(path) => {
                    for syn_node_path in path.syn_node_path(db).item_node_paths(db) { 
                        if let Some(path) = syn_node_path.path(db) && let Ok(decl) = path.decl(db) {
                            decls.push((path.into(), decl.into()))
                        }
                    }
                }
            }
        }
    }
    Ok(DeclSheet::new(db, decls))
}

#[test]
fn decl_sheet_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db("decl_sheet", DeclDb::decl_sheet);
}
