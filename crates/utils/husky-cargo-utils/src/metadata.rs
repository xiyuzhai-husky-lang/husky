use cargo::{
    core::{resolver::CliFeatures, Workspace},
    ops::{ExportInfo, OutputMetadataOptions},
};

pub fn output_metadata<R>(manifest_path: &std::path::Path, f: impl FnOnce(ExportInfo) -> R) -> R {
    let config = cargo::Config::default().expect("what the hell");
    let workspace = Workspace::new(manifest_path, &config).expect("what the hell");
    let cli_features = CliFeatures::new_all(true);
    let opt = &OutputMetadataOptions {
        cli_features,
        no_deps: false,
        version: 1,
        filter_platforms: vec![],
    };
    f(cargo::ops::output_metadata(&workspace, opt).unwrap())
}
