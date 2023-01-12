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
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EntitySymbolTableRef<'a>(&'a [EntitySymbolEntry]);

impl<'a> EntitySymbolTableRef<'a> {
    pub fn resolve_ident(&self, ident: Identifier) -> Option<EntitySymbol> {
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
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntitySymbolEntry {
    ident: Identifier,
    accessibility: Accessibility,
    symbol: EntitySymbol,
}

impl EntitySymbolEntry {
    pub(crate) fn new_crate_root(db: &dyn EntityTreeDb, crate_path: CratePath) -> Self {
        let root = ModulePath::new_root(db, crate_path);
        Self {
            ident: db.word_menu().crate_word(),
            accessibility: Accessibility::PublicUnder(root),
            symbol: EntitySymbol::CrateRoot(root),
        }
    }

    pub fn symbol(&self) -> EntitySymbol {
        self.symbol
    }

    pub fn ident(&self) -> Identifier {
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
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        todo!()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct NativeEntitySymbolTable(Vec<NativeEntitySymbolEntry>);

impl NativeEntitySymbolTable {
    pub(crate) fn entity_symbol_table(&self) -> EntitySymbolTable {
        EntitySymbolTable(self.0.iter().map(|entry| entry.into()).collect())
    }

    pub(crate) fn insert(&mut self, new_entry: NativeEntitySymbolEntry) -> EntityTreeResult<()> {
        self.0.push(new_entry);
        Ok(())
        // todo!()
    }
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for NativeEntitySymbolTable {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NativeEntitySymbolEntry {
    ident: Identifier,
    accessibility: Accessibility,
    symbol: NativeEntitySymbol,
}

impl Into<EntitySymbolEntry> for &NativeEntitySymbolEntry {
    fn into(self) -> EntitySymbolEntry {
        EntitySymbolEntry {
            ident: self.ident,
            accessibility: self.accessibility,
            symbol: self.symbol.into(),
        }
    }
}

impl NativeEntitySymbolEntry {
    pub fn new(
        ident: Identifier,
        accessibility: Accessibility,
        symbol: NativeEntitySymbol,
    ) -> Self {
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
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        todo!()
    }
}
