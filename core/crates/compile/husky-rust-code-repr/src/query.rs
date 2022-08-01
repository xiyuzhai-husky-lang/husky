// #[salsa::query_group(RustGenQueryStorage)]
// pub trait RustCodeReprQueryGroup: PackageQueryGroup {
//     fn rust_lib_rs_content(&self, main_file: FilePtr) -> Arc<String>;
//     fn rust_init_rs_content(&self, main_file: FilePtr) -> Arc<String>;
//     fn rust_mod_rs_content(&self, module: EntityRoutePtr) -> Arc<String>;
//     fn entity_route_kind_contains_eval_ref(&self, entity_route_kind: EntityRouteKind) -> bool;
//     fn entity_route_contains_eval_ref(&self, entity_route: EntityRoutePtr) -> bool;
//     fn is_defn_static(&self, entity_route: EntityRoutePtr) -> bool;
//     fn contains_spatial_parameters(&self, entity_route: EntityRoutePtr) -> bool;
//     fn entity_immediate_link_dependees(
//         &self,
//         entity_route: EntityRoutePtr,
//     ) -> Arc<VecSet<EntityRoutePtr>>;
//     fn entity_link_dependees(&self, entity_route: EntityRoutePtr) -> Arc<VecSet<EntityRoutePtr>>;
//     fn needs_eval_context(&self, entity_route: EntityRoutePtr) -> bool;
// }
