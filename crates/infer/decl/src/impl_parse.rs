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
        symbols: &LocalStack<Symbol>,
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

    pub(crate) fn parse_generics(
        &self,
        static_generic_placeholders: &[StaticGenericPlaceholder],
    ) -> (
        IdentDict<GenericPlaceholder>,
        Vec<GenericArgument>,
        LocalStack<Symbol>,
    ) {
        let generic_placeholders: IdentDict<_> =
            static_generic_placeholders.map(|static_generic_placeholder| GenericPlaceholder {
                ident: self.intern_word(static_generic_placeholder.name).custom(),
                variant: GenericPlaceholderVariant::Type { traits: vec![] },
            });
        let generic_arguments: Vec<_> = generic_placeholders.map(|generic_placeholder| {
            GenericArgument::Scope(self.intern_scope(EntityRoute {
                kind: EntityRouteKind::Generic {
                    ident: generic_placeholder.ident,
                    entity_kind: generic_placeholder.entity_kind(),
                },
                generics: vec![],
            }))
        });
        let mut symbols = LocalStack::new();
        for generic_placeholder in generic_placeholders.iter() {
            symbols.push(Symbol {
                ident: generic_placeholder.ident,
                kind: SymbolKind::EntityRoute(self.intern_scope(EntityRoute {
                    kind: EntityRouteKind::Generic {
                        ident: generic_placeholder.ident,
                        entity_kind: generic_placeholder.entity_kind(),
                    },
                    generics: vec![],
                })),
            })
        }
        (generic_placeholders, generic_arguments, symbols)
    }
}
