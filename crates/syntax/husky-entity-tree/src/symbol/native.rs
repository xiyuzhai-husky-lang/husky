use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NativeEntitySymbol {
    Submodule(SubmoduleSymbol),
    ModuleItem(ModuleItemSymbol),
}

impl From<NativeEntitySymbol> for EntitySymbol {
    fn from(val: NativeEntitySymbol) -> Self {
        match val {
            NativeEntitySymbol::Submodule(symbol) => EntitySymbol::Submodule(symbol),
            NativeEntitySymbol::ModuleItem(symbol) => EntitySymbol::ModuleItem(symbol),
        }
    }
}

impl NativeEntitySymbol {
    pub(crate) fn ast_idx(self, db: &dyn EntityTreeDb) -> AstIdx {
        match self {
            NativeEntitySymbol::Submodule(_) => todo!(),
            NativeEntitySymbol::ModuleItem(_) => todo!(),
        }
    }
}

impl From<ModuleItemSymbol> for NativeEntitySymbol {
    fn from(v: ModuleItemSymbol) -> Self {
        Self::ModuleItem(v)
    }
}

impl From<SubmoduleSymbol> for NativeEntitySymbol {
    fn from(v: SubmoduleSymbol) -> Self {
        Self::Submodule(v)
    }
}

impl<Db: EntityTreeDb + ?Sized> salsa::DebugWithDb<Db> for NativeEntitySymbol {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<EntityTreeJar>>::as_jar_db(db);
        match self {
            NativeEntitySymbol::Submodule(symbol) => f
                .debug_tuple("Submodule")
                .field(&symbol.debug_with(db, include_all_fields))
                .finish(),
            NativeEntitySymbol::ModuleItem(symbol) => f
                .debug_tuple("ModuleItem")
                .field(&symbol.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
