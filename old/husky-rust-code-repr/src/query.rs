// #[salsa::query_group(RustGenQueryStorage)]
// pub trait RustCodeReprQueryGroup: PackageQueryGroup {
//     fn rust_lib_rs_content(&self, target_entrance: FilePtr) -> Arc<String>;
//     fn rust_init_rs_content(&self, target_entrance: FilePtr) -> Arc<String>;
//     fn rust_mod_rs_content(&self, module: EntityRoutePtr) -> Arc<String>;
//     fn item_route_kind_contains_leash(&self, item_route_kind: EntityRouteKind) -> bool;
//     fn item_route_contains_leash(&self, item_path: EntityRoutePtr) -> bool;
//     fn is_defn_static(&self, item_path: EntityRoutePtr) -> bool;
//     fn contains_spatial_parameters(&self, item_path: EntityRoutePtr) -> bool;
//     fn item_immediate_link_dependees(
//         &self,
//         item_path: EntityRoutePtr,
//     ) -> Arc<VecSet<EntityRoutePtr>>;
//     fn item_link_dependees(&self, item_path: EntityRoutePtr) -> Arc<VecSet<EntityRoutePtr>>;
//     fn needs_eval_context(&self, item_path: EntityRoutePtr) -> bool;
// }
