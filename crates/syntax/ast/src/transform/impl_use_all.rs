use crate::*;
use atom::symbol_proxy::{Symbol, SymbolKind};
use entity_route::EntityRouteKind;
use text::TextRange;

impl<'a> AstTransformer<'a> {
    pub(super) fn use_all(&mut self, parent: EntityRoutePtr, range: TextRange) -> AstResult<()> {
        self.symbols.extend(
            self.db
                .subscope_table(parent)
                .map_err(|_| error!("scope not found", range))?
                .entries
                .iter()
                .filter_map(|entry| {
                    entry.ident.map(|ident| Symbol {
                        ident: ident.into(),
                        kind: SymbolKind::Scope(EntityRouteKind::ChildScope { parent, ident }),
                    })
                }),
        );
        Ok(())
    }
}
