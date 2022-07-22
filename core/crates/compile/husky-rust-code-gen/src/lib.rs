mod bin_main_rs_content;
mod cargo_toml_content;
mod code_generator;
mod contains_eval_ref;
mod eval_context;
mod init_content;
mod lib_rs_content;
mod linkage_collector;
mod mod_rs_content;
mod utils;

pub use cargo_toml_content::*;

use bin_main_rs_content::*;
use contains_eval_ref::*;
use defn_head::*;
use eval_context::*;
use husky_entity_route::{EntityRouteKind, EntityRoutePtr};
use husky_file::FilePtr;
use husky_liason_semantics::*;
use husky_package_semantics::PackageQueryGroup;
use init_content::*;
use lib_rs_content::*;
use linkage_collector::*;
use mod_rs_content::*;
use print_utils::*;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};
use utils::*;
use vec_like::VecSet;

#[salsa::query_group(RustGenQueryStorage)]
pub trait RustCodeGenQueryGroup: PackageQueryGroup {
    fn rust_lib_rs_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_init_rs_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_mod_rs_content(&self, module: EntityRoutePtr) -> Arc<String>;
    fn rust_bin_main_rs_content(&self, main_file: FilePtr, rel_crate_dir: PathBuf) -> Arc<String>;
    fn entity_route_kind_contains_eval_ref(&self, entity_route_kind: EntityRouteKind) -> bool;
    fn entity_route_contains_eval_ref(&self, entity_route: EntityRoutePtr) -> bool;
    fn is_defn_static(&self, entity_route: EntityRoutePtr) -> bool;
    fn contains_spatial_parameters(&self, entity_route: EntityRoutePtr) -> bool;
    fn entity_immediate_link_dependees(
        &self,
        entity_route: EntityRoutePtr,
    ) -> Arc<VecSet<EntityRoutePtr>>;
    fn entity_link_dependees(&self, entity_route: EntityRoutePtr) -> Arc<VecSet<EntityRoutePtr>>;
    fn needs_eval_context(&self, entity_route: EntityRoutePtr) -> bool;
}
