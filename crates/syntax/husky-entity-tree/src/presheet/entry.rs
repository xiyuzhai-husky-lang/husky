use super::*;

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
}

impl AsVecMapEntry for EntitySymbolEntry {
    type K = Identifier;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.ident
    }

    fn key_ref(&self) -> &Self::K {
        &self.ident
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

impl AsVecMapEntry for NativeEntitySymbolEntry {
    type K = Identifier;

    fn key(&self) -> Self::K
    where
        Self::K: Copy,
    {
        self.ident
    }

    fn key_ref(&self) -> &Self::K {
        &self.ident
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
