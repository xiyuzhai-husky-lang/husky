mod field;
mod method;

pub use field::*;
pub use method::*;

use atom::{symbol::Symbol, SymbolContext};
use fold::LocalStack;
use map_collect::MapCollect;
use print_utils::{epin, p};
use vec_map::HasKey;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MemberDecl {
    AssociatedType,
    AssociatedCall,
    TypeField(Arc<FieldDecl>),
    TypeMethod(Arc<MethodDecl>),
    TypeAssociatedCall(Arc<CallDecl>),
    TraitMethodImpl {
        trait_route: EntityRoutePtr,
        method: Arc<MethodDecl>,
    },
    TraitAssociatedTypeImpl {
        ident: CustomIdentifier,
        ty: EntityRoutePtr,
    },
    TraitAssociatedConstSizeImpl {
        ident: CustomIdentifier,
        value: usize,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MemberIdx(pub u8);

impl From<usize> for MemberIdx {
    fn from(raw: usize) -> Self {
        Self(raw.try_into().unwrap())
    }
}

impl MemberDecl {
    pub fn ident(&self) -> CustomIdentifier {
        match self {
            MemberDecl::AssociatedType => todo!(),
            MemberDecl::AssociatedCall => todo!(),
            MemberDecl::TypeField(field) => field.ident,
            MemberDecl::TypeMethod(method) => method.ident,
            MemberDecl::TraitMethodImpl { method, .. } => method.ident,
            MemberDecl::TraitAssociatedTypeImpl { ident, .. } => *ident,
            MemberDecl::TraitAssociatedConstSizeImpl { ident, .. } => *ident,
            MemberDecl::TypeAssociatedCall(call) => call.route.ident().custom(),
        }
    }
}

impl MemberDecl {
    pub(crate) fn from_trait_member_impl(
        trait_route: EntityRoutePtr,
        trait_member_impl: &TraitMemberImplDecl,
    ) -> Self {
        match trait_member_impl {
            TraitMemberImplDecl::Method(method) => MemberDecl::TraitMethodImpl {
                trait_route,
                method: method.clone(),
            },
            TraitMemberImplDecl::AssociatedType { ident, ty } => {
                MemberDecl::TraitAssociatedTypeImpl {
                    ident: *ident,
                    ty: *ty,
                }
            }
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }
}

impl From<&TyMemberDecl> for MemberDecl {
    fn from(decl: &TyMemberDecl) -> Self {
        match decl {
            TyMemberDecl::Field(field_decl) => MemberDecl::TypeField(field_decl.clone()),
            TyMemberDecl::Method(method_decl) => MemberDecl::TypeMethod(method_decl.clone()),
            TyMemberDecl::Call(call_decl) => MemberDecl::TypeAssociatedCall(call_decl.clone()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TyMemberDecl {
    Field(Arc<FieldDecl>),
    Method(Arc<MethodDecl>),
    Call(Arc<CallDecl>),
}

impl TyMemberDecl {
    pub(crate) fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TyMemberDecl::Field(_) => todo!(),
            TyMemberDecl::Method(method_decl) => {
                TyMemberDecl::Method(method_decl.instantiate(instantiator))
            }
            TyMemberDecl::Call(_) => todo!(),
        }
    }

    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        static_defn: &EntityStaticDefn,
        symbol_context: &SymbolContext,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::Method { .. } => {
                TyMemberDecl::Method(MethodDecl::from_static(db, static_defn, symbol_context))
            }
            _ => panic!(""),
        }
    }
}

impl HasKey<CustomIdentifier> for TyMemberDecl {
    fn key(&self) -> CustomIdentifier {
        match self {
            TyMemberDecl::Method(method_decl) => method_decl.ident,
            TyMemberDecl::Field(field_decl) => field_decl.ident,
            TyMemberDecl::Call(call_decl) => call_decl.route.ident().custom(),
        }
    }
}

impl MemberDecl {
    pub(crate) fn collect_all(
        db: &dyn DeclQueryGroup,
        type_members: &[TyMemberDecl],
        trait_impls: &[Arc<TraitImplDecl>],
    ) -> Vec<MemberDecl> {
        let mut members: Vec<MemberDecl> = type_members.map(|decl| decl.into());
        for trait_impl in trait_impls {
            for member in trait_impl.member_impls.iter() {
                members.push(MemberDecl::from_trait_member_impl(
                    trait_impl.trait_route,
                    member,
                ))
            }
        }
        members
    }
}

pub(crate) fn member_idx(db: &dyn DeclQueryGroup, member_route: EntityRoutePtr) -> MemberIdx {
    let this_ty = member_route.parent();
    let this_ty_decl = db.ty_decl(this_ty).unwrap();
    this_ty_decl.member_idx(member_route)
}
