use crate::*;
use husky_manifest::{HasPackageManifest, ManifestError};
use husky_vfs::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum PreludeError {
    #[error("{0}")]
    Toolchain(#[from] ToolchainError),
    #[error("core prelude")]
    CorePreludeEntityTreeSheet(Box<EntityTreeError>),
    #[error("manifest error")]
    ManifestError,
    #[error("vfs error {0}")]
    VfsError(#[from] VfsError),
}
pub type PreludeResult<T> = Result<T, PreludeError>;

impl From<&ManifestError> for PreludeError {
    fn from(value: &ManifestError) -> Self {
        todo!()
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn crate_specific_prelude(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> PreludeResult<EntitySymbolTable> {
    let package_path = crate_path.package_path(db);
    let package_dependencies = package_path.package_dependencies(db)?;
    let mut entries: EntitySymbolTable = Default::default();
    entries.insert(EntitySymbolEntry::new_crate_root(db, crate_path));
    entries.extend(package_dependencies.iter().map(|package_dependency| {
        EntitySymbolEntry::new_package_dependency(db, package_dependency)
    }));
    Ok(entries)
}

pub(crate) fn crate_symbol_context<'a>(
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
) -> PreludeResult<CrateSymbolContext<'a>> {
    let toolchain = crate_path.toolchain(db);
    let path_menu = db.vfs_path_menu(toolchain)?;
    let core_prelude_module = path_menu.core_prelude();
    Ok(CrateSymbolContext::new(
        entity_tree_sheet(db, core_prelude_module)
            .map_err(|e| PreludeError::CorePreludeEntityTreeSheet(Box::new(e.clone())))?
            .module_symbols(),
        crate_specific_prelude(db, crate_path)
            .as_ref()
            .map(|table| table.as_ref())
            .map_err(|e| e.clone())?,
    ))
}

#[test]
fn crate_symbol_context_works() {
    DB::default().ast_plain_test("crate-symbol-context", |db, crate_path| {
        let crate_symbol_context = crate_symbol_context(db, crate_path).unwrap();
        let t = |path: EntityPath| {
            let symbol = match crate_symbol_context.resolve_ident(path.ident(db)) {
                Some(symbol) => symbol,
                None => panic!(
                    r#"crate symbol context should contain {:?}
    crate symbol context is
    {:?}"#,
                    &path.debug(db),
                    crate_symbol_context.debug(db)
                ),
            };
            if symbol.path(db) != path {
                panic!(
                    "symbol.path(db)({:?}) should be equal to path({:?})",
                    symbol.path(db).debug(db),
                    path.debug(db)
                )
            }
        };
        let toolchain = crate_path.toolchain(db);
        let _vfs_path_menu = db.vfs_path_menu(toolchain).unwrap();
        let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
        t(entity_path_menu.bool_ty_path().into());
        t(entity_path_menu.i32_ty_path().into());
        t(entity_path_menu.i64_ty_path().into());
        t(entity_path_menu.f32_ty_path().into());
        t(entity_path_menu.f64_ty_path().into());
    })
}

#[derive(Debug, Clone, Copy)]
pub struct CrateSymbolContext<'a> {
    universal_prelude: EntitySymbolTableRef<'a>,
    crate_specific_symbol_context: EntitySymbolTableRef<'a>,
}

impl<'a> CrateSymbolContext<'a> {
    pub(crate) fn new(
        universal_prelude: EntitySymbolTableRef<'a>,
        crate_specific_symbol_context: EntitySymbolTableRef<'a>,
    ) -> Self {
        Self {
            universal_prelude,
            crate_specific_symbol_context,
        }
    }

    fn new_default(_db: &dyn EntityTreeDb) -> Self {
        todo!()
        // ad hoc
        // let menu = db.entity_path_menu(toolchain);
        // Self {}
    }

    pub(crate) fn resolve_ident(&self, ident: Ident) -> Option<EntitySymbol> {
        self.crate_specific_symbol_context
            .resolve_ident(ident)
            .or_else(|| self.universal_prelude.resolve_ident(ident))
    }
}

impl<'a, Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for CrateSymbolContext<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        f.debug_struct("CrateSymbolContext")
            .field(
                "universal_prelude",
                &self.universal_prelude.debug_with(db, level.next()),
            )
            .field(
                "crate_specific_symbol_context",
                &(&self.crate_specific_symbol_context).debug_with(db, level.next()),
            )
            .finish()
    }
}
