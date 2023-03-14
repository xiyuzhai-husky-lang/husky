use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
#[enum_class::from_variants]
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
    pub fn ast_idx(self, db: &dyn EntityTreeDb) -> AstIdx {
        match self {
            NativeEntitySymbol::Submodule(symbol) => symbol.ast_idx(db),
            NativeEntitySymbol::ModuleItem(symbol) => symbol.ast_idx(db),
        }
    }

    pub fn ident_token(self, db: &dyn EntityTreeDb) -> IdentToken {
        match self {
            NativeEntitySymbol::Submodule(symbol) => symbol.ident_token(db),
            NativeEntitySymbol::ModuleItem(symbol) => symbol.ident_token(db),
        }
    }
}
