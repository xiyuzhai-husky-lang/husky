use crate::EntityDefn;
use crate::*;
use atom::SymbolContext;
use dev_utils::DevSource;
use map_collect::MapCollect;
use static_defn::{EntityStaticDefn, MethodStaticDefnKind, StaticTraitImplDefn};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDefn {}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDefn {
    pub trai: EntityRoutePtr,
    pub member_impls: Vec<Arc<EntityDefn>>,
    pub dev_src: DevSource,
}

impl TraitImplDefn {
    pub fn from_static(
        symbol_context: &SymbolContext,
        static_trait_impl: &StaticTraitImplDefn,
    ) -> Arc<Self> {
        let trai = symbol_context
            .entity_route_from_str(static_trait_impl.trai)
            .unwrap();
        Arc::new(Self {
            trai,
            member_impls: static_trait_impl.member_impls.map(|static_trait_impl| {
                EntityDefn::trait_member_impl_from_static(symbol_context, trai, static_trait_impl)
                // match static_trait_impl {
                //     StaticTraitMemberImplDecl::Type { name, route } => {
                //     }
                // }
            }),
            dev_src: static_trait_impl.dev_src.into(),
        })
    }

    pub fn member_impl(&self, ident: CustomIdentifier) -> Option<&Arc<EntityDefn>> {
        self.member_impls
            .iter()
            .find(|member_impl| member_impl.ident.custom() == ident)
    }
}

impl EntityDefn {
    pub fn trait_member_impl_from_static(
        symbol_context: &SymbolContext,
        trai: EntityRoutePtr,
        static_trait_impl: &EntityStaticDefn,
    ) -> Arc<Self> {
        let variant = EntityDefnVariant::trait_member_impl_from_static(
            symbol_context,
            trai,
            static_trait_impl,
        );
        let ident = symbol_context
            .db
            .intern_word(static_trait_impl.name)
            .ident();
        Self::new(
            ident,
            variant,
            symbol_context.opt_this_ty.unwrap(),
            symbol_context
                .db
                .intern_file(static_trait_impl.dev_src.file.into()),
            static_trait_impl.dev_src.into(),
        )
    }
}

impl EntityDefnVariant {
    pub fn trait_member_impl_from_static(
        symbol_context: &SymbolContext,
        trai: EntityRoutePtr,
        static_defn: &EntityStaticDefn,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => {
                Self::TraitAssociatedTypeImpl {
                    trai,
                    ty: symbol_context.entity_route_from_str(ty).unwrap(),
                }
            }
            EntityStaticDefnVariant::Method {
                this_contract,
                input_placeholders: inputs,
                output_ty,
                output_contract,
                generic_placeholders,
                kind,
            } => {
                let method_variant = match kind {
                    MethodStaticDefnKind::TypeMethod { source } => todo!(),
                    MethodStaticDefnKind::TraitMethod { opt_default_source } => todo!(),
                    MethodStaticDefnKind::TraitMethodImpl { opt_source } => todo!(),
                };
                // MethodDefnVariant::TraitMethodImpl {
                //             trai,
                //             opt_source: Some(MethodSource::StaticMemberAccess {
                //                 ref_access,
                //                 move_access,
                //                 borrow_mut_access,
                //             }),
                //         };
                Self::Method {
                    input_placeholders: Arc::new(inputs.map(|input_placeholder| {
                        symbol_context.input_placeholder(input_placeholder)
                    })),
                    output_ty: RangedEntityRoute {
                        route: symbol_context.entity_route_from_str(output_ty).unwrap(),
                        range: Default::default(),
                    },
                    this_contract,
                    output_contract,
                    method_variant,
                    generic_placeholders: todo!(),
                }
            }
            _ => panic!(),
        }
    }
}
