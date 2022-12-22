// #![feature(try_trait_v2)]

// mod comptime;
// mod hot_reload;
// mod impl_necessary;
// mod impl_train;
// mod query;
// mod variant;

// pub use hot_reload::{HuskyRuntimeHotReloadM, HuskyRuntimeHotReloadR};
// pub use husky_comptime::*;
// pub use husky_feature_gen::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage, InternFeature};
// pub use husky_instruction_gen::InstructionDb;
// pub use query::*;

// use husky_check_utils::*;
// use husky_compiler::CompilerInstance;
// use husky_diagnostics::DiagnosticsDb;
// use husky_entity_semantics::EntityRouteStore;
// use husky_feature_eval::*;
// use husky_feature_eval::{EvalFeature, Session};
// use husky_feature_gen::FeatureInterner;
// use husky_linkage_table::LinkageTable;
// use husky_path::{FileQueryGroup, DiffPath};
// use husky_print_utils::*;

// use indexmap::IndexMap;

// use relative_path::RelativePathBuf;
// use std::sync::Arc;
// use sync_utils::ASafeRwLock;
// use variant::*;

// #[salsa::database(
//     husky_feature_gen::FeatureGenQueryGroupStorage,
//     husky_instruction_gen::InstructionDbStorage,
//     husky_data_viewer::DataViewerDbStorage,
//     // comptime
//     husky_path::FileQueryStorage,
//     husky_token_sheet::TokenQueryGroupStorage,
//     husky_entity_symbol::ScopeQueryGroupStorage,
//     husky_text::TextQueryGroupStorage,
//     husky_ast::AstQueryGroupStorage,
//     husky_syntax_fmt::FormatQueryGroupStorage,
//     husky_entity_semantics::EntityQueryGroupStorage,
//     husky_package_semantics::PackageQueryGroupStorage,
//     husky_diagnostics::DiagnosticsDbGroupStorage,
//     husky_rust_code_gen::RustGenQueryStorage,
//     husky_layout::HuskyLayoutQueryGroupStorage
// )]
// pub struct DevRuntime {
//     storage: salsa::Storage<DevRuntime>,
//     variant: HuskyRuntimeVariant,
//     config: RuntimeConfig,
//     live_docs: ASafeRwLock<IndexMap<DiffPath, ASafeRwLock<String>>>,
//     linkage_table: LinkageTable,
//     entity_route_store: EntityRouteStore,
// }

// #[derive(Debug)]
// pub struct RuntimeConfig {
//     pub evaluator: EvaluatorConfig,
//     pub comptime: ComptimeConfig,
// }

// impl DevRuntime {
//     pub fn new(config: RuntimeConfig) -> DevRuntime {
//         todo!()
//         // let mut runtime = Self {
//         //     storage: Default::default(),
//         //     variant: HuskyRuntimeVariant::None,
//         //     feature_interner: Default::default(),
//         //     // comptime
//         //     file_interner: Default::default(),
//         //     word_interner: Default::default(),
//         //     live_docs: Default::default(),
//         //     linkage_table: LinkageTable::new(config.comptime.linkage_table.clone()),
//         //     entity_route_store: Default::default(),
//         //     entity_route_interner: Default::default(),
//         //     // config
//         //     config,
//         // };
//         // runtime.init();
//         // runtime
//     }

//     fn init(&mut self) {
//         self.hot_reload();
//         let all_main_files = self.all_target_entrances();
//         should_eq!(all_main_files.len(), 1, "config = {:?}", self.config);
//         let all_diagnostics = self.all_diagnostics();
//         if all_diagnostics.len() > 0 {
//             p!(all_diagnostics);
//             panic!("diagnostic errors")
//         }
//         let package = match self.package(self.opt_target_entrance().unwrap()) {
//             Ok(package) => package,
//             Err(error) => {
//                 self.print_diagnostics();
//                 p!(error);
//                 panic!()
//             }
//         };
//         self.variant = HuskyRuntimeVariant::Learning {
//             session: Session::new(&package, self, &self.evaluator_config().vm).unwrap(),
//         }
//     }
// }
