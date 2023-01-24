use crate::*;
use husky_entity_path::EntityPath;
use vec_like::VecPairMap;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclSheet<'a> {
    decls: Vec<DeclResultBorrowed<'a, Decl>>,
}

pub fn decl_sheet<'a>(db: &'a dyn DeclDb, path: ModulePath) -> EntityTreeResult<DeclSheet<'a>> {
    DeclSheet::collect_from_module(db, path)
}

#[test]
fn decl_sheet_works() {
    use husky_vfs::VfsTestUtils;
    use tests::*;

    DB::default().vfs_expect_test_debug_with_db("decl_sheet", DeclDb::decl_sheet);
}

impl<'a> DeclSheet<'a> {
    pub fn collect_from_module(db: &'a dyn DeclDb, path: ModulePath) -> EntityTreeResult<Self> {
        let entity_tree_sheet = db.entity_tree_sheet(path)?;
        let mut decls: Vec<DeclResultBorrowed<'a, Decl>> = Default::default();
        for path in entity_tree_sheet.module_item_path_iter(db) {
            decls.push(db.module_item_decl(path))
        }
        for impl_block in entity_tree_sheet.impl_blocks() {
            let impl_block = *impl_block;
            decls.push(db.impl_block_decl(impl_block).map(|decl| decl.into()));
            for (_, associated_item) in db.impl_block_associated_items(impl_block).iter() {
                decls.push(
                    db.associated_item_decl(*associated_item)
                        .map(|decl| decl.into()),
                )
            }
        }
        Ok(DeclSheet::new(decls))
    }

    fn new(decls: Vec<DeclResultBorrowed<'a, Decl>>) -> Self {
        Self { decls }
    }

    pub fn decls(&self) -> &[DeclResultBorrowed<'a, Decl>] {
        &self.decls
    }
}

impl<'a, Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for DeclSheet<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        f.debug_struct("DeclSheet")
            .field("decls", &self.decls.debug(db))
            .finish()
    }
}
