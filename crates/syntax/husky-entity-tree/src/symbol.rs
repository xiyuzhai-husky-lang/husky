mod native;
mod table;

pub use native::*;
pub use table::*;

use crate::*;
use husky_token::{IdentToken, TokenIdx};

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct ModuleItemSymbol {
    #[id]
    pub path: ModuleItemPath,
    pub accessibility: Visibility,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct SubmoduleSymbol {
    #[id]
    pub path: ModulePath,
    pub accessibility: Visibility,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct UseSymbol {
    #[id]
    pub original_symbol: EntitySymbol,
    pub path: EntityPath,
    pub accessibility: Visibility,
    pub ast_idx: AstIdx,
    pub use_expr_idx: UseExprIdx,
}

impl ModuleItemSymbol {
    pub fn ident(&self, db: &dyn EntityTreeDb) -> Ident {
        self.path(db).ident(db)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
pub enum EntitySymbol {
    CrateRoot { root_module: ModulePath },
    PackageDependency { lib_module: ModulePath },
    Submodule(SubmoduleSymbol),
    ModuleItem(ModuleItemSymbol),
    Use(UseSymbol),
}

impl EntitySymbol {
    fn accessibility(&self, db: &dyn EntityTreeDb) -> Visibility {
        match self {
            EntitySymbol::CrateRoot { root_module } => Visibility::PublicUnder(*root_module),
            EntitySymbol::PackageDependency { .. } => Visibility::Public,
            EntitySymbol::Submodule(symbol) => symbol.accessibility(db),
            EntitySymbol::ModuleItem(symbol) => symbol.accessibility(db),
            EntitySymbol::Use(symbol) => symbol.accessibility(db),
        }
    }

    pub(crate) fn is_accessible_from(self, db: &dyn EntityTreeDb, module_path: ModulePath) -> bool {
        self.accessibility(db).is_accessible_from(db, module_path)
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> EntityPath {
        match self {
            EntitySymbol::CrateRoot { root_module } => root_module.into(),
            EntitySymbol::PackageDependency { lib_module } => lib_module.into(),
            EntitySymbol::Submodule(symbol) => symbol.path(db).into(),
            EntitySymbol::ModuleItem(symbol) => symbol.path(db).into(),
            EntitySymbol::Use(symbol) => symbol.path(db).into(),
        }
    }

    pub fn module_item_symbol(self) -> Option<ModuleItemSymbol> {
        match self {
            EntitySymbol::ModuleItem(symbol) => Some(symbol),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModuleSymbolContext<'a> {
    crate_prelude: CrateSymbolContext<'a>,
    module_symbols: EntitySymbolTableRef<'a>,
}

impl<'a> ModuleSymbolContext<'a> {
    pub fn new(
        crate_prelude: CrateSymbolContext<'a>,
        module_symbols: EntitySymbolTableRef<'a>,
    ) -> Self {
        Self {
            crate_prelude,
            module_symbols,
        }
    }

    pub fn new_default(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> PreludeResult<Self> {
        Ok(Self {
            crate_prelude: crate_symbol_context(db, crate_path)?,
            module_symbols: Default::default(),
        })
    }

    pub fn resolve_ident(&self, _token_idx: TokenIdx, ident: Ident) -> Option<EntitySymbol> {
        self.module_symbols
            .resolve_ident(ident)
            .or_else(|| self.crate_prelude.resolve_ident(ident))
    }
}

pub(crate) fn module_symbol_context<'a>(
    db: &'a dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<ModuleSymbolContext<'a>> {
    let entity_tree_sheet = db.entity_tree_sheet(module_path)?;
    Ok(ModuleSymbolContext {
        crate_prelude: crate_symbol_context(db, module_path.crate_path(db))?,
        module_symbols: entity_tree_sheet.module_symbols().into(),
    })
}
