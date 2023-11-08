use std::panic::RefUnwindSafe;

#[derive(Default)]
#[salsa::db(
    // comptime
    husky_val_repr::db::ValReprJar,
    // devtime
    husky_trace::db::TraceJar,
    // fs
    husky_vfs::VfsJar,
    // hir
    husky_hir_eager_expr::db::HirEagerExprJar,
    husky_hir_lazy_expr::db::HirLazyExprJar,
    husky_hir_expr::db::HirExprJar,
    husky_hir_deps::db::HirDepsJar,
    husky_hir_decl::db::HirDeclJar,
    husky_hir_defn::db::HirDefnJar,
    // ide
    husky_token_info::db::TokenInfoJar,
    // kernel
    husky_coword::CowordJar,
    husky_entity_path::EntityPathJar,
    husky_term_prelude::TermPreludeJar,
    husky_declarative_term::DeclarativeTermJar,
    husky_declarative_ty::DeclarativeTypeJar,
    husky_declarative_signature::DeclarativeSignatureJar,
    husky_ethereal_term::EtherealTermJar,
    husky_ethereal_signature::EtherealSignatureJar,
    husky_fluffy_term::FluffyTermJar,
    husky_hir_ty::db::HirTypeJar,
    // lex
    husky_token_data::db::TokenDataJar,
    husky_token::TokenJar,
    husky_toml_token::TomlTokenJar,
    husky_text::db::TextJar,
    // linkage
    husky_linkage_path::db::LinkagePathJar,
    // semantics
    husky_sema_expr::SemaExprJar,
    husky_corgi_config::CorgiConfigJar,
    husky_manifest::ManifestJar,
    // syntax
    husky_ast::AstJar,
    husky_toml_ast::TomlAstJar,
    husky_corgi_config_ast::CorgiConfigAstJar,
    husky_manifest_ast::ManifestAstJar,
    husky_entity_syn_tree::EntitySynTreeJar,
    husky_syn_expr::SynExprJar,
    husky_syn_decl::SynDeclJar,
    husky_syn_defn::SynDefnJar,
    // val
    husky_val::db::ValJar
)]
pub struct StandardDevComptimeDb {
    storage: salsa::Storage<Self>,
}

impl husky_vm::InterpreterQueryGroup for StandardDevComptimeDb {
    fn item_opt_instruction_sheet_by_uid(
        &self,
        uid: husky_vm::EntityUid,
    ) -> Option<husky_vm::Instructions> {
        todo!()
    }
}

impl salsa::Database for StandardDevComptimeDb {}

// ad hoc: is this correct?
impl RefUnwindSafe for StandardDevComptimeDb {}
