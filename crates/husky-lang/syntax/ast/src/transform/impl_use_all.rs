use scope::ScopeRoute;
use text::TextRange;

use crate::{
    atom::symbol_proxy::{Symbol, SymbolKind},
    *,
};

impl<'a> AstTransformer<'a> {
    pub(super) fn use_all(&mut self, parent: ScopePtr, range: TextRange) -> AstResult<()> {
        self.symbols.extend(
            self.db
                .subscope_table(parent)
                .map_err(|_| ast_error!(range, "scope not found"))?
                .entries
                .iter()
                .filter_map(|entry| {
                    entry.ident.map(|ident| Symbol {
                        ident: ident.into(),
                        kind: SymbolKind::Scope(ScopeRoute::ChildScope { parent, ident }),
                    })
                }),
        );
        Ok(())
    }
}
