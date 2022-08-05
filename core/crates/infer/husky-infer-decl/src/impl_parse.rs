use crate::*;
use fold::LocalStack;
use husky_atom::{
    context::{AtomContextKind, Symbol, SymbolKind},
    *,
};
use husky_entity_route::*;
use husky_text::{RangedCustomIdentifier, TextRange};
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

    pub fn spatial_parameters_from_static(
        &self,
        static_generic_parameters: &[StaticSpatialParameter],
    ) -> IdentDict<SpatialParameter> {
        static_generic_parameters.map(|static_spatial_parameter| SpatialParameter {
            ident: RangedCustomIdentifier {
                ident: self.intern_word(static_spatial_parameter.name).custom(),
                range: Default::default(),
            },
            variant: SpatialParameterVariant::Type { traits: vec![] },
            file: self.intern_file(static_spatial_parameter.dev_src.file.into()),
            range: TextRange::from_line(static_spatial_parameter.dev_src.line),
        })
    }

    pub fn spatial_arguments_from_spatial_parameters(
        &self,
        spatial_parameters: &[SpatialParameter],
    ) -> ThinVec<SpatialArgument> {
        spatial_parameters.map(|spatial_parameter| {
            SpatialArgument::EntityRoute(self.intern_entity_route(EntityRoute {
                variant: EntityRouteVariant::Generic {
                    ident: spatial_parameter.ident.ident,
                    entity_kind: spatial_parameter.entity_kind(),
                    file: spatial_parameter.file,
                    range: spatial_parameter.range,
                },
                temporal_arguments: thin_vec![],
                spatial_arguments: thin_vec![],
            }))
        })
    }

    pub fn symbols_from_spatial_parameters(
        &self,
        spatial_parameters: &[SpatialParameter],
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for spatial_parameter in spatial_parameters.iter() {
            symbols.push(Symbol {
                init_ident: spatial_parameter.ident,
                kind: SymbolKind::EntityRoute(self.intern_entity_route(EntityRoute {
                    variant: EntityRouteVariant::Generic {
                        ident: spatial_parameter.ident.ident,
                        entity_kind: spatial_parameter.entity_kind(),
                        file: spatial_parameter.file,
                        range: spatial_parameter.range,
                    },
                    temporal_arguments: thin_vec![],
                    spatial_arguments: thin_vec![],
                })),
            })
        }
        symbols
    }
}
