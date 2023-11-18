use cargo_manifest::{Edition, Manifest, MaybeInherited, Package, Workspace};

use crate::*;

#[derive(Debug, PartialEq)]
pub(crate) struct RustManifest(Manifest);

impl Eq for RustManifest {}

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_rust_manifest(
    db: &dyn RustTranspilationDb,
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
        workspace: Some(Workspace {
            members: vec![],
            default_members: None,
            exclude: None,
            resolver: None,
            dependencies: None,
            package: None,
        }),
        // ad hoc
        dependencies: None,
        // ad hoc
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
