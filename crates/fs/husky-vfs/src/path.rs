pub mod crate_path;
pub mod linktime_target_path;
pub mod menu;
pub mod module_path;
pub mod package_path;
pub mod virtual_path;
pub mod workspace_path;

pub use self::crate_path::*;
pub use self::menu::*;
pub use self::module_path::*;
pub use self::package_path::*;
pub use self::virtual_path::*;

use crate::*;
use husky_minimal_toml_utils::read_package_name_from_manifest;

pub(crate) fn package_manifest_path(
    db: &::salsa::Db,
    package: PackagePath,
) -> VfsResult<VirtualPath> {
    VirtualPath::try_new(
        db,
        &package_dir(db, package)
            .as_ref()?
            .data(db)
            .join("Corgi.toml"),
    )
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn module_virtual_path(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> VfsResult<VirtualPath> {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => VirtualPath::try_new(
            db,
            &package_dir(db, crate_path.package_path(db))
                .as_ref()?
                .data(db)
                .join(crate_path.relative_path(db).as_ref()),
        )
        .map_err(|e| e.into()),
        ModulePathData::Child { parent, ident } => {
            let parent_module_path = module_virtual_path(db, parent)?;
            let dir = match parent.data(db) {
                ModulePathData::Root(_) => parent_module_path.data(db).parent().unwrap().to_owned(),
                ModulePathData::Child {
                    parent: _,
                    ident: _,
                } => parent_module_path.data(db).with_extension(""),
            };
            VirtualPath::try_new(db, &dir.join(ident.data(db)).with_extension("hsy"))
        }
    }
}

// this shouldn't be tracked
pub(crate) fn resolve_module_path(
    db: &::salsa::Db,
    toolchain: Toolchain,
    path: impl AsRef<Path>,
) -> VfsResult<ModulePath> {
    let path = path.as_ref();
    if path.extension().and_then(|s| s.to_str()) != Some("hsy") {
        todo!()
    }
    let parent = path.parent().ok_or(VfsError::ModulePathResolveFailure)?;
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(VfsError::ModulePathResolveFailure)?;
    Ok(if parent.ends_with("src") {
        match file_stem {
            "lib" => CratePath::new(
                PackagePath::new_local_or_toolchain_package(
                    db,
                    toolchain,
                    parent.parent().ok_or(VfsError::ModulePathResolveFailure)?,
                )?,
                CrateKind::Lib,
                db,
            )?
            .root_module_path(db),
            "main" => CratePath::new(
                PackagePath::new_local_or_toolchain_package(
                    db,
                    toolchain,
                    parent.parent().ok_or(VfsError::ModulePathResolveFailure)?,
                )?,
                CrateKind::Main,
                db,
            )?
            .root_module_path(db),
            _ => {
                let lib_path = parent.join("lib.hsy");
                if lib_path.exists() {
                    ModulePath::new_child(
                        db,
                        resolve_module_path(db, toolchain, lib_path)?,
                        Ident::from_ref(db, file_stem).ok_or(VfsError::ModulePathResolveFailure)?,
                    )?
                    .into()
                } else {
                    let main_path = parent.join("main.hsy");
                    if main_path.exists() {
                        ModulePath::new_child(
                            db,
                            resolve_module_path(db, toolchain, main_path)?,
                            Ident::from_ref(db, file_stem)
                                .ok_or(VfsError::ModulePathResolveFailure)?,
                        )?
                        .into()
                    } else {
                        todo!()
                    }
                }
            }
        }
    } else {
        let parent_module_path = parent.with_extension("hsy");
        if !parent_module_path.exists() {
            todo!()
        }
        ModulePath::new_child(
            db,
            resolve_module_path(db, toolchain, parent_module_path)?,
            Ident::from_ref(db, file_stem).ok_or(VfsError::ModulePathResolveFailure)?,
        )?
        .into()
    })
}

#[test]
fn resolve_module_path_works() {
    DB::default().vfs_plain_test(
        |db, module_path| {
            let abs_path = module_virtual_path(db, module_path).unwrap();
            let item_path_resolved = db.resolve_module_path(abs_path.data(db)).unwrap();
            assert_eq!(module_path, item_path_resolved)
        },
        &VfsTestConfig::new("resolve-module-path"),
    )
}
