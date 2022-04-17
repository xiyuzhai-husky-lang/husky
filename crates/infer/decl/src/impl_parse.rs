use crate::*;
use atom::{
    symbol_proxy::{Symbol, SymbolKind},
    *,
};
use entity_route::*;
use fold::LocalStack;
use map_collect::MapCollect;
use word::IdentDict;

impl<'a> dyn DeclQueryGroup + 'a {
    pub(crate) fn parse_entity(
        &self,
        text: &str,
        opt_this_ty: Option<EntityRoutePtr>,
        symbols: &[Symbol],
    ) -> AtomResult<EntityRoutePtr> {
        parse_entity(
            SymbolProxy {
                opt_package_main: None,
                db: self.upcast(),
                opt_this_ty,
                symbols,
            },
            &self.tokenize(text),
        )
    }

    pub(crate) fn parse_generic_placeholders_from_static(
        &self,
        static_generic_placeholders: &[StaticGenericPlaceholder],
    ) -> IdentDict<GenericPlaceholder> {
        static_generic_placeholders.map(|static_generic_placeholder| GenericPlaceholder {
            ident: self.intern_word(static_generic_placeholder.name).custom(),
            variant: GenericPlaceholderVariant::Type { traits: vec![] },
        })
    }

    pub(crate) fn generic_arguments_from_generic_placeholders(
        &self,
        generic_placeholders: &[GenericPlaceholder],
    ) -> Vec<GenericArgument> {
        generic_placeholders.map(|generic_placeholder| {
            GenericArgument::EntityRoute(self.intern_entity_route(EntityRoute {
                kind: EntityRouteKind::Generic {
                    ident: generic_placeholder.ident,
                    entity_kind: generic_placeholder.entity_kind(),
                },
                generic_arguments: vec![],
            }))
        })
    }

    pub(crate) fn symbols_from_generic_placeholders(
        &self,
        generic_placeholders: &[GenericPlaceholder],
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for generic_placeholder in generic_placeholders.iter() {
            symbols.push(Symbol {
                ident: generic_placeholder.ident,
                kind: SymbolKind::EntityRoute(self.intern_entity_route(EntityRoute {
                    kind: EntityRouteKind::Generic {
                        ident: generic_placeholder.ident,
                        entity_kind: generic_placeholder.entity_kind(),
                    },
                    generic_arguments: vec![],
                })),
            })
        }
        symbols
    }
}
