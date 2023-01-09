use crate::*;
use husky_manifest::ManifestError;
use husky_token::TokenIdx;
use thiserror::Error;
use vec_like::VecMapGetEntry;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum PreludeError {
    #[error("{0}")]
    Toolchain(#[from] ToolchainError),
    #[error("core prelude")]
    CorePreludeEntityTreeSheet(Box<EntityTreeError>),
    #[error("manifest error {0}")]
    ManifestError(#[from] ManifestError),
}
pub type PreludeResult<T> = Result<T, PreludeError>;

pub(crate) fn module_prelude<'a>(
    db: &'a dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<ModulePrelude<'a>> {
    let entity_tree_sheet = db.entity_tree_sheet(module_path)?;
    Ok(ModulePrelude {
        crate_prelude: crate_prelude(db, module_path.crate_path(db))?,
        module_symbols: entity_tree_sheet.module_symbols(),
    })
}

fn crate_prelude<'a>(
    db: &'a dyn EntityTreeDb,
    crate_path: CratePath,
) -> PreludeResult<CratePrelude<'a>> {
    let toolchain = crate_path.toolchain(db);
    let path_menu = db.path_menu(toolchain)?;
    let core_prelude_module = path_menu.core_prelude();
    Ok(CratePrelude::new(
        module_entity_tree(db, core_prelude_module)
            .map_err(|e| PreludeError::CorePreludeEntityTreeSheet(Box::new(e.clone())))?
            .module_symbols(),
        crate_specific_prelude(db, crate_path)
            .as_ref()
            .map_err(|e| e.clone())?,
    ))
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn crate_specific_prelude(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> PreludeResult<VecMap<EntitySymbol>> {
    let package_path = crate_path.package_path(db);
    let package_dependencies = db.package_dependencies(package_path)?;
    let mut nodes: VecMap<EntitySymbol> = VecMap::default();
    let crate_word = db.word_menu().crate_word();
    nodes.insert(EntitySymbol::CrateRoot {
        ident: crate_word,
        module_path: ModulePath::new_root(db, crate_path),
    });
    nodes
        .extend(
            package_dependencies
                .iter()
                .map(|package_dependency| todo!()),
        )
        .unwrap();
    Ok(nodes)
}

#[derive(Debug, Clone, Copy)]
pub struct CratePrelude<'a> {
    universal_prelude: &'a [EntitySymbol],
    crate_specific_prelude: &'a [EntitySymbol],
}

impl<'a> CratePrelude<'a> {
    pub(crate) fn new(
        universal_prelude: &'a [EntitySymbol],
        crate_specific_prelude: &'a [EntitySymbol],
    ) -> Self {
        Self {
            universal_prelude,
            crate_specific_prelude,
        }
    }

    fn new_default(db: &dyn EntityTreeDb) -> Self {
        todo!()
        // ad hoc
        // let menu = db.entity_path_menu(toolchain);
        // Self {}
    }

    pub(crate) fn resolve_ident(&self, ident: Identifier) -> Option<&'a EntitySymbol> {
        self.crate_specific_prelude
            .get_entry(ident)
            .or_else(|| self.universal_prelude.get_entry(ident))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModulePrelude<'a> {
    crate_prelude: CratePrelude<'a>,
    module_symbols: &'a [EntitySymbol],
}

impl<'a> ModulePrelude<'a> {
    pub fn new(crate_prelude: CratePrelude<'a>, module_symbols: &'a [EntitySymbol]) -> Self {
        Self {
            crate_prelude,
            module_symbols,
        }
    }

    pub fn new_default(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> PreludeResult<Self> {
        Ok(Self {
            crate_prelude: crate_prelude(db, crate_path)?,
            module_symbols: &[],
        })
    }

    pub fn resolve_ident(
        &self,
        token_idx: TokenIdx,
        ident: Identifier,
    ) -> Option<&'a EntitySymbol> {
        self.module_symbols
            .get_entry(ident)
            .or_else(|| self.crate_prelude.resolve_ident(ident))
    }
}
