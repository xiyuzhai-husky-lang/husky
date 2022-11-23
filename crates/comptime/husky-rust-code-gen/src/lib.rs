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
use husky_layout::LayoutDb;
use husky_source_path::SourcePath;
use salsa::DbWithJar;

use crate::registration_content::rust_registration_rs_content;
use contains_eval_ref::*;
use eval_context::*;
use husky_defn_head::*;
use husky_loop_syntax::*;
use husky_opn_syntax::*;
use husky_print_utils::*;
use husky_rust_code_repr::entity_path::*;
use husky_term::Ty;
use init_content::*;
use lib_rs_content::*;
use linkage_collector::*;
use mangle::*;
use mod_rs_content::*;
use std::sync::Arc;
use utils::*;
use vec_like::VecSet;

#[salsa::jar(db = RustTranspileDb)]
pub struct RustTranspileJar();

pub trait RustTranspileDb: DbWithJar<RustTranspileJar> + LayoutDb {
    fn rust_lib_rs_content(&self, target_entrance: SourcePath) -> Arc<String>;
    fn rust_registration_rs_content(&self, target_entrance: SourcePath) -> Arc<String>;
    fn rust_init_rs_content(&self, target_entrance: SourcePath) -> Arc<String>;
    fn rust_mod_rs_content(&self, module: Ty) -> Arc<String>;
    fn entity_route_variant_contains_eval_ref(&self, entity_path: Ty) -> bool;
    fn entity_route_contains_eval_ref(&self, entity_path: Ty) -> bool;
    fn is_defn_static(&self, entity_path: Ty) -> bool;
    fn contains_spatial_parameters(&self, entity_path: Ty) -> bool;
    fn entity_immediate_link_dependees(&self, entity_path: Ty) -> Arc<VecSet<Ty>>;
    fn entity_link_dependees(&self, entity_path: Ty) -> Arc<VecSet<Ty>>;
    fn needs_eval_context(&self, entity_path: Ty) -> bool;
    fn mangled_intrinsic_ty(&self, entity_path: Ty) -> Arc<String>;
    fn mangled_intrinsic_ty_vtable(&self, entity_path: Ty) -> Arc<String>;
    fn mangled_ty(&self, entity_path: Ty) -> Arc<String>;
    fn mangled_ty_vtable(&self, entity_path: Ty) -> Arc<String>;
}

impl<T> RustTranspileDb for T
where
    T: DbWithJar<RustTranspileJar> + LayoutDb,
{
    fn rust_lib_rs_content(&self, target_entrance: SourcePath) -> Arc<String> {
        todo!()
    }

    fn rust_registration_rs_content(&self, target_entrance: SourcePath) -> Arc<String> {
        todo!()
    }

    fn rust_init_rs_content(&self, target_entrance: SourcePath) -> Arc<String> {
        todo!()
    }

    fn rust_mod_rs_content(&self, module: Ty) -> Arc<String> {
        todo!()
    }

    fn entity_route_variant_contains_eval_ref(&self, entity_path: Ty) -> bool {
        todo!()
    }

    fn entity_route_contains_eval_ref(&self, entity_path: Ty) -> bool {
        todo!()
    }

    fn is_defn_static(&self, entity_path: Ty) -> bool {
        todo!()
    }

    fn contains_spatial_parameters(&self, entity_path: Ty) -> bool {
        todo!()
    }

    fn entity_immediate_link_dependees(&self, entity_path: Ty) -> Arc<VecSet<Ty>> {
        todo!()
    }

    fn entity_link_dependees(&self, entity_path: Ty) -> Arc<VecSet<Ty>> {
        todo!()
    }

    fn needs_eval_context(&self, entity_path: Ty) -> bool {
        todo!()
    }

    fn mangled_intrinsic_ty(&self, entity_path: Ty) -> Arc<String> {
        todo!()
    }

    fn mangled_intrinsic_ty_vtable(&self, entity_path: Ty) -> Arc<String> {
        todo!()
    }

    fn mangled_ty(&self, entity_path: Ty) -> Arc<String> {
        todo!()
    }

    fn mangled_ty_vtable(&self, entity_path: Ty) -> Arc<String> {
        todo!()
    }
}
