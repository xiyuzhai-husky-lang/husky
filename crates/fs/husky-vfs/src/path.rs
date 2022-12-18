use husky_absolute_path::AbsolutePath;
use husky_entity_path::{EntityPath, EntityPathData};
use husky_package_path::{CrateKind, PackagePath, PackagePathData};
use husky_path_utils::collect_package_dirs;
use salsa::Durability;

use crate::*;

pub(crate) fn package_manifest_path(
    db: &dyn VfsDb,
    package: PackagePath,
) -> VfsResult<AbsolutePath> {
    AbsolutePath::new(&package_dir(db, package).as_ref()?.join("Corgi.toml")).map_err(|e| todo!())
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn package_dir(db: &dyn VfsDb, package: PackagePath) -> VfsResult<AbsolutePath> {
    match db.package_path_data(package) {
        PackagePathData::Builtin { toolchain, .. } => {
            let name = db.package_name(package);
            // ad hoc;
            // shall include toolchain
            AbsolutePath::new(&db.vfs_config().library_dir().join(name)).map_err(|e| e.into())
        }
        PackagePathData::Global { version } => todo!(),
        PackagePathData::Local(path) => Ok(path.clone()),
        PackagePathData::Git(_) => todo!(),
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn module_path(db: &dyn VfsDb, entity_path: EntityPath) -> VfsResult<AbsolutePath> {
    match entity_path.data(db) {
        EntityPathData::CrateRoot(crate_path) => AbsolutePath::new(
            &package_dir(db, crate_path.package_path(db))
                .as_ref()?
                .join(crate_path.relative_path(db).as_ref()),
        )
        .map_err(|e| e.into()),
        EntityPathData::Childpath { parent, ident } => {
            let parent_module_path = module_path(db, parent).as_ref()?;
            let dir = match db.dt_entity_path(parent) {
                EntityPathData::CrateRoot(_) => parent_module_path.parent().unwrap().to_owned(),
                EntityPathData::Childpath { parent, ident } => {
                    parent_module_path.with_extension("")
                }
            };
            AbsolutePath::new(&dir.join(db.dt_ident(ident)).with_extension("hsy"))
                .map_err(|e| e.into())
        }
    }
}

// this shouldn't be tracked
pub(crate) fn resolve_module_path(db: &dyn VfsDb, path: impl AsRef<Path>) -> VfsResult<EntityPath> {
    let path = path.as_ref();
    if path.extension().and_then(|s| s.to_str()) != Some("hsy") {
        todo!()
    }
    let parent = path.parent().ok_or(VfsError::ModulePathResolveFailure)?;
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(VfsError::ModulePathResolveFailure)?;
    Ok(
        if parent.ends_with("src")
            && parent
                .parent()
                .ok_or(VfsError::ModulePathResolveFailure)?
                .join("Corgi.toml")
                .exists()
        {
            match file_stem {
                "lib" =>  EntityPath::new_crate_root( db, db.it_package_path(PackagePathData::Local(AbsolutePath::new(
                        parent.parent().ok_or(VfsError::ModulePathResolveFailure)?,
                    )?)),  CrateKind::Library,
                ) ,
                "main" => EntityPath::new_crate_root( db, 
                    // ad hoc
                    // todo: correctly recognize toolchain
                    db.it_package_path(PackagePathData::Local(AbsolutePath::new(
                        parent.parent().ok_or(VfsError::ModulePathResolveFailure)?,
                    )?)),  CrateKind::Main,
                 ),
                _ => {
                    if let lib_path = parent.join("lib.hsy") && lib_path.exists() {
                        db.it_entity_path(EntityPathData::Childpath {
                            parent: resolve_module_path(db, lib_path)?,
                            ident: db
                                .it_ident_borrowed(file_stem)
                                .ok_or(VfsError::ModulePathResolveFailure)?,
                        })
                    } else if let main_path = parent.join("main.hsy") && main_path.exists() {
                        db.it_entity_path(EntityPathData::Childpath {
                            parent: resolve_module_path(db, main_path)?,
                            ident: db
                                .it_ident_borrowed(file_stem)
                                .ok_or(VfsError::ModulePathResolveFailure)?,
                        })
                    } else {
                        todo!()
                    }
                }
            }
        } else {
            let parent_module_path = parent.with_extension("hsy");
            if !parent_module_path.exists() {
                todo!()
            }
            db.it_entity_path(EntityPathData::Childpath {
                parent: resolve_module_path(db, parent_module_path)?,
                ident: db
                    .it_ident_borrowed(file_stem)
                    .ok_or(VfsError::ModulePathResolveFailure)?,
            })
        },
    )
}

#[test]
fn resolve_module_path_works() {
    use crate::tests::*;

    fn t(db: &DB, dir: &Path) {
        for package in db.collect_local_packages(dir).unwrap() {
            for module in db.collect_possible_modules(package).unwrap() {
                assert_eq!(
                    db.resolve_module_path(module_path(db, module).as_ref().unwrap()),
                    Ok(module)
                )
            }
        }
    }

    let db = DB::default();
    t(&db, db.vfs_config().library_dir());
    t(&db, db.vfs_config().examples_dir());
}
