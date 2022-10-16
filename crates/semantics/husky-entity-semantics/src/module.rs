use crate::*;

pub fn module_defn(
    db: &dyn EntityDefnQueryGroup,
    entity_route: EntityRoutePtr,
    file: FileItd,
) -> SemanticResultArc<EntityDefn> {
    let opt_main_defn = if file.ends_with("main.hsy") {
        Some(db.main_defn(file)?)
    } else {
        None
    };
    Ok(EntityDefn::new(
        db,
        entity_route.ident(),
        EntityDefnVariant::Module {
            module_items: module_items(db, entity_route).unwrap(),
            opt_main_defn,
        },
        entity_route,
        file,
        Default::default(),
    ))
}

pub fn module_items(
    db: &dyn EntityDefnQueryGroup,
    entity_route: EntityRoutePtr,
) -> SemanticResult<Avec<EntityDefn>> {
    let subroute_table = db.subroute_table(entity_route).unwrap();
    Ok(Arc::new(
        subroute_table
            .subroute_iter(db.upcast(), entity_route)
            .map(|subroute| db.entity_defn(subroute))
            .collect::<SemanticResult<Vec<_>>>()?,
    ))
}
