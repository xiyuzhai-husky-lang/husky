use super::*;
use husky_manifest::HasPackageManifest;

#[derive(Debug, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct CratePrelude<'a> {
    universal_prelude: Option<EntitySymbolTableRef<'a>>,
    crate_specific_symbol_context: EntitySymbolTableRef<'a>,
}

impl<'a> CratePrelude<'a> {
    pub(crate) fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> PreludeResult<Self> {
        let crate_specific_symbol_context = crate_specific_prelude(db, crate_path)
            .as_ref()
            .map(|table| table.as_ref())
            .map_err(|e| e.clone())?;
        let universal_prelude =
            if crate_path == db.vfs_path_menu(crate_path.toolchain(db)).core_library() {
                None
            } else {
                Some(
                    none_core_crate_universal_prelude(db, crate_path.toolchain(db))
                        .as_ref()?
                        .as_ref(),
                )
            };
        Ok(Self {
            universal_prelude,
            crate_specific_symbol_context,
        })
    }

    pub(crate) fn resolve_ident(
        &self,
        db: &dyn EntityTreeDb,
        reference_module_path: ModulePath,
        ident: Ident,
    ) -> Option<EntitySymbol> {
        self.crate_specific_symbol_context
            .resolve_ident(db, reference_module_path.into(), ident)
            .or_else(|| {
                self.universal_prelude?
                    .resolve_ident(db, reference_module_path.into(), ident)
            })
    }
}

pub struct UniversalPrelude {}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn none_core_crate_universal_prelude(
    db: &dyn EntityTreeDb,
    toolchain: Toolchain,
) -> PreludeResult<EntitySymbolTable> {
    let vfs_path_menu = db.vfs_path_menu(toolchain);
    let entity_path_menu = db.entity_path_menu(toolchain);
    let coword_menu = db.coword_menu();
    let core_prelude_module = vfs_path_menu.core_prelude();
    let mut table = EntitySymbolTable::default();
    table.insert(EntitySymbolEntry {
        ident: coword_menu.core_ident(),
        visibility: Scope::Pub,
        symbol: EntitySymbol::UniversalPrelude {
            entity_path: vfs_path_menu.core_root().into(),
        },
    });
    table.extend(
        entity_tree_sheet(db, core_prelude_module)
            .map_err(|e| PreludeError::CorePreludeEntityTreeSheet(Box::new(e)))?
            .module_symbols()
            .data()
            .iter()
            .map(|entry| EntitySymbolEntry {
                ident: entry.ident,
                visibility: Scope::Pub,
                symbol: EntitySymbol::UniversalPrelude {
                    entity_path: entry.symbol.path(db),
                },
            }),
    );
    Ok(table)
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
fn crate_specific_prelude(
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
