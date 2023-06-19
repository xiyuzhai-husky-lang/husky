use crate::*;
use vec_like::VecPairMap;

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct NodeDeclSheet {
    #[return_ref]
    pub decls: Vec<(EntityNodeId, NodeDecl)>,
}

pub trait HasNodeDeclSheet: Copy {
    fn node_decl_sheet(self, db: &dyn DeclDb) -> EntityTreeResult<NodeDeclSheet>;
}

impl HasNodeDeclSheet for ModulePath {
    fn node_decl_sheet(self, db: &dyn DeclDb) -> EntityTreeResult<NodeDeclSheet> {
        node_decl_sheet(db, self)
    }
}

// useful for diagnostics and testing
#[salsa::tracked(jar = DeclJar)]
pub fn node_decl_sheet(db: &dyn DeclDb, path: ModulePath) -> EntityTreeResult<NodeDeclSheet> {
    let entity_tree_sheet = db.entity_tree_sheet(path)?;
    let mut decls: Vec<(EntityNodeId, NodeDecl)> = Default::default();
    for node_id in entity_tree_sheet.major_entity_node_ids() {
        decls.push((node_id, node_id.node_decl(db)))
    }
    // todo: handle trait items
    for node_id in entity_tree_sheet.impl_block_node_ids() {
        decls.push((node_id.into(), node_id.node_decl(db).into()));
        match node_id {
            ImplBlockNodeId::TypeImplBlock(node_id) => {
                for node_id in node_id.item_node_ids(db).iter().copied() {
                    decls.push((node_id.into(), node_id.node_decl(db).into()))
                }
            }
            ImplBlockNodeId::TraitForTypeImplBlock(node_id) => {
                for node_id in node_id.item_node_ids(db).iter().copied() {
                    decls.push((node_id.into(), node_id.node_decl(db).into()))
                }
            }
            ImplBlockNodeId::IllFormedImplBlock(node_id) => {
                todo!()
                // for ill_formed_item_node_id in
                //     impl_block_node_id.item_node_ids(db).iter().copied()
                // {
                //     decls.push((
                //         ill_formed_item_node_id.into(),
                //         ill_formed_item_node_id.node_decl(db).into(),
                //     ))
                // }
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

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct DeclSheet {
    #[return_ref]
    pub decls: Vec<(EntityPath, Decl)>,
}

// only useful for testing purposes
#[salsa::tracked(jar = DeclJar)]
pub fn decl_sheet(db: &dyn DeclDb, path: ModulePath) -> EntityTreeResult<DeclSheet> {
    // get decls through entity paths
    let entity_tree_sheet = db.entity_tree_sheet(path)?;
    let mut decls: Vec<(EntityPath, Decl)> = Default::default();
    for node_id in entity_tree_sheet.major_entity_node_ids() {
        if let Some(path) = node_id.path(db) && let Ok(decl) = path.decl(db) {
            decls.push((path, decl))
        }
    }
    // todo: trait item
    for node_id in entity_tree_sheet.impl_block_node_ids() {
        if let Some(path) = node_id.path(db) && let Ok(decl) = path.decl(db) {
            decls.push(( path.into(), decl.into()));
            match path {
                ImplBlockPath::TypeImplBlock(path) => {
                    for node_id in path.node_id(db).item_node_ids(db).iter().copied() {
                        let path = node_id.path(db);
                        if let Ok(decl) = path.decl(db) {
                            decls.push(( path.into(), decl.into()))
                        }
                    }
                }
                ImplBlockPath::TraitForTypeImplBlock(path) => {
                    for node_id in path.node_id(db).item_node_ids(db).iter().copied() { 
                        if let Some(path) = node_id.path(db) &&let Ok(decl) = path.decl(db) {
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
