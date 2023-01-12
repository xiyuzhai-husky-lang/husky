mod native;

pub use native::*;

use crate::*;
use husky_token::{TokenAccessibility, TokenIdx};
use vec_like::VecMapGetEntry;

#[salsa::tracked(jar = EntityTreeJar)]
pub struct ModuleItemSymbol {
    #[id]
    pub path: ModuleItemPath,
    pub accessibility: Accessibility,
    pub ast_idx: AstIdx,
}

#[salsa::tracked(jar = EntityTreeJar)]
pub struct SubmoduleSymbol {
    #[id]
    pub path: ModulePath,
    pub accessibility: Accessibility,
    pub ast_idx: AstIdx,
}

#[salsa::tracked(jar = EntityTreeJar)]
pub struct UseSymbol {
    #[id]
    pub path: EntityPath,
    pub ident: Identifier,
    pub accessibility: Accessibility,
    pub ast_idx: AstIdx,
    pub use_expr_idx: UseExprIdx,
}

impl ModuleItemSymbol {
    pub fn ident(&self, db: &dyn EntityTreeDb) -> Identifier {
        self.path(db).ident(db)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EntitySymbol {
    CrateRoot(ModulePath),
    Submodule(SubmoduleSymbol),
    ModuleItem(ModuleItemSymbol),
    Use(UseSymbol),
}

impl From<UseSymbol> for EntitySymbol {
    fn from(v: UseSymbol) -> Self {
        Self::Use(v)
    }
}

impl EntitySymbol {
    fn accessibility(&self, db: &dyn EntityTreeDb) -> Accessibility {
        match self {
            EntitySymbol::CrateRoot(root) => Accessibility::PublicUnder(*root),
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
            EntitySymbol::CrateRoot(root) => root.into(),
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

    // pub(crate) fn new_crate_root(db: &dyn EntityTreeDb, crate_path: CratePath) -> Self {
    //     Self {
    //         path: todo!(),
    //         accessibility: todo!(),
    //     }
    // }
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntitySymbol {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModuleSymbolContext<'a> {
    crate_prelude: CrateSymbolContext<'a>,
    module_exports: &'a [EntitySymbolEntry],
}

impl<'a> ModuleSymbolContext<'a> {
    pub fn new(
        crate_prelude: CrateSymbolContext<'a>,
        module_exports: &'a [EntitySymbolEntry],
    ) -> Self {
        Self {
            crate_prelude,
            module_exports,
        }
    }

    pub fn new_default(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> PreludeResult<Self> {
        Ok(Self {
            crate_prelude: crate_prelude(db, crate_path)?,
            module_exports: &[],
        })
    }

    pub fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<EntitySymbol> {
        self.module_exports
            .get_entry(ident)
            .map(|t| t.symbol())
            .or_else(|| self.crate_prelude.resolve_ident(ident))
    }
}

pub(crate) fn module_symbol_context<'a>(
    db: &'a dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<ModuleSymbolContext<'a>> {
    let entity_tree_sheet = db.entree_module_sheet(module_path)?;
    Ok(ModuleSymbolContext {
        crate_prelude: crate_prelude(db, module_path.crate_path(db))?,
        module_exports: entity_tree_sheet.module_symbols(),
    })
}
