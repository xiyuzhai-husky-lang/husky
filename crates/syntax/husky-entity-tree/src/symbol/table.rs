use husky_manifest::PackageDependency;

use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntitySymbolTable(Vec<EntitySymbolEntry>);

impl EntitySymbolTable {
    pub(crate) fn as_ref<'a>(&'a self) -> EntitySymbolTableRef<'a> {
        EntitySymbolTableRef(&self.0)
    }

    pub(crate) fn data<'a>(&'a self) -> &'a [EntitySymbolEntry] {
        &self.0
    }

    pub(crate) fn insert(&mut self, new_entry: EntitySymbolEntry) -> EntityTreeResult<()> {
        for _ in self.0.iter().filter(|entry| entry.ident == new_entry.ident) {
            // todo!()
            // ad hoc
            return Ok(());
        }
        self.0.push(new_entry);
        Ok(())
        // todo!()
    }

    pub(crate) fn extend(
        &mut self,
        iter: impl IntoIterator<Item = EntitySymbolEntry>,
    ) -> EntityTreeResult<()> {
        for new_entry in iter {
            self.insert(new_entry)?
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EntitySymbolTableRef<'a>(&'a [EntitySymbolEntry]);

impl<'a> EntitySymbolTableRef<'a> {
    pub fn resolve_ident(&self, ident: Ident) -> Option<EntitySymbol> {
        // ad hoc
        // todo: override
        self.0
            .iter()
            .find_map(|entry| (entry.ident == ident).then_some(entry.symbol))
    }

    pub(crate) fn data(&self) -> &'a [EntitySymbolEntry] {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntitySymbolTableRef<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        f.debug_tuple("EntitySymbolTableRef")
            .field(&(&self.0).debug(db))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct EntitySymbolEntry {
    ident: Ident,
    visibility: Visibility,
    symbol: EntitySymbol,
}

impl EntitySymbolEntry {
    pub(crate) fn new_crate_root(db: &dyn EntityTreeDb, crate_path: CratePath) -> Self {
        let root_module_path = ModulePath::new_root(db, crate_path);
        Self {
            ident: db.word_menu().crate_ident(),
            visibility: Visibility::PubUnder(root_module_path),
            symbol: EntitySymbol::CrateRoot { root_module_path },
        }
    }

    pub(crate) fn new_package_dependency(
        db: &dyn EntityTreeDb,
        package_dependency: &PackageDependency,
    ) -> Self {
        let package_path = package_dependency.package_path();
        Self {
            ident: package_path.ident(db),
            visibility: Visibility::Pub,
            symbol: EntitySymbol::PackageDependency {
                entity_path: package_path.lib_module(db).into(),
            },
        }
    }

    pub(crate) fn new_use_symbol_entry(
        db: &dyn EntityTreeDb,
        original_symbol: EntitySymbol,
        rule: &mut UseExprRule,
    ) -> Self {
        rule.mark_as_resolved(original_symbol);
        let visibility = rule.visibility();
        Self {
            ident: rule.ident().unwrap(),
            visibility: visibility,
            symbol: UseSymbol::new(
                db,
                original_symbol,
                original_symbol.path(db),
                visibility,
                rule.ast_idx(),
                rule.use_expr_idx(),
            )
            .into(),
        }
    }

    pub(crate) fn export_via_use_all(
        &self,
        db: &dyn EntityTreeDb,
        target_module_path: ModulePath,
        rule: &UseAllRule,
    ) -> Option<Self> {
        self.symbol
            .is_visible_from(db, target_module_path)
            .then_some(EntitySymbolEntry {
                ident: self.ident,
                visibility: rule.visibility(),
                symbol: UseSymbol::new(
                    db,
                    self.symbol,
                    self.symbol.path(db),
                    rule.visibility(),
                    rule.ast_idx(),
                    rule.use_expr_idx(),
                )
                .into(),
            })
    }

    pub fn symbol(&self) -> EntitySymbol {
        self.symbol
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct NativeEntitySymbolTable(Vec<NativeEntitySymbolEntry>);

impl NativeEntitySymbolTable {
    pub(crate) fn entity_symbol_table(&self) -> EntitySymbolTable {
        EntitySymbolTable(self.0.iter().map(|entry| entry.into()).collect())
    }

    pub(crate) fn insert(
        &mut self,
        db: &dyn EntityTreeDb,
        new_entry: NativeEntitySymbolEntry,
    ) -> EntityTreeResult<()> {
        if let Some(old_entry) = self.0.iter().find(|entry| entry.ident == new_entry.ident) {
            Err(OriginalEntityTreeError::EntitySymbolAlreadyDefined {
                old: old_entry.symbol,
                new: new_entry.symbol,
            })?
        }
        self.0.push(new_entry);
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct NativeEntitySymbolEntry {
    ident: Ident,
    visibility: Visibility,
    symbol: NativeEntitySymbol,
}

impl From<&NativeEntitySymbolEntry> for EntitySymbolEntry {
    fn from(val: &NativeEntitySymbolEntry) -> Self {
        EntitySymbolEntry {
            ident: val.ident,
            visibility: val.visibility,
            symbol: val.symbol.into(),
        }
    }
}

impl NativeEntitySymbolEntry {
    pub fn new(ident: Ident, visibility: Visibility, symbol: NativeEntitySymbol) -> Self {
        Self {
            ident,
            visibility,
            symbol,
        }
    }

    pub fn symbol(&self) -> NativeEntitySymbol {
        self.symbol
    }
}
