use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EntitySymbol {
    Module {
        ident: Identifier,
        accessibility: Accessibility,
        module_path: ModulePath,
    },
    ModuleItem {
        ident: Identifier,
        accessibility: Accessibility,
        ast_idx: AstIdx,
        path: ModuleItemPath,
    },
    EntityUse {
        ident: Identifier,
        accessibility: Accessibility,
        path: EntityPath,
    },
}

impl EntitySymbol {
    pub(crate) fn accessility(&self) -> Accessibility {
        match self {
            EntitySymbol::Module { accessibility, .. }
            | EntitySymbol::ModuleItem { accessibility, .. }
            | EntitySymbol::EntityUse { accessibility, .. } => *accessibility,
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
            EntitySymbol::Module { ident, .. }
            | EntitySymbol::ModuleItem { ident, .. }
            | EntitySymbol::EntityUse { ident, .. } => *ident,
        }
    }

    fn key_ref(&self) -> &Self::K {
        match self {
            EntitySymbol::Module { ident, .. }
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
            EntitySymbol::Module {
                ident,
                accessibility,
                module_path,
            } => f
                .debug_struct("Submodule")
                .field("ident", &ident.debug_with(db, include_all_fields))
                .field(
                    "accessibility",
                    &accessibility.debug_with(db as &dyn VfsDb, include_all_fields),
                )
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
                .field("path", &path.debug_with(db, include_all_fields))
                .finish(),
            EntitySymbol::EntityUse {
                ident,
                accessibility,
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
