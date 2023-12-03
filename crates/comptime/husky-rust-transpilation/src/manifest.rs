use cargo_manifest::{
    Dependency, DependencyDetail, Edition, Manifest, MaybeInherited, Package, Product, Resolver,
    Workspace,
};
use husky_vfs::linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData};

use crate::{
    package::{rust_transpilation_packages, RustTranspilationPackageKind},
    *,
};

#[derive(Debug, PartialEq)]
pub(crate) struct RustManifest(Manifest);

impl Eq for RustManifest {}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn linktime_target_rust_workspace_manifest(
    db: &::salsa::Db,
    linktime_target_path: LinktimeTargetPath,
) -> String {
    let rust_transpilation_packages = rust_transpilation_packages(db, linktime_target_path);
    let members = rust_transpilation_packages
        .iter()
        .map(|package| package.relative_path_in_workspace(db))
        .collect();
    let dependencies = rust_transpilation_packages
        .iter()
        .map(|package| {
            (
                package.name(db),
                Dependency::Detailed(DependencyDetail {
                    version: None,
                    registry: None,
                    registry_index: None,
                    path: Some(package.relative_path_in_workspace(db)),
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
        })
        .collect();
    toml::to_string(&Manifest::<toml::Value> {
        package: None,
        cargo_features: None,
        workspace: Some(Workspace {
            members,
            default_members: None,
            exclude: None,
            resolver: Some(Resolver::V2),
            dependencies: Some(dependencies),
            package: None,
        }),
        dependencies: None,
        dev_dependencies: None,
        build_dependencies: None,
        target: None,
        features: None,
        bin: None,
        bench: None,
        test: None,
        example: None,
        patch: None,
        lib: None,
        profile: None,
        badges: None,
    })
    .unwrap()
}

// #[deprecated(note = "ad hoc")]
// #[salsa::tracked(jar = RustTranspilationJar, return_ref)]
// pub(crate) fn linktime_target_rust_workspace_members(
//     db: &::salsa::Db,
//     linktime_target_path: LinktimeTargetPath,
// ) -> Vec<String> {
//     match linktime_target_path.data(db) {
//         LinktimeTargetPathData::Package(package_path) => {
//             vec![package_path.name(db).data(db).to_string()]
//         }
//         LinktimeTargetPathData::Workspace(_) => todo!(),
//     }
// }

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_source_rust_package_manifest(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> String {
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
        cargo_features: None,
        workspace: None,
        dependencies: None,
        dev_dependencies: None,
        build_dependencies: None,
        target: None,
        features: None,
        bin: None,
        bench: None,
        test: None,
        example: None,
        patch: None,
        lib: None,
        profile: None,
        badges: None,
    })
    .unwrap()
}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_linkages_rust_package_manifest(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> String {
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
        cargo_features: None,
        workspace: None,
        dependencies: None,
        dev_dependencies: None,
        build_dependencies: None,
        target: None,
        features: None,
        bin: None,
        bench: None,
        test: None,
        example: None,
        patch: None,
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
        profile: None,
        badges: None,
    })
    .unwrap()
}
