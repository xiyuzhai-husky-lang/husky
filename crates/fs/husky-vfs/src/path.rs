mod crate_path;
mod diff_path;
mod menu;
mod module_path;
mod package_path;

pub use self::crate_path::*;
pub use self::diff_path::*;
pub use self::menu::*;
pub use self::module_path::*;
pub use self::package_path::*;
use husky_minimal_toml_utils::read_package_name_from_manifest;

use crate::*;

pub(crate) fn package_manifest_path(db: &dyn VfsDb, package: PackagePath) -> VfsResult<DiffPath> {
    DiffPath::try_new(
        db,
        &package_dir(db, package)
            .as_ref()?
            .path(db)
            .join("Corgi.toml"),
    )
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn module_diff_path(db: &dyn VfsDb, module_path: ModulePath) -> VfsResult<DiffPath> {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => DiffPath::try_new(
            db,
            &package_dir(db, crate_path.package_path(db))
                .as_ref()?
                .path(db)
                .join(crate_path.relative_path(db).as_ref()),
        )
        .map_err(|e| e.into()),
        ModulePathData::Child { parent, ident } => {
            let parent_module_path = module_diff_path(db, parent)?;
            let dir = match parent.data(db) {
                ModulePathData::Root(_) => parent_module_path.path(db).parent().unwrap().to_owned(),
                ModulePathData::Child {
                    parent: _,
                    ident: _,
                } => parent_module_path.path(db).with_extension(""),
            };
            DiffPath::try_new(db, &dir.join(db.dt_ident(ident)).with_extension("hsy"))
        }
    }
}

// this shouldn't be tracked
pub(crate) fn resolve_module_path(
    db: &dyn VfsDb,
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
    let manifest_path = parent
        .parent()
        .ok_or(VfsError::ModulePathResolveFailure)?
        .join("Corgi.toml");
    Ok(if parent.ends_with("src") && manifest_path.exists() {
        let package_name = read_package_name_from_manifest(db, &manifest_path)
            .ok_or(VfsError::FailToReadPackageNameFromManifest)?;
        match file_stem {
            "lib" => CratePath::new(
                PackagePath::new_local_or_toolchain_package(
                    db,
                    toolchain,
                    package_name,
                    parent.parent().ok_or(VfsError::ModulePathResolveFailure)?,
                )?,
                CrateKind::Library,
                db,
            )?
            .root_module_path(db),
            "main" => CratePath::new(
                PackagePath::new_local_or_toolchain_package(
                    db,
                    toolchain,
                    package_name,
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
                        db.it_ident_borrowed(file_stem)
                            .ok_or(VfsError::ModulePathResolveFailure)?,
                    )?
                    .into()
                } else {
                    let main_path = parent.join("main.hsy");
                    if main_path.exists() {
                        ModulePath::new_child(
                            db,
                            resolve_module_path(db, toolchain, main_path)?,
                            db.it_ident_borrowed(file_stem)
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
            db.it_ident_borrowed(file_stem)
                .ok_or(VfsError::ModulePathResolveFailure)?,
        )?
        .into()
    })
}

#[test]
fn resolve_module_path_works() {
    DB::default().vfs_plain_test(
        |db, module_path| {
            let abs_path = module_diff_path(db, module_path).unwrap();
            let toolchain = module_path.toolchain(db);
            let item_path_resolved = db
                .resolve_module_path(toolchain, abs_path.path(db))
                .unwrap();
            assert_eq!(module_path, item_path_resolved)
        },
        &VfsTestConfig::new("resolve-module-path"),
    )
}
