use check_utils::should_eq;
use defn_head::{GenericPlaceholder, GenericPlaceholderVariant};
use entity_route::*;
use entity_route_query::*;
use map_collect::MapCollect;
use print_utils::p;
use word::CustomIdentifier;

pub struct Instantiator<'a> {
    pub db: &'a dyn EntityRouteSalsaQueryGroup,
    pub generic_placeholders: &'a [GenericPlaceholder],
    pub dst_generics: &'a [GenericArgument],
}

impl<'a> Instantiator<'a> {
    fn find_generic(&self, ident: CustomIdentifier) -> Option<usize> {
        self.generic_placeholders
            .iter()
            .position(|p| p.ident == ident)
    }

    pub fn instantiate_entity_route(&self, src_scope: EntityRoutePtr) -> GenericArgument {
        match self.db.raw_entity_kind(src_scope) {
            EntityKind::Module => GenericArgument::EntityRoute(src_scope),
            EntityKind::Literal => todo!(),
            _ => {
                let (kind, mut generics) = match src_scope.kind {
                    EntityRouteKind::Package { .. } => panic!(),
                    EntityRouteKind::Root { ident } => (src_scope.kind, vec![]),
                    EntityRouteKind::Child { parent, ident } => (
                        EntityRouteKind::Child {
                            parent: self.instantiate_entity_route(parent).as_entity_route(),
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
                            p!(ident, self.generic_placeholders);
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
        placeholder: &GenericPlaceholder,
    ) -> Option<GenericPlaceholder> {
        for targeted_placeholder in self.generic_placeholders.iter() {
            if targeted_placeholder.ident == placeholder.ident {
                todo!()
            }
        }
        Some(GenericPlaceholder {
            ident: placeholder.ident,
            variant: match placeholder.variant {
                GenericPlaceholderVariant::Const => GenericPlaceholderVariant::Const,
                GenericPlaceholderVariant::Type { ref traits } => GenericPlaceholderVariant::Type {
                    traits: traits.map(|trai| todo!()),
                },
            },
        })
    }

    // pub fn instantiate_field_access_decl(&self, signature: &MembAccessDecl) -> MembAccessDecl {
    //     todo!()
    // }

    // pub fn instantiate_field_routine_decl(&self, signature: &MembCallDecl) -> MembCallDecl {
    //     MembCallDecl {
    //         this_contract: signature.this_contract,
    //         inputs: signature
    //             .inputs
    //             .iter()
    //             .map(|input| self.instantiate_input_decl(input))
    //             .collect(),
    //         output: self.instantiate_scope(signature.output).as_scope(),
    //         args: signature.args.clone(),
    //     }
    // }

    // fn instantiate_input_decl(&self, input: &InputDecl) -> InputDecl {
    //     InputDecl {
    //         contract: input.contract,
    //         ty: self.instantiate_scope(input.ty).as_scope(),
    //     }
    // }
}
