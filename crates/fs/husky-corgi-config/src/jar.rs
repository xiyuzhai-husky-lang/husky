use crate::*;

#[salsa::jar]
pub struct CorgiConfigJar(
    package_corgi_config,
    package_corgi_config_paths_aux,
    package_registry_path,
    root_corgi_config_path,
    crate::transpilation_setup::linktime_target_transpilation_setup,
    crate::transpilation_setup::TranspilationSetup,
);
