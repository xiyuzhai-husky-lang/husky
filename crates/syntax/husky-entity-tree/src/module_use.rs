use crate::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct ModuleUseSheet {
    module_uses: HashMap<EntityPath, VfsResult<Vec<EntityTree>>>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_use_sheet(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> PrimalModuleUseSheet {
    todo!()
}

pub struct ModuleUseBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    root: EntityPath,
    aliases: HashMap<EntityPath, VfsResult<Vec<EntityTree>>>,
    modified_modules: HashSet<EntityPath>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}
