use crate::*;

use husky_entity_tree::DeclRegionPath;
use vec_like::VecPairMap;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclSheet<'a> {
    decls: VecPairMap<DeclRegionPath, DeclResultRef<'a, Decl>>,
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
        let mut decls: VecPairMap<DeclRegionPath, DeclResultRef<'a, Decl>> = Default::default();
        for path in entity_tree_sheet.module_item_path_iter(db) {
            decls
                .insert_new((
                    DeclRegionPath::Entity(path.into()),
                    db.module_item_decl(path),
                ))
                .unwrap()
        }
        for im in entity_tree_sheet.ims().iter().copied() {
            decls
                .insert_new((
                    DeclRegionPath::Impl(im.id(db)),
                    db.impl_decl(im).map(|decl| decl.into()),
                ))
                .unwrap();
            for (_, associated_item) in db.impl_associated_items(im).iter().copied() {
                decls
                    .insert_new((
                        DeclRegionPath::AssociatedItem(associated_item.id(db)),
                        db.associated_item_decl(associated_item)
                            .map(|decl| decl.into()),
                    ))
                    .unwrap()
            }
        }
        Ok(DeclSheet::new(decls))
    }

    fn new(decls: VecPairMap<DeclRegionPath, DeclResultRef<'a, Decl>>) -> Self {
        Self { decls }
    }

    pub fn decls(&self) -> &[(DeclRegionPath, DeclResultRef<'a, Decl>)] {
        &self.decls
    }
}

impl<'a, Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for DeclSheet<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        f.debug_struct("DeclSheet")
            .field("decls", &self.decls.debug(db))
            .finish()
    }
}
