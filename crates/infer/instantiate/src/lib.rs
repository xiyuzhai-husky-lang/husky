use check_utils::should_eq;
use defn_head::{GenericParameter, GenericPlaceholderVariant};
use entity_route::*;
use entity_syntax::*;
use map_collect::MapCollect;
use print_utils::p;
use word::CustomIdentifier;

pub struct Instantiator<'a> {
    pub db: &'a dyn EntityRouteSalsaQueryGroup,
    pub generic_parameters: &'a [GenericParameter],
    pub dst_generics: &'a [GenericArgument],
}

impl<'a> Instantiator<'a> {
    fn find_generic(&self, ident: CustomIdentifier) -> Option<usize> {
        self.generic_parameters
            .iter()
            .position(|p| p.ident == ident)
    }

    pub fn instantiate_entity_route(&self, src_scope: EntityRoutePtr) -> GenericArgument {
        match self.db.entity_kind(src_scope).unwrap() {
            EntityKind::Module => GenericArgument::EntityRoute(src_scope),
            EntityKind::EnumLiteral => todo!(),
            _ => {
                let (kind, mut generics) = match src_scope.kind {
                    EntityRouteKind::Package { .. } => panic!(),
                    EntityRouteKind::Root { ident } => (src_scope.kind, vec![]),
                    EntityRouteKind::Child { parent, ident } => (
                        EntityRouteKind::Child {
                            parent: self.instantiate_entity_route(parent).take_entity_route(),
                            ident,
                        },
                        vec![],
                    ),
                    EntityRouteKind::Input { main } => todo!(),
                    EntityRouteKind::Generic { ident, .. } => {
                        if let Some(idx) = self.find_generic(ident) {
                            match self.dst_generics[idx] {
                                GenericArgument::Const(value) => {
                                    should_eq!(src_scope.generic_arguments.len(), 0);
                                    return GenericArgument::Const(value);
                                }
                                GenericArgument::EntityRoute(scope) => {
                                    (scope.kind, scope.generic_arguments.clone())
                                }
                            }
                        } else {
                            p!(ident, self.generic_parameters);
                            todo!()
                        }
                    }
                    EntityRouteKind::ThisType => (EntityRouteKind::ThisType, vec![]),
                    EntityRouteKind::TypeAsTraitMember {
                        ty: parent,
                        trai,
                        ident,
                    } => todo!(),
                };
                // convention: A<B,C> = A<B><C>
                generics.extend(self.instantiate_generic_arguments(&src_scope.generic_arguments));
                GenericArgument::EntityRoute(self.db.intern_entity_route(EntityRoute {
                    kind,
                    generic_arguments: generics,
                }))
            }
        }
    }

    fn instantiate_generic_arguments(
        &self,
        src_generic_arguments: &[GenericArgument],
    ) -> Vec<GenericArgument> {
        src_generic_arguments
            .iter()
            .map(|src_generic| match src_generic {
                GenericArgument::Const(_) => *src_generic,
                GenericArgument::EntityRoute(scope) => self.instantiate_entity_route(*scope),
            })
            .collect()
    }

    pub fn instantiate_generic_placeholder(
        &self,
        placeholder: &GenericParameter,
    ) -> Option<GenericParameter> {
        for targeted_placeholder in self.generic_parameters.iter() {
            if targeted_placeholder.ident == placeholder.ident {
                todo!()
            }
        }
        Some(GenericParameter {
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
