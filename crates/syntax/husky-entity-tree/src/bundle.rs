use crate::*;
use vec_like::VecMap;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityTreeBundle {
    sheets: VecMap<ModulePath, EntityTreeSheet>,
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_tree_bundle(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> VfsResult<EntityTreeBundle> {
    todo!()
}
