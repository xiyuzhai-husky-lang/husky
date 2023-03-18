use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
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

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntitySymbolTable {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        f.debug_tuple("EntitySymbolTable")
            .field(&(&self.0).debug(db))
            .finish()
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
pub struct EntitySymbolEntry {
    ident: Ident,
    accessibility: Accessibility,
    symbol: EntitySymbol,
}

impl EntitySymbolEntry {
    pub(crate) fn new_crate_root(db: &dyn EntityTreeDb, crate_path: CratePath) -> Self {
        let root = ModulePath::new_root(db, crate_path);
        Self {
            ident: db.word_menu().crate_ident(),
            accessibility: Accessibility::PublicUnder(root),
            symbol: EntitySymbol::CrateRoot(root),
        }
    }

    pub(crate) fn new_use_symbol_entry(
        db: &dyn EntityTreeDb,
        original_symbol: EntitySymbol,
        rule: &mut UseExprRule,
    ) -> Self {
        rule.mark_as_resolved(original_symbol);
        let accessibility = rule.accessibility();
        Self {
            ident: rule.ident().unwrap(),
            accessibility,
            symbol: UseSymbol::new(
                db,
                original_symbol,
                original_symbol.path(db),
                accessibility,
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
            .is_accessible_from(db, target_module_path)
            .then_some(EntitySymbolEntry {
                ident: self.ident,
                accessibility: rule.accessibility(),
                symbol: UseSymbol::new(
                    db,
                    self.symbol,
                    self.symbol.path(db),
                    rule.accessibility(),
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

    pub fn accessibility(&self) -> Accessibility {
        self.accessibility
    }
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for EntitySymbolEntry {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        f.debug_struct("EntitySymbolEntry")
            .field("ident", &self.ident.debug(db))
            .field("accessibility", &self.accessibility.debug(db))
            .field("symbol", &self.symbol.debug(db))
            .finish()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
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

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for NativeEntitySymbolTable {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        f.debug_tuple("NativeEntitySymbolTable")
            .field(&(&self.0).debug(db))
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NativeEntitySymbolEntry {
    ident: Ident,
    accessibility: Accessibility,
    symbol: NativeEntitySymbol,
}

impl From<&NativeEntitySymbolEntry> for EntitySymbolEntry {
    fn from(val: &NativeEntitySymbolEntry) -> Self {
        EntitySymbolEntry {
            ident: val.ident,
            accessibility: val.accessibility,
            symbol: val.symbol.into(),
        }
    }
}

impl NativeEntitySymbolEntry {
    pub fn new(ident: Ident, accessibility: Accessibility, symbol: NativeEntitySymbol) -> Self {
        Self {
            ident,
            accessibility,
            symbol,
        }
    }

    pub fn symbol(&self) -> NativeEntitySymbol {
        self.symbol
    }
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for NativeEntitySymbolEntry {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        f.debug_struct("NativeEntitySymbolEntry")
            .field("ident", &self.ident.debug(db))
            .field("accessibility", &self.accessibility.debug(db))
            .field("symbol", &self.symbol.debug(db))
            .finish()
    }
}
