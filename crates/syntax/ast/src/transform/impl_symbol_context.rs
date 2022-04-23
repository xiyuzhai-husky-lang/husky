use crate::*;
use atom::symbol::{Symbol, SymbolContext, SymbolContextKind};

impl<'a> AstTransformer<'a> {
    pub(super) fn symbol_context(&self) -> SymbolContext {
        let variant = SymbolContextKind::Normal;
        SymbolContext {
            db: self.db.upcast(),
            symbols: (&self.symbols as &[Symbol]).into(),
            opt_package_main: Some(self.main),
            opt_this_ty: self.this.value(),
            kind: variant,
        }
    }
}
