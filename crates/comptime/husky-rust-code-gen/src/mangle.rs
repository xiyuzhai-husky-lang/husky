use super::*;

pub(crate) fn mangled_intrinsic_ty(
    _db: &dyn RustTranspileDb,
    _item_path: EtherealTerm,
) -> Arc<String> {
    todo!()
    // db.mangled_ty(item_path.intrinsic())
}

pub(crate) fn mangled_ty(_db: &dyn RustTranspileDb, _item_path: EtherealTerm) -> Arc<String> {
    todo!()
    // msg_once!("ad hoc");
    // Arc::new(if item_path.spatial_arguments.len() > 0 {
    //     let mut result = item_path.ident().as_str().to_string();
    //     w!(result; "_");
    //     for subroute in item_path.spatial_arguments.iter() {
    //         let subroute = subroute.take_item_route();
    //         write!(result, "_{}", db.mangled_ty(subroute)).unwrap();
    //     }
    //     result
    // } else {
    //     item_path.ident().as_str().to_string()
    // })
}

pub(crate) fn mangled_intrinsic_ty_vtable(
    _db: &dyn RustTranspileDb,
    _item_path: EtherealTerm,
) -> Arc<String> {
    todo!()
    // db.mangled_ty_vtable(item_path.intrinsic())
}
pub(crate) fn mangled_ty_vtable(
    _db: &dyn RustTranspileDb,
    _item_path: EtherealTerm,
) -> Arc<String> {
    todo!()
    // Arc::new(match item_path {
    //     EtherealTerm::Root(_) => {
    //         format!("__{}_VTABLE", item_path.ident().as_str().to_uppercase())
    //     }
    //     EtherealTerm::Custom(_) => format!(
    //         "__{}_VTABLE",
    //         &db.mangled_ty(item_path).to_case(Case::UpperSnake)
    //     ),
    // })
}
