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
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct SubmoduleSymbol {
    #[id]
    pub path: ModulePath,
    pub visibility: Scope,
    pub ast_idx: AstIdx,
    pub ident_token: IdentToken,
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar)]
pub struct UseSymbol {
    #[id]
    pub original_symbol: EntitySymbol,
    pub path: EntityPath,
    pub visibility: Scope,
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
    CrateRoot {
        root_module_path: ModulePath,
    },
    SelfModule {
        module_path: ModulePath,
    },
    SuperModule {
        current_module_path: ModulePath,
        super_module_path: ModulePath,
    },
    PackageDependency {
        entity_path: EntityPath,
    },
    Submodule(SubmoduleSymbol),
    ModuleItem(ModuleItemSymbol),
    Use(UseSymbol),
}

impl EntitySymbol {
    fn visibility(self, db: &dyn EntityTreeDb) -> Scope {
        match self {
            EntitySymbol::CrateRoot { root_module_path } => Scope::PubUnder(root_module_path),
            EntitySymbol::SelfModule { module_path } => Scope::Private(module_path),
            EntitySymbol::SuperModule {
                current_module_path,
                ..
            } => Scope::Private(current_module_path),
            EntitySymbol::PackageDependency { .. } => Scope::Pub,
            EntitySymbol::Submodule(symbol) => symbol.visibility(db),
            EntitySymbol::ModuleItem(symbol) => symbol.visibility(db),
            EntitySymbol::Use(symbol) => symbol.visibility(db),
        }
    }

    pub(crate) fn is_visible_from(self, db: &dyn EntityTreeDb, module_path: ModulePath) -> bool {
        self.visibility(db).is_visible_from(db, module_path)
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> EntityPath {
        match self {
            EntitySymbol::CrateRoot { root_module_path } => root_module_path.into(),
            EntitySymbol::SelfModule { module_path } => module_path.into(),
            EntitySymbol::SuperModule {
                super_module_path, ..
            } => super_module_path.into(),
            EntitySymbol::PackageDependency { entity_path } => entity_path.into(),
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
