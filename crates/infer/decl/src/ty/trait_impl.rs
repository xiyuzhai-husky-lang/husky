use atom::{
    symbol::{Symbol, SymbolContextKind},
    SymbolContext,
};
use entity_kind::{FieldKind, MemberKind};
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::{msg_once, p};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDecl {
    pub trait_route: EntityRoutePtr,
    pub this_ty: EntityRoutePtr,
    pub member_impls: Vec<TraitMemberImplDecl>,
}

impl TraitImplDecl {
    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        static_trait_impl: &StaticTraitImplDefn,
        symbol_context: &SymbolContext,
    ) -> Arc<Self> {
        let trait_route = symbol_context
            .entity_route_from_str(static_trait_impl.trai)
            .unwrap();
        let trait_decl = db.trait_decl(trait_route).unwrap();
        let member_impls = TraitMemberImplDecl::collect_from_static(
            db,
            symbol_context,
            &trait_decl,
            static_trait_impl.member_impls,
        );
        Arc::new(Self {
            trait_route,
            this_ty: symbol_context.opt_this_ty.unwrap(),
            member_impls,
        })
        // Self::from_trait_decl(db, trait_decl, symbol_context.opt_this_ty.unwrap())
    }

    pub(crate) fn clone_trait_impl(db: &dyn DeclQueryGroup, this_ty: EntityRoutePtr) -> Arc<Self> {
        todo!()
        // Self::from_trait_decl(db, db.trait_decl_menu().clone_trait.clone(), this_ty)
    }

    pub(crate) fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            trait_route: instantiator
                .instantiate_entity_route(self.trait_route)
                .as_entity_route(),
            this_ty: instantiator
                .instantiate_entity_route(self.this_ty)
                .as_entity_route(),
            member_impls: self
                .member_impls
                .map(|member| member.instantiate(instantiator)),
        })
    }

    pub(crate) fn member(&self, ident: CustomIdentifier) -> Option<&TraitMemberImplDecl> {
        self.member_impls
            .iter()
            .find(|impl_decl| impl_decl.ident() == ident)
    }

    pub(crate) fn implicit_trai_impls(
        db: &dyn DeclQueryGroup,
        this_ty: EntityRoutePtr,
        ty_kind: TyKind,
        ty_members: &[TyMemberDecl],
        variants: &[EnumVariantDecl],
    ) -> Vec<Arc<TraitImplDecl>> {
        let mut trait_impl_decls = Vec::new();
        let entity_route_menu = db.entity_route_menu();
        if derive_is_copyable(db, ty_kind, ty_members, variants) {
            trait_impl_decls.push(Arc::new(TraitImplDecl {
                trait_route: entity_route_menu.copy_trait,
                this_ty,
                member_impls: Vec::new(),
            }))
        }
        msg_once!("handle other traits, Clone, PartialEq, Eq");
        // if derive_is_clonable(db, ty_kind, ty_members, variants) {
        //     trait_impl_decls.push(Arc::new(TraitImplDecl {
        //         trait_route: entity_route_menu.clone_trait,
        //         this_ty,
        //         member_impls: Vec::new(),
        //     }))
        // }
        trait_impl_decls
    }
}

fn derive_is_copyable(
    db: &dyn DeclQueryGroup,
    ty_kind: TyKind,
    ty_members: &[TyMemberDecl],
    variants: &[EnumVariantDecl],
) -> bool {
    match ty_kind {
        TyKind::Enum => {
            for variant in variants {
                match variant.variant {
                    EnumVariantDeclVariant::Constant => (),
                }
            }
            true
        }
        TyKind::Record => false,
        TyKind::Struct => false,
        TyKind::Primitive => todo!(),
        TyKind::Vec => todo!(),
        TyKind::Array => todo!(),
        TyKind::Other => todo!(),
    }
}

// fn derive_is_clonable(
//     db: &dyn DeclQueryGroup,
//     ty_kind: TyKind,
//     ty_members: &[TyMemberDecl],
//     variants: &[EnumVariantDecl],
// ) -> bool {
//     match ty_kind {
//         TyKind::Enum => todo!(),
//         TyKind::Record => false,
//         TyKind::Struct => {
//             for ty_member in ty_members {
//                 match ty_member {
//                     TyMemberDecl::Field(field) => {
//                         if field.kind == FieldKind::StructOriginal {
//                             if ! db.is_clonable
//                         }
//                     }
//                     TyMemberDecl::Method(_) | TyMemberDecl::Call(_) => (),
//                 }
//             }
//             true
//         }
//         TyKind::Primitive => todo!(),
//         TyKind::Vec => todo!(),
//         TyKind::Array => todo!(),
//         TyKind::Other => todo!(),
//     }
// }

