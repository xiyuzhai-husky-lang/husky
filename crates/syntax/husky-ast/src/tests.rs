use crate::*;
use expect_test::expect_file;
use husky_absolute_path::AbsolutePath;
use husky_entity_path::{CratePathKind, EntityPathData, EntityPathDb, EntityPathJar};
use husky_package_path::{PackagePathData, PackagePathDb, PackagePathJar};
use husky_print_utils::p;
use husky_token::TokenJar;
use husky_toml_ast::TomlAstJar;
use husky_toml_token::TomlTokenJar;
use husky_toolchain::ToolchainJar;
use husky_vfs::*;
use husky_word::{WordDb, WordJar};
use salsa::{Database, DebugWithDb, ParallelDatabase, Snapshot};
use std::{borrow::Cow, sync::Arc};

#[salsa::db(
    WordJar,
    ToolchainJar,
    PackagePathJar,
    EntityPathJar,
    VfsJar,
    TokenJar,
    AstJar
)]
#[derive(Default)]
struct MimicDB {
    storage: salsa::Storage<Self>,
    source_path_config: VfsConfigMimic,
}

impl HasVfsConfig for MimicDB {
    fn vfs_config(&self) -> &VfsConfig {
        &self.source_path_config
    }
}

impl Database for MimicDB {}

impl ParallelDatabase for MimicDB {
    fn snapshot(&self) -> Snapshot<Self> {
        Snapshot::new(MimicDB {
            storage: self.storage.snapshot(),
            source_path_config: self.source_path_config.clone(),
        })
    }
}

#[test]
fn library_ast_works() {
    let db = MimicDB::default();
    let package_path_menu = db.package_path_menu();
    let entity_path_menu = db.entity_path_menu();

    macro_rules! t {
        ($($module:ident),*) => {
            $(
                expect_file![format!("../tests/single/{}_ast_sheet.txt", stringify!($module))]
                    .assert_eq(&format!("{:#?}", db.ast_sheet(entity_path_menu.$module())))
            );*
        }
    }

    t!(core, core_basic, core_num, std);
}

#[test]
fn examples_ast_works() {
    fn mimic_dir(db: &dyn VfsDb, module: EntityPath) -> PathBuf {
        match db.dt_entity_path(module) {
            EntityPathData::Crate { package, kind } => {
                db.package_dir(package).as_ref().unwrap().join("ast")
            }
            EntityPathData::Childpath { parent, ident } => {
                mimic_dir(db, parent).join(db.dt_ident(ident))
            }
        }
    }

    fn mimic_path(db: &MimicDB, module: EntityPath) -> PathBuf {
        let dir = mimic_dir(db, module);
        dir.join(format!(
            "{}.ast.txt",
            match db.dt_entity_path(module) {
                EntityPathData::Crate { package, kind } => match kind {
                    CratePathKind::Library => "lib",
                    CratePathKind::Main => "main",
                    CratePathKind::Binary(_) => todo!(),
                },
                EntityPathData::Childpath { ident, .. } => db.dt_ident(ident),
            }
        ))
    }

    use husky_path_utils::*;
    let db = MimicDB::default();
    let cargo_manifest_dir = cargo_manifest_dir().unwrap();
    let examples_dir = cargo_manifest_dir
        .join("tests/examples")
        .canonicalize()
        .unwrap();
    collect_package_dirs(examples_dir)
        .into_iter()
        .for_each(|path| {
            let package =
                db.it_package_path(PackagePathData::Local(AbsolutePath::new(&path).unwrap()));
            for module in db.all_possible_modules(package).unwrap() {
                let path = mimic_path(&db, module);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                expect_file![path].assert_debug_eq(db.ast_sheet(module))
            }
        });
}
