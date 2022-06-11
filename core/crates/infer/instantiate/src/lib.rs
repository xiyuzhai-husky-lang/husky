use check_utils::should_eq;
use defn_head::{GenericPlaceholderVariant, SpatialParameter};
use entity_route::*;
use entity_syntax::*;
use map_collect::MapCollect;
use print_utils::p;
use thin_vec::{thin_vec, ThinVec};
use word::CustomIdentifier;

pub struct Instantiator<'a> {
    pub db: &'a dyn EntitySyntaxSalsaQueryGroup,
    pub generic_parameters: &'a [SpatialParameter],
    pub dst_generics: &'a [SpatialArgument],
}

impl<'a> Instantiator<'a> {
    fn find_generic(&self, ident: CustomIdentifier) -> Option<usize> {
        self.generic_parameters
            .iter()
            .position(|p| p.ident.ident == ident)
    }

    pub fn instantiate_entity_route(&self, src_scope: EntityRoutePtr) -> SpatialArgument {
        match self.db.entity_kind(src_scope).unwrap() {
            EntityKind::Module => SpatialArgument::EntityRoute(src_scope),
            EntityKind::EnumLiteral => todo!(),
            _ => {
                let (kind, mut generics) = match src_scope.kind {
                    EntityRouteKind::Package { .. } => panic!(),
                    EntityRouteKind::Root { ident } => (src_scope.kind, thin_vec![]),
                    EntityRouteKind::Child { parent, ident } => (
                        EntityRouteKind::Child {
                            parent: self.instantiate_entity_route(parent).take_entity_route(),
                            ident,
                        },
                        thin_vec![],
                    ),
                    EntityRouteKind::Input { main } => todo!(),
                    EntityRouteKind::Generic { ident, .. } => {
                        if let Some(idx) = self.find_generic(ident) {
                            match self.dst_generics[idx] {
                                SpatialArgument::Const(value) => {
                                    should_eq!(src_scope.spatial_arguments.len(), 0);
                                    return SpatialArgument::Const(value);
                                }
                                SpatialArgument::EntityRoute(scope) => {
                                    (scope.kind, scope.spatial_arguments.clone())
                                }
                            }
                        } else {
                            p!(ident, self.generic_parameters);
                            todo!()
                        }
                    }
                    EntityRouteKind::ThisType => (EntityRouteKind::ThisType, thin_vec![]),
                    EntityRouteKind::TypeAsTraitMember {
                        ty: parent,
                        trai,
                        ident,
                    } => todo!(),
                };
                // convention: A<B,C> = A<B><C>
                generics.extend(self.instantiate_generic_arguments(&src_scope.spatial_arguments));
                SpatialArgument::EntityRoute(self.db.intern_entity_route(EntityRoute {
                    kind,
                    temporal_arguments: thin_vec![],
                    spatial_arguments: generics,
                }))
            }
        }
    }

    fn instantiate_generic_arguments(
        &self,
        src_generic_arguments: &[SpatialArgument],
    ) -> ThinVec<SpatialArgument> {
        src_generic_arguments
            .iter()
            .map(|src_generic| match src_generic {
                SpatialArgument::Const(_) => *src_generic,
                SpatialArgument::EntityRoute(scope) => self.instantiate_entity_route(*scope),
            })
            .collect()
    }

    pub fn instantiate_generic_placeholder(
        &self,
        placeholder: &SpatialParameter,
    ) -> Option<SpatialParameter> {
        for targeted_placeholder in self.generic_parameters.iter() {
            if targeted_placeholder.ident == placeholder.ident {
                todo!()
            }
        }
        Some(SpatialParameter {
            ident: placeholder.ident,
            variant: match placeholder.variant {
                GenericPlaceholderVariant::Const => GenericPlaceholderVariant::Const,
                GenericPlaceholderVariant::Type { ref traits } => GenericPlaceholderVariant::Type {
                    traits: traits.map(|trai| todo!()),
                },
            },
        })
    }
}
