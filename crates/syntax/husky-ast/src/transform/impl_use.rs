use super::*;
use crate::*;
use husky_atom::context::{Symbol, SymbolKind};
use husky_entity_route::{EntityRoute, EntityRouteVariant};
use husky_text::TextRange;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_use(&mut self, token_group: &[Token]) -> AstResult<AstVariant> {
        if token_group.len() <= 1 {
            return err!("expect route after keyword `use`", token_group.text_range());
        }
        if token_group.last().unwrap().kind
            == TokenKind::Special(SpecialToken::BinaryOpr(BinaryOpr::Pure(PureBinaryOpr::Mul)))
        {
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
                parser.parse_all_atoms()
            })?;
            must_be!(
                atoms.len() == 1,
                "expect one entity route, but get multiple tokens instead",
                token_group[1..(token_group.len() - 2)].text_range()
            );
            let parent = match atoms[0].variant {
                HuskyAtomVariant::EntityRoute { route, .. } => {
                    if route.spatial_arguments.len() != 0 {
                        todo!("expect no generics")
                    }
                    route
                }
                _ => {
                    p!(self.file, atoms[0]);
                    todo!()
                }
            };
            self.use_all(parent, atoms[0].range)?;
            Ok(AstVariant::Use {
                use_variant: UseVariant::All { parent },
            })
        } else {
            // use route
            let atoms = self.parse_atoms(&token_group[1..], |parser| parser.parse_all_atoms())?;
            let route = if atoms.len() != 1 {
                todo!("expect one atom for entity route")
            } else {
                match atoms[0].variant {
                    HuskyAtomVariant::EntityRoute { route, .. } => {
                        if route.spatial_arguments.len() != 0 {
                            todo!("expect no generics")
                        }
                        route
                    }
                    HuskyAtomVariant::Unrecognized(_) => {
                        return err!("unrecognized ident", atoms[0].range)
                    }
                    _ => todo!(),
                }
            };
            self.use_route(route, token_group.last().unwrap().range)?;
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
                        init_ident: ident.into(),
                        kind: SymbolKind::EntityRoute(self.db.intern_entity_route(EntityRoute {
                            variant: EntityRouteVariant::Child {
                                parent,
                                ident: ident.ident,
                            },
                            temporal_arguments: Default::default(),
                            spatial_arguments: Default::default(),
                        })),
                    })
                }),
        );
        Ok(())
    }

    fn use_route(&mut self, route: EntityRoutePtr, range: TextRange) -> AstResult<()> {
        if route.spatial_arguments.len() != 0 {
            todo!()
        }
        self.symbols.push(Symbol {
            init_ident: RangedCustomIdentifier {
                ident: route.ident().custom(),
                range,
            },
            kind: SymbolKind::EntityRoute(route),
        });
        Ok(())
    }
}
