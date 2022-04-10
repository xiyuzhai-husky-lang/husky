use atom::symbol_proxy::Symbol;
use fold::LocalStack;
use map_collect::MapCollect;
use print_utils::msg_once;
use vec_dict::HasKey;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MemberDecl {
    AssociatedType,
    AssociatedCall,
    Field,
    TypeMethod(Arc<MethodDecl>),
    TraitMethod { trait_route: EntityRoutePtr },
}

impl MemberDecl {
    pub(crate) fn from_trait(trait_route: EntityRoutePtr, trait_member: &TraitMemberDecl) -> Self {
        match trait_member {
            TraitMemberDecl::Method(_) => MemberDecl::TraitMethod { trait_route },
        }
    }
}

impl From<&TypeMemberDecl> for MemberDecl {
    fn from(decl: &TypeMemberDecl) -> Self {
        match decl {
            TypeMemberDecl::Field(_) => todo!(),
            TypeMemberDecl::Method(method_decl) => MemberDecl::TypeMethod(method_decl.clone()),
            TypeMemberDecl::Call => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeMemberDecl {
    Field(Arc<FieldDecl>),
    Method(Arc<MethodDecl>),
    Call,
}

impl TypeMemberDecl {
    pub(crate) fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TypeMemberDecl::Field(_) => todo!(),
            TypeMemberDecl::Method(method_decl) => {
                TypeMemberDecl::Method(method_decl.instantiate(instantiator))
            }
            TypeMemberDecl::Call => todo!(),
        }
    }

    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        member_decl: &StaticTypeMemberDecl,
        this_ty: EntityRoutePtr,
        symbols: &LocalStack<Symbol>,
    ) -> Self {
        match member_decl {
            StaticTypeMemberDecl::Field => todo!(),
            StaticTypeMemberDecl::Method(method_decl) => {
                TypeMemberDecl::Method(MethodDecl::from_static(db, method_decl, this_ty, symbols))
            }
            StaticTypeMemberDecl::Call => todo!(),
        }
    }
}

impl HasKey<CustomIdentifier> for TypeMemberDecl {
    fn key(&self) -> CustomIdentifier {
        match self {
            TypeMemberDecl::Method(method_decl) => method_decl.ident,
            TypeMemberDecl::Field(_) => todo!(),
            TypeMemberDecl::Call => todo!(),
        }
    }
}

impl MemberDecl {
    pub(crate) fn collect_all(
        db: &dyn DeclQueryGroup,
        type_members: &[TypeMemberDecl],
        trait_impls: &[Arc<TraitImplDecl>],
    ) -> Vec<MemberDecl> {
        let mut members: Vec<MemberDecl> = type_members.map(|decl| decl.into());
        for trait_impl in trait_impls {
            let trait_decl = db.trait_decl(trait_impl.route).unwrap();
            msg_once!("associated types");
            for member in trait_decl.members.iter() {
                members.push(MemberDecl::from_trait(trait_impl.route, member))
            }
        }
        members
    }
}
