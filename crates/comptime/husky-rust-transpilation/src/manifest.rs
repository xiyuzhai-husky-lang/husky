use crate::{
    package::{rust_transpilation_packages, RustTranspilationPackageKind},
    *,
};
use cargo_manifest::{
    Dependency, DependencyDetail, Edition, InheritedDependencyDetail, Manifest, MaybeInherited,
    Package, Product, Resolver, True, Workspace,
};
use husky_corgi_config::transpilation_setup::{HasTranspilationSetup, TranspilationSetup};
use husky_manifest::HasPackageManifest;
use husky_vfs::linktime_target_path::LinktimeTargetPath;
use pathdiff::diff_paths;

#[derive(Debug, PartialEq)]
pub(crate) struct RustManifest(Manifest);

impl Eq for RustManifest {}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn linktime_target_rust_workspace_manifest(
    db: &::salsa::Db,
    linktime_target_path: LinktimeTargetPath,
) -> String {
    let rust_transpilation_packages = rust_transpilation_packages(db, linktime_target_path);
    let rust_workspace_abs_dir = linktime_target_path.rust_workspace_abs_dir(db);
    let members = rust_transpilation_packages
        .iter()
        .filter_map(|package| {
            (!(package.package_path.is_virtual(db)
                && package.kind == RustTranspilationPackageKind::Source))
                .then(|| package.path_in_workspace(&rust_workspace_abs_dir, db))
        })
        .collect();
    let toolchain = linktime_target_path.toolchain(db);
    let library_abs_path = toolchain.library_abs_path(db);
    let library_diffpath = diff_paths(&library_abs_path, &rust_workspace_abs_dir).unwrap();
    let rust_transpilation_setup_data = linktime_target_path
        .transpilation_setup(db)
        .rust_data(db)
        .unwrap();
    let task_dependency_abs_path = rust_transpilation_setup_data
        .task_dependency_path
        .abs_path(db)
        .unwrap();
    let task_dependency_diffpath =
        diff_paths(&task_dependency_abs_path, &rust_workspace_abs_dir).unwrap();
    let dependencies = [
        (
            "husky-core".to_string(),
            Dependency::Detailed(DependencyDetail {
                path: Some(
                    library_diffpath
                        .join("core")
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .to_string(),
                ),
                ..Default::default()
            }),
        ),
        (
            rust_transpilation_setup_data
                .task_dependency_name
                .data(db)
                .to_string(),
            Dependency::Detailed(DependencyDetail {
                path: Some(
                    task_dependency_diffpath
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .to_string(),
                ),
                ..Default::default()
            }),
        ),
    ]
    .into_iter()
    .chain(rust_transpilation_packages.iter().map(|package| {
        (
            package.name(db),
            Dependency::Detailed(DependencyDetail {
                version: None,
                registry: None,
                registry_index: None,
                path: Some(package.path_in_workspace(&rust_workspace_abs_dir, db)),
                git: None,
                branch: None,
                tag: None,
                rev: None,
                features: None,
                optional: None,
                default_features: None,
                package: None,
            }),
        )
    }))
    .collect();
    toml::to_string(&Manifest::<toml::Value> {
        package: None,
        cargo_features: None,
        workspace: Some(Workspace {
            members,
            resolver: Some(Resolver::V2),
            dependencies: Some(dependencies),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap()
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_source_rust_package_manifest(
    db: &::salsa::Db,
    package_path: PackagePath,
    transpilation_setup: TranspilationSetup,
) -> String {
    let rust_transpilation_setup_data = transpilation_setup.rust_data(db).unwrap();
    let dependencies = [
        "husky-core".to_string(),
        rust_transpilation_setup_data
            .task_dependency_name
            .data(db)
            .to_string(),
    ]
    .into_iter()
    .chain(
        package_path
            .dependencies(db)
            .unwrap()
            .iter()
            .map(|dep| dep.package_path().name_string(db)),
    )
    .map(|name| (name, INHERITED))
    .collect();
    toml::to_string(&Manifest {
        package: Some(Package::<toml::Value> {
            name: package_path.name(db).data(db).to_owned(),
            edition: Some(MaybeInherited::Local(Edition::E2021)),
            version: MaybeInherited::Local("0.1.0".to_string()),
            build: None,
            workspace: None,
            authors: None,
            links: None,
            description: None,
            homepage: None,
            documentation: None,
            readme: None,
            keywords: None,
            categories: None,
            license: None,
            license_file: None,
            repository: None,
            metadata: None,
            rust_version: None,
            exclude: None,
            include: None,
            default_run: None,
            autobins: true,
            autoexamples: true,
            autotests: true,
            autobenches: true,
            publish: None,
            resolver: None,
        }),
        dependencies: Some(dependencies),
        ..Default::default()
    })
    .unwrap()
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_linkages_rust_package_manifest(
    db: &::salsa::Db,
    package_path: PackagePath,
    transpilation_setup: TranspilationSetup,
) -> String {
    let rust_transpilation_setup_data = transpilation_setup.rust_data(db).unwrap();
    let dependencies = [
        "husky-core".to_string(),
        rust_transpilation_setup_data
            .task_dependency_name
            .data(db)
            .to_string(),
    ]
    .into_iter()
    .chain(
        package_path
            .dependencies(db)
            .unwrap()
            .iter()
            .map(|dep| dep.package_path().name_string(db)),
    )
    .chain([package_path.name_string(db)])
    .map(|name| (name, INHERITED))
    .collect();
    toml::to_string(&Manifest {
        package: Some(Package::<toml::Value> {
            name: format!("{}-linkages", package_path.name(db).data(db)),
            edition: Some(MaybeInherited::Local(Edition::E2021)),
            version: MaybeInherited::Local("0.1.0".to_string()),
            build: None,
            workspace: None,
            authors: None,
            links: None,
            description: None,
            homepage: None,
            documentation: None,
            readme: None,
            keywords: None,
            categories: None,
            license: None,
            license_file: None,
            repository: None,
            metadata: None,
            rust_version: None,
            exclude: None,
            include: None,
            default_run: None,
            autobins: true,
            autoexamples: true,
            autotests: true,
            autobenches: true,
            publish: None,
            resolver: None,
        }),
        dependencies: Some(dependencies),
        lib: Some(Product {
            path: None,
            name: None,
            test: true,
            doctest: true,
            bench: true,
            doc: true,
            plugin: false,
            proc_macro: false,
            harness: true,
            edition: None,
            required_features: vec![],
            crate_type: Some(vec!["cdylib".into()]),
        }),
        ..Default::default()
    })
    .unwrap()
}

const INHERITED: Dependency = Dependency::Inherited(InheritedDependencyDetail {
    workspace: True,
    features: None,
    optional: None,
});
