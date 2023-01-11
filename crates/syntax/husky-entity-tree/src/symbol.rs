use crate::*;
use husky_token::{TokenAccessibility, TokenIdx};
use vec_like::VecMapGetEntry;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ModuleItem {
    ident: Identifier,
    accessibility: Accessibility,
    path: ModuleItemPath,
    ast_idx: AstIdx,
}

impl ModuleItem {
    pub(crate) fn new(
        ident: Identifier,
        accessibility: Accessibility,
        path: ModuleItemPath,
        ast_idx: AstIdx,
    ) -> Self {
        Self {
            ident,
            accessibility,
            path,
            ast_idx,
        }
    }

    pub fn ident(&self) -> Identifier {
        self.ident
    }

    pub fn accessibility(&self) -> Accessibility {
        self.accessibility
    }

    pub fn path(&self) -> ModuleItemPath {
        self.path
    }

    pub fn ast_idx(&self) -> ArenaIdx<Ast> {
        self.ast_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntitySymbol {
    CrateRoot {
        ident: Identifier,
        module_path: ModulePath,
    },
    Submodule {
        ident: Identifier,
        accessibility: Accessibility,
        module_path: ModulePath,
        ast_idx: AstIdx,
    },
    ModuleItem(ModuleItem),
    EntityUse {
        ident: Identifier,
        accessibility: Accessibility,
        path: EntityPath,
        ast_idx: AstIdx,
        use_expr_idx: UseTreeExprIdx,
    },
}

impl EntitySymbol {
    pub(crate) fn ident(&self) -> Identifier {
        match self {
            EntitySymbol::CrateRoot { ident, .. }
            | EntitySymbol::Submodule { ident, .. }
            | EntitySymbol::EntityUse { ident, .. } => *ident,
            EntitySymbol::ModuleItem(module_item) => module_item.ident,
        }
    }
    pub(crate) fn accessility(&self) -> Accessibility {
        match self {
            EntitySymbol::CrateRoot { module_path, .. } => Accessibility::PublicUnder(*module_path),
            EntitySymbol::Submodule { accessibility, .. }
            | EntitySymbol::ModuleItem(ModuleItem { accessibility, .. })
            | EntitySymbol::EntityUse { accessibility, .. } => *accessibility,
        }
    }

    pub(crate) fn ast_idx(&self) -> Option<AstIdx> {
        match self {
            EntitySymbol::CrateRoot { .. } => None,
            EntitySymbol::Submodule { ast_idx, .. }
            | EntitySymbol::ModuleItem(ModuleItem { ast_idx, .. })
            | EntitySymbol::EntityUse { ast_idx, .. } => Some(*ast_idx),
        }
    }

    pub(crate) fn is_accessible_from(&self, db: &dyn VfsDb, module_path: ModulePath) -> bool {
        self.accessility().is_accessible_from(db, module_path)
    }

    pub fn entity_path(&self) -> EntityPath {
        match self {
            EntitySymbol::CrateRoot { module_path, .. }
            | EntitySymbol::Submodule { module_path, .. } => (*module_path).into(),
            EntitySymbol::ModuleItem(ModuleItem { path, .. }) => (*path).into(),
            EntitySymbol::EntityUse { path, .. } => *path,
        }
    }

    pub fn module_item(&self) -> Option<&ModuleItem> {
        match self {
            EntitySymbol::ModuleItem(module_item) => Some(module_item),
            _ => None,
        }
    }
}

impl AsVecMapEntry for EntitySymbol {
    type K = Identifier;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        match self {
            EntitySymbol::CrateRoot { ident, .. }
            | EntitySymbol::Submodule { ident, .. }
            | EntitySymbol::ModuleItem(ModuleItem { ident, .. })
            | EntitySymbol::EntityUse { ident, .. } => *ident,
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            EntitySymbol::CrateRoot { ident, .. }
            | EntitySymbol::Submodule { ident, .. }
            | EntitySymbol::ModuleItem(ModuleItem { ident, .. })
            | EntitySymbol::EntityUse { ident, .. } => ident,
        }
    }
}

impl salsa::DebugWithDb<dyn EntityTreeDb + '_> for EntitySymbol {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityTreeDb,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        match self {
            EntitySymbol::CrateRoot { ident, module_path } => f
                .debug_struct("CrateRoot")
                .field("ident", &ident.debug_with(db, include_all_fields))
                .field(
                    "module_path",
                    &module_path.debug_with(db as &dyn VfsDb, include_all_fields),
                )
                .finish(),
            EntitySymbol::Submodule {
                ident,
                accessibility,
                ast_idx,
                module_path,
            } => f
                .debug_struct("Module")
                .field("ident", &ident.debug_with(db, include_all_fields))
                .field(
                    "accessibility",
                    &accessibility.debug_with(db as &dyn VfsDb, include_all_fields),
                )
                .field("ast_idx", ast_idx)
                .field(
                    "module_path",
                    &module_path.debug_with(db as &dyn VfsDb, include_all_fields),
                )
                .finish(),
            EntitySymbol::ModuleItem(ModuleItem {
                ident,
                accessibility,
                ast_idx,
                path,
            }) => f
                .debug_struct("ModuleItem")
                .field(
                    "ident",
                    &ident.debug_with(db as &dyn WordDb, include_all_fields),
                )
                .field(
                    "accessibility",
                    &accessibility.debug_with(db as &dyn VfsDb, include_all_fields),
                )
                .field("ast_idx", ast_idx)
                .field(
                    "path",
                    &path.debug_with(db as &dyn EntityPathDb, include_all_fields),
                )
                .finish(),
            EntitySymbol::EntityUse {
                ident,
                accessibility,
                ast_idx,
                use_expr_idx,
                path,
            } => f
                .debug_struct("EntityUse")
                .field(
                    "ident",
                    &ident.debug_with(db as &dyn WordDb, include_all_fields),
                )
                .field(
                    "accessibility",
                    &accessibility.debug_with(db as &dyn VfsDb, include_all_fields),
                )
                .field(
                    "path",
                    &path.debug_with(db as &dyn EntityPathDb, include_all_fields),
                )
                .finish(),
        }
    }
}
impl<Db: EntityTreeDb> salsa::DebugWithDb<Db> for EntitySymbol {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityTreeDb, include_all_fields)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModuleSymbolContext<'a> {
    crate_prelude: CrateSymbolContext<'a>,
    module_specific_symbols: &'a [EntitySymbol],
}

impl<'a> ModuleSymbolContext<'a> {
    pub fn new(
        crate_prelude: CrateSymbolContext<'a>,
        module_specific_symbols: &'a [EntitySymbol],
    ) -> Self {
        Self {
            crate_prelude,
            module_specific_symbols,
        }
    }

    pub fn new_default(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> PreludeResult<Self> {
        Ok(Self {
            crate_prelude: crate_prelude(db, crate_path)?,
            module_specific_symbols: &[],
        })
    }

    pub fn resolve_ident(
        &self,
        token_idx: TokenIdx,
        ident: Identifier,
    ) -> Option<&'a EntitySymbol> {
        self.module_specific_symbols
            .get_entry(ident)
            .or_else(|| self.crate_prelude.resolve_ident(ident))
    }
}

pub(crate) fn module_symbol_context<'a>(
    db: &'a dyn EntityTreeDb,
    module_path: ModulePath,
) -> EntityTreeResult<ModuleSymbolContext<'a>> {
    let entity_tree_sheet = db.entity_tree_sheet(module_path)?;
    Ok(ModuleSymbolContext {
        crate_prelude: crate_prelude(db, module_path.crate_path(db))?,
        module_specific_symbols: entity_tree_sheet.module_specific_symbols(),
    })
}