// fn is_partial_equatable(
//     db: &dyn DeclQueryGroup,
//     ty_kind: TyKind,
//     ty_members: &[TyMemberDecl],
//     variants: &[EnumVariantDecl],
// ) -> bool {
//     match ty_kind {
//         TyKind::Enum => todo!(),
//         TyKind::Record => true,
//         TyKind::Struct => todo!(),
//         TyKind::Primitive => todo!(),
//         TyKind::Vec => todo!(),
//         TyKind::Array => todo!(),
//         TyKind::Other => todo!(),
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberImplDecl {
    Method(Arc<MethodDecl>),
    AssociatedType {
        ident: CustomIdentifier,
        ty: EntityRoutePtr,
    },
    Call {},
    AssociatedConstSize {},
}

impl TraitMemberImplDecl {
    pub fn ident(&self) -> CustomIdentifier {
        match self {
            TraitMemberImplDecl::Method(method_decl) => method_decl.ident,
            TraitMemberImplDecl::AssociatedType { ident, .. } => *ident,
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }

    pub fn kind(&self) -> MemberKind {
        match self {
            TraitMemberImplDecl::Method(_) => MemberKind::Method,
            TraitMemberImplDecl::AssociatedType { .. } => MemberKind::TraitAssociatedType,
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => MemberKind::TraitAssociatedConstSize,
        }
    }

    pub fn generic_argument(&self) -> GenericArgument {
        match self {
            TraitMemberImplDecl::Method(_) => todo!(),
            TraitMemberImplDecl::AssociatedType { ident, ty } => GenericArgument::EntityRoute(*ty),
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }

    pub fn collect_from_static(
        db: &dyn DeclQueryGroup,
        symbol_context: &SymbolContext,
        trait_decl: &TraitDecl,
        static_member_impls: &[EntityStaticDefn],
    ) -> Vec<Self> {
        let member_symbol_impls: Vec<_> = static_member_impls
            .iter()
            .filter_map(|static_member_impl| match static_member_impl.variant {
                EntityStaticDefnVariant::Method { .. } => None,
                EntityStaticDefnVariant::TraitAssociatedType { trai, traits } => todo!(),
                EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => Some((
                    db.intern_word(static_member_impl.name).custom(),
                    GenericArgument::EntityRoute(symbol_context.entity_route_from_str(ty).unwrap()),
                )),
                EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
                _ => panic!(),
                // StaticTraitMemberImplDefnVariant::Type { route } =>

                // StaticTraitMemberImplDefnVariant::Method { .. } => None,
            })
            .collect();
        // let member_impl_context: Vec<_> =
        //     member_impls.map(|member_impl| (member_impl.ident(), member_impl.generic_argument()));
        let this_ty = symbol_context.opt_this_ty.unwrap();
        let implementor = Implementor::new(db.upcast(), this_ty, &member_symbol_impls);

        trait_decl
            .members
            .map(|trait_member_decl| trait_member_decl.implement(db, &implementor))
        //     match trait_member_decl {
        //     TraitMemberDecl::Method(method_decl) => {
        //         TraitMemberImplDecl::Method(method_decl.implement(&implementor))
        //     }
        //     TraitMemberDecl::Type { ident, traits } => {
        //         TraitMemberImplDecl::AssociatedType { ident: (), ty: () }
        //     }
        //     TraitMemberDecl::ConstSize(_) => todo!(),
        //     TraitMemberDecl::Call {} => todo!(),
        // }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TraitMemberImplDecl::Method(method_decl) => {
                TraitMemberImplDecl::Method(method_decl.instantiate(instantiator))
            }
            TraitMemberImplDecl::AssociatedType { ident, ty } => {
                TraitMemberImplDecl::AssociatedType {
                    ident: *ident,
                    ty: instantiator.instantiate_entity_route(*ty).as_entity_route(),
                }
            }
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }
}
