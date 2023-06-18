use crate::*;
use vec_like::VecPairMap;

#[salsa::tracked(db = DeclDb, jar = DeclJar, constructor = new)]
pub struct NodeDeclSheet {
    #[return_ref]
    pub decls: Vec<(EntityNodePath, NodeDecl)>,
}

pub trait HasNodeDeclSheet: Copy {
    fn node_decl_sheet(self, db: &dyn DeclDb) -> EntityTreeResult<NodeDeclSheet>;
}

impl HasNodeDeclSheet for ModulePath {
    fn node_decl_sheet(self, db: &dyn DeclDb) -> EntityTreeResult<NodeDeclSheet> {
        node_decl_sheet(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub fn node_decl_sheet(db: &dyn DeclDb, path: ModulePath) -> EntityTreeResult<NodeDeclSheet> {
    let entity_tree_sheet = db.entity_tree_sheet(path)?;
    let mut decls: Vec<(EntityNodePath, NodeDecl)> = Default::default();
    for node_path in entity_tree_sheet.major_entity_node_paths() {
        decls.push((node_path, node_path.node_decl(db)))
    }
    // todo: handle trait items
    for impl_block_node_path in entity_tree_sheet.impl_block_node_paths() {
        decls.push((
            impl_block_node_path.into(),
            impl_block_node_path.node_decl(db).into(),
        ));
        match impl_block_node_path {
            ImplBlockNodePath::TypeImplBlock(impl_block_node_path) => {
                for ty_item_node_path in impl_block_node_path.item_node_paths(db).iter().copied() {
                    decls.push((
                        ty_item_node_path.into(),
                        ty_item_node_path.node_decl(db).into(),
                    ))
                }
            }
            ImplBlockNodePath::TraitForTypeImplBlock(impl_block_node_path) => {
                for trai_for_ty_item_node_path in
                    impl_block_node_path.item_node_paths(db).iter().copied()
                {
                    decls.push((
                        trai_for_ty_item_node_path.into(),
                        trai_for_ty_item_node_path.node_decl(db).into(),
                    ))
                }
            }
            ImplBlockNodePath::IllFormedImplBlock(impl_block_node_path) => {
                todo!()
                // for ill_formed_item_node_path in
                //     impl_block_node_path.item_node_paths(db).iter().copied()
                // {
                //     decls.push((
                //         ill_formed_item_node_path.into(),
                //         ill_formed_item_node_path.node_decl(db).into(),
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

pub trait HasDeclSheet: Copy {
    fn decl_sheet(self, db: &dyn DeclDb) -> EntityTreeResult<DeclSheet>;
}

impl HasDeclSheet for ModulePath {
    fn decl_sheet(self, db: &dyn DeclDb) -> EntityTreeResult<DeclSheet> {
        decl_sheet(db, self)
    }
}

#[salsa::tracked(jar = DeclJar)]
pub fn decl_sheet(db: &dyn DeclDb, path: ModulePath) -> EntityTreeResult<DeclSheet> {
    let entity_tree_sheet = db.entity_tree_sheet(path)?;
    let mut decls: Vec<(EntityPath, Decl)> = Default::default();
    for node_path in entity_tree_sheet.major_entity_node_paths() {
        if let Some(path) = node_path.path(db) && let Ok(decl) = path.decl(db) {
            decls.push((path, decl))
        }
    }
    todo!()
    // for impl_block_node in entity_tree_sheet.impl_block_nodes().iter().copied() {
    //     decls.push((
    //         DeclRegionPath::ImplBlock(impl_block_node.path(db)),
    //         impl_block_node.decl(db).map(|decl| decl.into()),
    //     ));
    //     for (_, associated_item) in impl_block_node.items(db).iter().copied() {
    //         decls.push((
    //             DeclRegionPath::AssociatedItem(associated_item.id(db)),
    //             associated_item.decl(db).map(|decl| decl.into()),
    //         ))
    //     }
    // }
    // Ok(DeclSheet::new(db, decls))
}

#[test]
fn decl_sheet_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db("decl_sheet", DeclDb::decl_sheet);
}
