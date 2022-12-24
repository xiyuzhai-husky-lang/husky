use crate::*;
use husky_token::TokenAccessibility;

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
    ModuleItem {
        ident: Identifier,
        accessibility: Accessibility,
        path: ModuleItemPath,
        ast_idx: AstIdx,
    },
    EntityUse {
        ident: Identifier,
        accessibility: Accessibility,
        path: EntityPath,
        ast_idx: AstIdx,
        use_expr_idx: UseExprIdx,
    },
}

impl EntitySymbol {
    pub(crate) fn accessility(&self) -> Accessibility {
        match self {
            EntitySymbol::CrateRoot { module_path, .. } => Accessibility::PublicUnder(*module_path),
            EntitySymbol::Submodule { accessibility, .. }
            | EntitySymbol::ModuleItem { accessibility, .. }
            | EntitySymbol::EntityUse { accessibility, .. } => *accessibility,
        }
    }

    pub(crate) fn ast_idx(&self) -> Option<AstIdx> {
        match self {
            EntitySymbol::CrateRoot { .. } => None,
            EntitySymbol::Submodule { ast_idx, .. }
            | EntitySymbol::ModuleItem { ast_idx, .. }
            | EntitySymbol::EntityUse { ast_idx, .. } => Some(*ast_idx),
        }
    }

    pub(crate) fn is_accessible_from(&self, db: &dyn VfsDb, module_path: ModulePath) -> bool {
        self.accessility().is_accessible_from(db, module_path)
    }

    pub(crate) fn synthesize_accessibility(
        &self,
        db: &dyn VfsDb,
        use_accessibility: Accessibility,
    ) -> EntityTreeResult<Accessibility> {
        todo!()
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
            | EntitySymbol::ModuleItem { ident, .. }
            | EntitySymbol::EntityUse { ident, .. } => *ident,
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            EntitySymbol::CrateRoot { ident, .. }
            | EntitySymbol::Submodule { ident, .. }
            | EntitySymbol::ModuleItem { ident, .. }
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
            EntitySymbol::ModuleItem {
                ident,
                accessibility,
                ast_idx,
                path,
            } => f
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
