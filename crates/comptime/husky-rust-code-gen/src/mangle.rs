use super::*;




pub(crate) fn mangled_intrinsic_ty(_db: &dyn RustTranspileDb, _entity_path: Term) -> Arc<String> {
    todo!()
    // db.mangled_ty(entity_path.intrinsic())
}

pub(crate) fn mangled_ty(_db: &dyn RustTranspileDb, _entity_path: Term) -> Arc<String> {
    todo!()
    // msg_once!("ad hoc");
    // Arc::new(if entity_path.spatial_arguments.len() > 0 {
    //     let mut result = entity_path.ident().as_str().to_string();
    //     w!(result; "_");
    //     for subroute in entity_path.spatial_arguments.iter() {
    //         let subroute = subroute.take_entity_route();
    //         write!(result, "_{}", db.mangled_ty(subroute)).unwrap();
    //     }
    //     result
    // } else {
    //     entity_path.ident().as_str().to_string()
    // })
}

pub(crate) fn mangled_intrinsic_ty_vtable(
    _db: &dyn RustTranspileDb,
    _entity_path: Term,
) -> Arc<String> {
    todo!()
    // db.mangled_ty_vtable(entity_path.intrinsic())
}
pub(crate) fn mangled_ty_vtable(_db: &dyn RustTranspileDb, _entity_path: Term) -> Arc<String> {
    todo!()
    // Arc::new(match entity_path {
    //     Term::Root(_) => {
    //         format!("__{}_VTABLE", entity_path.ident().as_str().to_uppercase())
    //     }
    //     Term::Custom(_) => format!(
    //         "__{}_VTABLE",
    //         &db.mangled_ty(entity_path).to_case(Case::UpperSnake)
    //     ),
    // })
}
