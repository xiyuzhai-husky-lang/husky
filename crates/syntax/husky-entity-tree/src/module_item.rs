use husky_word::{IdentMap, IdentPairMap, Identifier};
use vec_like::VecMapEntry;

use crate::*;
use std::collections::HashMap;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_items_map(
    db: &dyn EntityTreeDb,
    craet_path: CratePath,
) -> VfsResult<HashMap<EntityPath, IdentMap<ModuleItem>>> {
    todo!()
    // let ast_sheet = db.ast_sheet(module).as_ref()?;
    // EntityTreeCollector1::new(db, module, ast_sheet).build()
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleItem {
    Defn {
        ident: Identifier,
        accessibility: Accessibility,
        tree_idx: EntityTreeIdx,
    },
    Use {
        ident: Identifier,
        accessibility: Accessibility,
        path: EntityPath,
        use_expr_idx: EntityUseExprIdx,
    },
}

impl VecMapEntry<Identifier> for ModuleItem {
    fn key(&self) -> Identifier {
        match self {
            ModuleItem::Defn { ident, .. } | ModuleItem::Use { ident, .. } => *ident,
        }
    }
}

struct EntityTreeCollector1<'a> {
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
    root: EntityPath,
    module_items_map: HashMap<EntityPath, IdentMap<ModuleItem>>,
    unresolved_use_alls: HashMap<EntityPath, Vec<EntityTreeIdx>>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

impl<'a> EntityTreeCollector1<'a> {
    fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> Self {
        todo!();
        // let primal_module_use_sheet = primal_module_use_sheet(db, crate_path);
        Self {
            db,
            crate_path,
            root: db.it_entity_path(EntityPathData::CrateRoot(crate_path)),
            module_items_map: todo!(),
            unresolved_use_alls: todo!(),
            errors: vec![],
        }
    }

    fn collect_all(&mut self) -> HashMap<EntityPath, IdentMap<ModuleItem>> {
        todo!();
        self.module_items_map
    }
}
