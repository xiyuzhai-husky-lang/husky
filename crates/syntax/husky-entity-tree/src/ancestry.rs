use crate::*;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn closest_module_in_apparent_ancestry(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<EntityPath> {
    Ok(match entity_path.data(db) {
        EntityPathData::CrateRoot(_) => entity_path,
        EntityPathData::Childpath { parent, ident } => {
            match entity_class(db, entity_path).as_ref()? {
                EntityCard::Module => entity_path,
                _ => *closest_module_in_apparent_ancestry(db, entity_path).as_ref()?,
            }
        }
    })
}
