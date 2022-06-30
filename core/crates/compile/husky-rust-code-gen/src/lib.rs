mod bin_main_rs_content;
mod cargo_toml_content;
mod generator;
mod lib_rs_content;
mod mod_rs_content;

use bin_main_rs_content::*;
use cargo_toml_content::*;
use defn_head::*;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use liason::*;
use lib_rs_content::*;
use mod_rs_content::*;
use pack_semantics::PackageQueryGroup;
use print_utils::*;
use std::sync::Arc;

#[salsa::query_group(RustGenQueryStorage)]
pub trait RustCodeGenQueryGroup: PackageQueryGroup {
    fn cargo_toml_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_lib_rs_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_bin_main_rs_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_mod_rs_content(&self, module: EntityRoutePtr) -> Arc<String>;
}
