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
    todo!();
    // for path in entity_tree_sheet.module_item_path_iter(db) {
    //     decls.push((DeclRegionPath::Entity(path.into()), path.decl(db)))
    // }
    // for impl_block in entity_tree_sheet.impl_blocks().iter().copied() {
    //     decls.push((
    //         DeclRegionPath::ImplBlock(impl_block.path(db)),
    //         impl_block.decl(db).map(|decl| decl.into()),
    //     ));
    //     for (_, associated_item) in impl_block.items(db).iter().copied() {
    //         decls.push((
    //             DeclRegionPath::AssociatedItem(associated_item.id(db)),
    //             associated_item.decl(db).map(|decl| decl.into()),
    //         ))
    //     }
    // }
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
    todo!();
    // for path in entity_tree_sheet.module_item_path_iter(db) {
    //     decls.push((DeclRegionPath::Entity(path.into()), path.decl(db)))
    // }
    // for impl_block in entity_tree_sheet.impl_blocks().iter().copied() {
    //     decls.push((
    //         DeclRegionPath::ImplBlock(impl_block.path(db)),
    //         impl_block.decl(db).map(|decl| decl.into()),
    //     ));
    //     for (_, associated_item) in impl_block.items(db).iter().copied() {
    //         decls.push((
    //             DeclRegionPath::AssociatedItem(associated_item.id(db)),
    //             associated_item.decl(db).map(|decl| decl.into()),
    //         ))
    //     }
    // }
    Ok(DeclSheet::new(db, decls))
}

#[test]
fn decl_sheet_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db("decl_sheet", DeclDb::decl_sheet);
}
