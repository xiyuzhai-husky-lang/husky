use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NativeEntitySymbol {
    Submodule(SubmoduleSymbol),
    ModuleItem(ModuleItemSymbol),
}

impl Into<EntitySymbol> for NativeEntitySymbol {
    fn into(self) -> EntitySymbol {
        match self {
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

// impl NativeEntitySymbol {
//     pub(crate) fn ident(&self, db: &dyn EntityTreeDb) -> Identifier {
//         match self {
//             NativeEntitySymbol::Submodule(symbol) => symbol.ident(db),
//             // NativeEntitySymbol::EntityUse(symbol) => symbol.ident(db),
//             NativeEntitySymbol::ModuleItem(symbol) => symbol.ident(db),
//         }
//     }
//     pub(crate) fn accessibility(&self) -> Accessibility {
//         match self {
//             NativeEntitySymbol::Submodule { accessibility, .. }
//             | NativeEntitySymbol::ModuleItem(ModuleItem { accessibility, .. })
//             | NativeEntitySymbol::EntityUse { accessibility, .. } => *accessibility,
//         }
//     }

//     pub(crate) fn export(&self) -> EntitySymbol {
//         EntitySymbol {
//             path: self.path(),
//             accessibility: self.accessibility(),
//         }
//     }

//     pub fn path(&self) -> EntityPath {
//         match self {
//             NativeEntitySymbol::Submodule { module_path, .. } => (*module_path).into(),
//             NativeEntitySymbol::ModuleItem(ModuleItem { path, .. }) => (*path).into(),
//             NativeEntitySymbol::EntityUse { path, .. } => *path,
//         }
//     }

//     pub fn module_item(&self) -> Option<&ModuleItem> {
//         match self {
//             NativeEntitySymbol::ModuleItem(module_item) => Some(module_item),
//             _ => None,
//         }
//     }
// }

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
