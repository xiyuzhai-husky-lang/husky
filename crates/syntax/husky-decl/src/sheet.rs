use crate::*;

use husky_entity_tree::DeclRegionPath;
use vec_like::VecPairMap;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclSheet<'a> {
    decls: Vec<(DeclRegionPath, DeclResultRef<'a, Decl>)>,
}

pub fn decl_sheet<'a>(db: &'a dyn DeclDb, path: ModulePath) -> EntityTreeResult<DeclSheet<'a>> {
    DeclSheet::collect_from_module(db, path)
}

#[test]
fn decl_sheet_works() {
    use tests::*;

    DB::default().ast_expect_test_debug_with_db("decl_sheet", DeclDb::decl_sheet);
}

impl<'a> DeclSheet<'a> {
    pub fn collect_from_module(db: &'a dyn DeclDb, path: ModulePath) -> EntityTreeResult<Self> {
        let entity_tree_sheet = db.entity_tree_sheet(path)?;
        let mut decls: Vec<(DeclRegionPath, DeclResultRef<'a, Decl>)> = Default::default();
        for path in entity_tree_sheet.module_item_path_iter(db) {
            decls.push((DeclRegionPath::Entity(path.into()), path.decl(db)))
        }
        for impl_block in entity_tree_sheet.impl_blocks().iter().copied() {
            decls.push((
                DeclRegionPath::ImplBlock(impl_block.id(db)),
                impl_block.decl(db).map(|decl| decl.into()),
            ));
            for (_, associated_item) in impl_block.items(db).iter().copied() {
                decls.push((
                    DeclRegionPath::AssociatedItem(associated_item.id(db)),
                    associated_item.decl(db).map(|decl| decl.into()),
                ))
            }
        }
        Ok(DeclSheet::new(decls))
    }

    fn new(decls: Vec<(DeclRegionPath, DeclResultRef<'a, Decl>)>) -> Self {
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
