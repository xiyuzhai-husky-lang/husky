mod cargo_toml_content;
mod code_generator;
mod contains_eval_ref;
mod eval_context;
mod init_content;
mod lib_rs_content;
mod linkage_collector;
mod mangle;
mod mod_rs_content;
mod registration_content;
mod utils;

pub use cargo_toml_content::*;
use husky_layout::HuskyLayoutQueryGroup;

use crate::registration_content::rust_registration_rs_content;
use contains_eval_ref::*;
use eval_context::*;
use husky_defn_head::*;
use husky_entity_route::{EntityRouteItd, EntityRouteVariant};
use husky_file::FileItd;
use husky_init_syntax::*;
use husky_liason_semantics::*;
use husky_loop_syntax::*;
use husky_opn_syntax::*;
use husky_package_semantics::PackageQueryGroup;
use husky_print_utils::*;
use husky_rust_code_repr::entity_route::*;
use init_content::*;
use lib_rs_content::*;
use linkage_collector::*;
use mangle::*;
use mod_rs_content::*;
use std::sync::Arc;
use utils::*;
use vec_like::VecSet;

#[salsa::query_group(RustGenQueryStorage)]
pub trait RustCodeGenQueryGroup: PackageQueryGroup + HuskyLayoutQueryGroup {
    fn rust_lib_rs_content(&self, target_entrance: FileItd) -> Arc<String>;
    fn rust_registration_rs_content(&self, target_entrance: FileItd) -> Arc<String>;
    fn rust_init_rs_content(&self, target_entrance: FileItd) -> Arc<String>;
    fn rust_mod_rs_content(&self, module: EntityRouteItd) -> Arc<String>;
    fn entity_route_variant_contains_eval_ref(&self, entity_route: EntityRouteItd) -> bool;
    fn entity_route_contains_eval_ref(&self, entity_route: EntityRouteItd) -> bool;
    fn is_defn_static(&self, entity_route: EntityRouteItd) -> bool;
    fn contains_spatial_parameters(&self, entity_route: EntityRouteItd) -> bool;
    fn entity_immediate_link_dependees(
        &self,
        entity_route: EntityRouteItd,
    ) -> Arc<VecSet<EntityRouteItd>>;
    fn entity_link_dependees(&self, entity_route: EntityRouteItd) -> Arc<VecSet<EntityRouteItd>>;
    fn needs_eval_context(&self, entity_route: EntityRouteItd) -> bool;
    fn mangled_intrinsic_ty(&self, entity_route: EntityRouteItd) -> Arc<String>;
    fn mangled_intrinsic_ty_vtable(&self, entity_route: EntityRouteItd) -> Arc<String>;
    fn mangled_ty(&self, entity_route: EntityRouteItd) -> Arc<String>;
    fn mangled_ty_vtable(&self, entity_route: EntityRouteItd) -> Arc<String>;
}
