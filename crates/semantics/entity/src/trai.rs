use crate::EntityDefn;
use crate::*;
use atom::SymbolContext;
use map_collect::MapCollect;
use static_defn::{StaticTraitMemberImplDefn, StaticTraitMemberImplDefnVariant};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDefn {}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDefn {
    pub trai: EntityRoutePtr,
    pub member_impls: Vec<Arc<EntityDefn>>,
}

impl TraitImplDefn {
    pub fn from_static(
        symbol_context: &SymbolContext,
        static_trait_impl: &static_defn::StaticTraitImplDefn,
    ) -> Arc<Self> {
        let trai = symbol_context
            .entity_route_from_str(static_trait_impl.route)
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
        })
    }
}

impl EntityDefn {
    pub fn trait_member_impl_from_static(
        symbol_context: &SymbolContext,
        trai: EntityRoutePtr,
        static_trait_impl: &StaticTraitMemberImplDefn,
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
        static_trait_impl: &StaticTraitMemberImplDefn,
    ) -> Self {
        match static_trait_impl.variant {
            StaticTraitMemberImplDefnVariant::Type { route } => Self::TraitAssociatedTypeImpl {
                trai,
                ty: symbol_context.entity_route_from_str(route).unwrap(),
            },
            StaticTraitMemberImplDefnVariant::Method {
                this_contract,
                input_placeholders,
                output,
                ref_access,
                move_access,
                borrow_mut_access,
            } => {
                Self::TraitMethodImpl {
                    trai,
                    input_placeholders: Arc::new(input_placeholders.map(|input_placeholder| {
                        symbol_context.input_placeholder(input_placeholder)
                    })),
                    output: RangedEntityRoute {
                        route: symbol_context.entity_route_from_str(output).unwrap(),
                        range: Default::default(),
                    },
                    this_contract,
                    method_variant: MethodDefnVariant::StaticMemberAccess {
                        ref_access,
                        move_access,
                        borrow_mut_access,
                    },
                }
            }
        }
    }
}
