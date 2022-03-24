mod bin_main_rs_content;
mod cargo_toml_content;
mod generator;
mod lib_rs_content;

use bin_main_rs_content::*;
use cargo_toml_content::*;
use file::FilePtr;
use lib_rs_content::*;
use print_utils::*;
use semantics_package::PackageQueryGroup;
use std::sync::Arc;

#[salsa::query_group(RustGenQueryStorage)]
pub trait RustGenQueryGroup: PackageQueryGroup {
    fn cargo_toml_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_lib_rs_content(&self, main_file: FilePtr) -> Arc<String>;
    fn rust_bin_main_rs_content(&self, main_file: FilePtr) -> Arc<String>;
}
