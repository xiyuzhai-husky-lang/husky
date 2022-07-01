mod bin_main_rs_content;
mod cargo_toml_content;
mod contains_eval_ref;
mod generator;
mod lib_rs_content;
mod mod_rs_content;

use bin_main_rs_content::*;
use cargo_toml_content::*;
use contains_eval_ref::*;
use defn_head::*;
use file::FilePtr;
use husky_entity_route_syntax::{EntityRouteKind, EntityRoutePtr};
use husky_liason_semantics::*;
use husky_package_semantics::PackageQueryGroup;
use lib_rs_content::*;
use mod_rs_content::*;
use print_utils::*;
use std::sync::Arc;

#[salsa::query_group(RustGenQueryStorage)]
pub trait RustCodeGenQueryGroup: PackageQueryGroup {
    fn cargo_toml_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_lib_rs_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_bin_main_rs_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_mod_rs_content(&self, module: EntityRoutePtr) -> Arc<String>;
    fn entity_route_kind_contains_eval_ref(&self, entity_route_kind: EntityRouteKind) -> bool;
    fn entity_route_contains_eval_ref(&self, entity_route: EntityRoutePtr) -> bool;
}
