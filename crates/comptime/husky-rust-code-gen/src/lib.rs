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
use husky_ethereal_term::EtherealTerm;
use husky_layout::LayoutDb;
use husky_vfs::*;
use salsa::DbWithJar;
use std::sync::Arc;
use vec_like::VecSet;

#[salsa::jar(db = RustTranspileDb)]
pub struct RustTranspileJar();

pub trait RustTranspileDb: DbWithJar<RustTranspileJar> + LayoutDb {
    fn rust_lib_rs_content(&self, target_entrance: DiffPath) -> Arc<String>;
    fn rust_registration_rs_content(&self, target_entrance: DiffPath) -> Arc<String>;
    fn rust_init_rs_content(&self, target_entrance: DiffPath) -> Arc<String>;
    fn rust_mod_rs_content(&self, module: EtherealTerm) -> Arc<String>;
    fn entity_route_variant_contains_eval_ref(&self, entity_path: EtherealTerm) -> bool;
    fn entity_route_contains_eval_ref(&self, entity_path: EtherealTerm) -> bool;
    fn is_defn_static(&self, entity_path: EtherealTerm) -> bool;
    fn contains_spatial_parameters(&self, entity_path: EtherealTerm) -> bool;
    fn entity_immediate_link_dependees(
        &self,
        entity_path: EtherealTerm,
    ) -> Arc<VecSet<EtherealTerm>>;
    fn entity_link_dependees(&self, entity_path: EtherealTerm) -> Arc<VecSet<EtherealTerm>>;
    fn needs_eval_context(&self, entity_path: EtherealTerm) -> bool;
    fn mangled_intrinsic_ty(&self, entity_path: EtherealTerm) -> Arc<String>;
    fn mangled_intrinsic_ty_vtable(&self, entity_path: EtherealTerm) -> Arc<String>;
    fn mangled_ty(&self, entity_path: EtherealTerm) -> Arc<String>;
    fn mangled_ty_vtable(&self, entity_path: EtherealTerm) -> Arc<String>;
}

impl<T> RustTranspileDb for T
where
    T: DbWithJar<RustTranspileJar> + LayoutDb,
{
    fn rust_lib_rs_content(&self, _target_entrance: DiffPath) -> Arc<String> {
        todo!()
    }

    fn rust_registration_rs_content(&self, _target_entrance: DiffPath) -> Arc<String> {
        todo!()
    }

    fn rust_init_rs_content(&self, _target_entrance: DiffPath) -> Arc<String> {
        todo!()
    }

    fn rust_mod_rs_content(&self, _module: EtherealTerm) -> Arc<String> {
        todo!()
    }

    fn entity_route_variant_contains_eval_ref(&self, _entity_path: EtherealTerm) -> bool {
        todo!()
    }

    fn entity_route_contains_eval_ref(&self, _entity_path: EtherealTerm) -> bool {
        todo!()
    }

    fn is_defn_static(&self, _entity_path: EtherealTerm) -> bool {
        todo!()
    }

    fn contains_spatial_parameters(&self, _entity_path: EtherealTerm) -> bool {
        todo!()
    }

    fn entity_immediate_link_dependees(
        &self,
        _entity_path: EtherealTerm,
    ) -> Arc<VecSet<EtherealTerm>> {
        todo!()
    }

    fn entity_link_dependees(&self, _entity_path: EtherealTerm) -> Arc<VecSet<EtherealTerm>> {
        todo!()
    }

    fn needs_eval_context(&self, _entity_path: EtherealTerm) -> bool {
        todo!()
    }

    fn mangled_intrinsic_ty(&self, _entity_path: EtherealTerm) -> Arc<String> {
        todo!()
    }

    fn mangled_intrinsic_ty_vtable(&self, _entity_path: EtherealTerm) -> Arc<String> {
        todo!()
    }

    fn mangled_ty(&self, _entity_path: EtherealTerm) -> Arc<String> {
        todo!()
    }

    fn mangled_ty_vtable(&self, _entity_path: EtherealTerm) -> Arc<String> {
        todo!()
    }
}
