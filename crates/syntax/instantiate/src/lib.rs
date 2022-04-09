use ast::GenericPlaceholder;
use check_utils::should_eq;
use entity_route::*;
use entity_route_query::*;
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
            EntityKind::Module => GenericArgument::Scope(src_scope),
            EntityKind::Type(_)
            | EntityKind::Trait
            | EntityKind::Routine
            | EntityKind::Feature
            | EntityKind::Pattern => {
                let (kind, mut generics) = match src_scope.kind {
                    EntityRouteKind::Package { .. } => panic!(),
                    EntityRouteKind::Root { ident } => (src_scope.kind, vec![]),
                    EntityRouteKind::ChildScope { parent, ident } => todo!(),
                    EntityRouteKind::Contextual { main, ident } => todo!(),
                    EntityRouteKind::Generic { ident, .. } => {
                        if let Some(idx) = self.find_generic(ident) {
                            match self.dst_generics[idx] {
                                GenericArgument::Const(value) => {
                                    should_eq!(src_scope.generics.len(), 0);
                                    return GenericArgument::Const(value);
                                }
                                GenericArgument::Scope(scope) => {
                                    (scope.kind, scope.generics.clone())
                                }
                            }
                        } else {
                            todo!()
                        }
                    }
                    EntityRouteKind::ThisType => (EntityRouteKind::ThisType, vec![]),
                };
                // convention: A<B,C> = A<B><C>
                generics.extend(self.instantiate_generics(&src_scope.generics));
                GenericArgument::Scope(self.db.intern_scope(EntityRoute { kind, generics }))
            }
            EntityKind::Literal => todo!(),
        }
    }

    fn instantiate_generics(&self, src_generics: &[GenericArgument]) -> Vec<GenericArgument> {
        src_generics
            .iter()
            .map(|src_generic| match src_generic {
                GenericArgument::Const(_) => *src_generic,
                GenericArgument::Scope(scope) => self.instantiate_entity_route(*scope),
            })
            .collect()
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
