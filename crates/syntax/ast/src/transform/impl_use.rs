use super::*;
use crate::*;
use atom::context::{Symbol, SymbolKind};
use entity_route::{EntityRoute, EntityRouteKind};
use text::TextRange;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_use(&mut self, token_group: &[Token]) -> AstResult<AstVariant> {
        if token_group.len() <= 1 {
            return err!("expect route after keyword `use`", token_group.text_range());
        }
        if token_group.last().unwrap().kind == TokenKind::Special(SpecialToken::Star) {
            // use all
            must_be!(
                token_group.len() >= 4,
                "expect at least four tokens for use all",
                token_group.text_range()
            );
            let second_last_token = &token_group[token_group.len() - 2];
            must_be!(
                second_last_token.kind == TokenKind::Special(SpecialToken::DoubleColon),
                "expect `::`",
                second_last_token.range
            );
            let atoms = self.parse_atoms(&token_group[1..(token_group.len() - 2)], |parser| {
                parser.parse_all()
            })?;
            must_be!(
                atoms.len() == 1,
                "expect one entity route, but get multiple tokens instead",
                token_group[1..(token_group.len() - 2)].text_range()
            );
            let parent = match atoms[0].kind {
                AtomVariant::EntityRoute { route, .. } => {
                    if route.spatial_arguments.len() != 0 {
                        todo!("expect no generics")
                    }
                    route
                }
                _ => todo!(),
            };
            self.use_all(parent, atoms[0].range)?;
            Ok(AstVariant::Use {
                use_variant: UseVariant::All { parent },
            })
        } else {
            // use route
            let atoms = self.parse_atoms(&token_group[1..], |parser| parser.parse_all())?;
            let route = if atoms.len() != 1 {
                todo!("expect one atom for entity route")
            } else {
                match atoms[0].kind {
                    AtomVariant::EntityRoute { route, .. } => {
                        if route.spatial_arguments.len() != 0 {
                            todo!("expect no generics")
                        }
                        route
                    }
                    AtomVariant::Unrecognized(_) => {
                        return err!("unrecognized ident", atoms[0].range)
                    }
                    _ => todo!(),
                }
            };
            self.use_route(route)?;
            Ok(AstVariant::Use {
                use_variant: UseVariant::Route { route },
            })
        }
    }

    pub(super) fn use_all(&mut self, parent: EntityRoutePtr, range: TextRange) -> AstResult<()> {
        self.symbols.extend(
            self.db
                .subroute_table(parent)
                .map_err(|_| error!("scope not found", range))?
                .entries
                .iter()
                .filter_map(|entry| {
                    entry.ident.map(|ident| Symbol {
                        ident: ident.into(),
                        kind: SymbolKind::EntityRoute(self.db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child { parent, ident },
                            spatial_arguments: vec![],
                        })),
                    })
                }),
        );
        Ok(())
    }

    fn use_route(&mut self, route: EntityRoutePtr) -> AstResult<()> {
        if route.spatial_arguments.len() != 0 {
            todo!()
        }
        self.symbols.push(Symbol {
            ident: route.ident().custom(),
            kind: SymbolKind::EntityRoute(route),
        });
        Ok(())
    }
}
