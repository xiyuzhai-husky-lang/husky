use crate::*;
use fold::LocalStack;
use husky_atom::{
    context::{AtomContextKind, Symbol, SymbolKind},
    *,
};
use husky_entity_route::*;
use husky_text::RangedCustomIdentifier;
use husky_word::IdentDict;
use map_collect::MapCollect;
use thin_vec::{thin_vec, ThinVec};

impl<'a> dyn DeclQueryGroup + 'a {
    // pub fn parse_entity(
    //     &self,
    //     text: &str,
    //     symbol_context: &mut SymbolContext,
    // ) -> AtomResult<EntityRoutePtr> {
    //     parse_entity(symbol_context, &self.tokenize(text))
    // }

    pub fn generic_parameters_from_static(
        &self,
        static_generic_parameters: &[StaticSpatialParameter],
    ) -> IdentDict<SpatialParameter> {
        static_generic_parameters.map(|static_generic_placeholder| SpatialParameter {
            ident: RangedCustomIdentifier {
                ident: self.intern_word(static_generic_placeholder.name).custom(),
                range: Default::default(),
            },
            variant: SpatialPlaceholderVariant::Type { traits: vec![] },
        })
    }

    pub fn generic_arguments_from_generic_parameters(
        &self,
        generic_parameters: &[SpatialParameter],
    ) -> ThinVec<SpatialArgument> {
        generic_parameters.map(|generic_placeholder| {
            SpatialArgument::EntityRoute(self.intern_entity_route(EntityRoute {
                kind: EntityRouteKind::Generic {
                    ident: generic_placeholder.ident.ident,
                    entity_kind: generic_placeholder.entity_kind(),
                },
                temporal_arguments: thin_vec![],
                spatial_arguments: thin_vec![],
            }))
        })
    }

    pub fn symbols_from_generic_parameters(
        &self,
        generic_parameters: &[SpatialParameter],
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for generic_placeholder in generic_parameters.iter() {
            symbols.push(Symbol {
                init_ident: generic_placeholder.ident,
                kind: SymbolKind::EntityRoute(self.intern_entity_route(EntityRoute {
                    kind: EntityRouteKind::Generic {
                        ident: generic_placeholder.ident.ident,
                        entity_kind: generic_placeholder.entity_kind(),
                    },
                    temporal_arguments: thin_vec![],
                    spatial_arguments: thin_vec![],
                })),
            })
        }
        symbols
    }
}
