use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
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
