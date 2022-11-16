use crate::*;

pub fn module_defn(
    db: &dyn EntityDefnQueryGroup,
    entity_path: Ty,
    file: PathItd,
) -> SemanticResultArc<EntityDefn> {
    todo!()
    // let opt_main_defn = if file.ends_with("main.hsy") {
    //     Some(db.main_defn(file)?)
    // } else {
    //     None
    // };
    // Ok(EntityDefn::new(
    //     db,
    //     entity_path.ident(),
    //     EntityDefnVariant::Module {
    //         module_items: module_items(db, entity_path).unwrap(),
    //         opt_main_defn,
    //     },
    //     entity_path,
    //     file,
    //     Default::default(),
    // ))
}

pub fn module_items(
    db: &dyn EntityDefnQueryGroup,
    entity_path: Ty,
) -> SemanticResult<Avec<EntityDefn>> {
    todo!()
    // let subroute_table = db.subroute_table(entity_path).unwrap();
    // Ok(Arc::new(
    //     subroute_table
    //         .subroute_iter(db.upcast(), entity_path)
    //         .map(|subroute| db.entity_defn(subroute))
    //         .collect::<SemanticResult<Vec<_>>>()?,
    // ))
}
