use crate::*;
use husky_entity_path::EntityPath;
use vec_like::VecPairMap;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclSheet {
    decls: VecPairMap<EntityPath, DeclResult<Decl>>,
}

impl DeclSheet {
    pub fn collect_from_module(db: &dyn DeclDb, path: ModulePath) -> EntityTreeResult<Self> {
        let entity_tree_sheet = db.entity_tree_sheet(path)?;
        let mut decls: VecPairMap<EntityPath, DeclResult<Decl>> = Default::default();
        for path in entity_tree_sheet.module_item_path_iter() {
            decls.insert((path.into(), db.module_item_decl(path)))
        }
        // self.parse_decl(*ast_idx, (*path).into()))
        for impl_block in entity_tree_sheet.impl_blocks() {
            // todo!()
        }
        Ok(DeclSheet::new(decls))
    }

    fn new(decls: VecPairMap<EntityPath, DeclResult<Decl>>) -> Self {
        Self { decls }
    }

    pub fn decls(&self) -> &VecPairMap<EntityPath, DeclResult<Decl>> {
        &self.decls
    }
}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for DeclSheet {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        f.debug_struct("DeclSheet")
            .field("decls", &(&self.decls.data()).debug(db))
            .finish()
    }
}
