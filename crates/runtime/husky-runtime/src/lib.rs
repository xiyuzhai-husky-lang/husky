mod comptime;
mod impl_necessary;
mod impl_train;
mod query;
mod variant;

pub use husky_comptime::*;
use husky_diagnostics::DiagnosticQuery;
use husky_entity_semantics::EntityRouteStore;
use husky_feature_gen::FeatureInterner;
pub use husky_feature_gen::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage, InternFeature};
pub use husky_instruction_gen::InstructionGenQueryGroup;
use husky_linkage_table::LinkageTable;
use husky_vm::{__Linkage, __StaticLinkageKey};
use indexmap::IndexMap;
pub use query::*;

use convert_case::{Boundary, Case, Casing};
use husky_check_utils::*;
use husky_feature_eval::*;
use husky_feature_eval::{EvalFeature, Session};
use husky_file::{FilePtr, FileQueryGroup};
use husky_print_utils::*;
use libloading::Library;
use std::{path::Path, sync::Arc};
use sync_utils::ASafeRwLock;
use variant::*;

#[salsa::database(
    husky_feature_gen::FeatureGenQueryGroupStorage,
    husky_instruction_gen::InstructionGenQueryGroupStorage,
    husky_data_viewer::HuskyDataViewerQueryGroupStorage,
    // comptime
    husky_file::FileQueryStorage,
    husky_token::TokenQueryGroupStorage,
    husky_entity_syntax::ScopeQueryGroupStorage,
    husky_text::TextQueryGroupStorage,
    husky_ast::AstQueryGroupStorage,
    husky_fmt::FormatQueryGroupStorage,
    infer_decl::DeclQueryGroupStorage,
    husky_infer_entity_route::InferEntityRouteQueryGroupStorage,
    infer_contract::InferContractQueryGroupStorage,
    husky_infer_qualified_ty::InferQualifiedTyQueryGroupStorage,
    husky_entity_semantics::EntityQueryGroupStorage,
    husky_package_semantics::PackageQueryGroupStorage,
    husky_diagnostics::DiagnosticSalsaQueryGroupStorage,
    husky_rust_code_gen::RustGenQueryStorage,
    husky_layout::HuskyLayoutQueryGroupStorage
)]
pub struct Runtime {
    storage: salsa::Storage<Runtime>,
    feature_interner: FeatureInterner,
    variant: HuskyRuntimeVariant,
    config: RuntimeConfig,
    // comptime
    file_interner: Arc<husky_file::FileInterner>,
    word_interner: Arc<husky_word::WordInterner>,
    entity_route_interner: Arc<husky_entity_route::EntityRouteInterner>,
    live_docs: ASafeRwLock<IndexMap<FilePtr, ASafeRwLock<String>>>,
    linkage_table: LinkageTable,
    entity_route_store: EntityRouteStore,
}

#[derive(Debug)]
pub struct RuntimeConfig {
    pub evaluator: EvaluatorConfig,
    pub comptime: ComptimeConfig,
}

type GetLinkagesFromCDylib = unsafe extern "C" fn() -> &'static [(__StaticLinkageKey, __Linkage)];

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Runtime {
        let feature_interner = husky_feature_gen::new_feature_interner();
        let opt_library = get_library(&config.comptime.package_dir);
        let linkages_from_cdylib: &[(__StaticLinkageKey, __Linkage)] = opt_library
            .as_ref()
            .map(|library| unsafe {
                library
                    .get::<GetLinkagesFromCDylib>(b"get_linkages")
                    .expect("what")()
            })
            .unwrap_or(&[]);
        let mut runtime = Self {
            storage: Default::default(),
            variant: HuskyRuntimeVariant::None,
            feature_interner,
            // comptime
            file_interner: Arc::new(husky_file::new_file_interner()),
            word_interner: Arc::new(husky_word::new_word_interner()),
            live_docs: Default::default(),
            linkage_table: LinkageTable::new(config.comptime.linkage_table.clone()),
            entity_route_store: Default::default(),
            entity_route_interner: Arc::new(husky_entity_route::new_entity_route_interner()),
            // config
            config,
        };
        todo!();
        // init_runtime(&mut runtime);
        let all_main_files = runtime.all_target_entrances();
        should_eq!(all_main_files.len(), 1, "config = {:?}", runtime.config);
        runtime.init();
        runtime.into()
    }

    fn init(&mut self) {
        let all_diagnostics = self.all_diagnostics();
        if all_diagnostics.len() > 0 {
            p!(all_diagnostics);
            panic!("diagnostic errors")
        }
        let package = match self.package(self.opt_target_entrance().unwrap()) {
            Ok(package) => package,
            Err(error) => {
                self.print_diagnostics();
                p!(error);
                panic!()
            }
        };
        self.variant = HuskyRuntimeVariant::Learning {
            session: Session::new(&package, self, &self.evaluator_config().vm).unwrap(),
        }
    }
}

fn get_library(package_dir: &Path) -> Option<Library> {
    #[cfg(target_os = "linux")]
    static DYLIB_EXTENSION: &'static str = "so";
    #[cfg(target_os = "macos")]
    static DYLIB_EXTENSION: &'static str = "dylib";
    let package_name = package_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .with_boundaries(&[Boundary::Hyphen])
        .to_case(Case::Snake);
    let library_release_path = package_dir.join(format!(
        "__rust_gen__/target/release/lib{}.{DYLIB_EXTENSION}",
        package_name
    ));
    if library_release_path.exists() {
        return Some(unsafe { Library::new(library_release_path) }.expect("it should work"));
    }
    let library_debug_path = package_dir.join(format!(
        "__rust_gen__/target/debug/lib{}.{DYLIB_EXTENSION}",
        package_name,
    ));
    if library_debug_path.exists() {
        todo!()
    }
    None
}
