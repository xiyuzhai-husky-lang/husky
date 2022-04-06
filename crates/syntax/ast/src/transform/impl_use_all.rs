use entity_route::EntityRouteKind;
use text::TextRange;

use crate::{
    atom::symbol_proxy::{Symbol, SymbolKind},
    *,
};

impl<'a> AstTransformer<'a> {
    pub(super) fn use_all(&mut self, parent: EntityRoutePtr, range: TextRange) -> AstResult<()> {
        self.symbols.extend(
            self.db
                .subscope_table(parent)
                .map_err(|_| error!(Some(self.file), range, "scope not found"))?
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
