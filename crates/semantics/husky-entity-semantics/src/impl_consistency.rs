use crate::*;
use infer_decl::{MemberDecl, TyDecl, TyMemberDecl};

impl EntityDefn {
    pub fn check_consistency_with_ty_decl(&self, ty_decl: &TyDecl) {
        match self.variant {
            EntityDefnVariant::Ty {
                ref ty_members,
                ref members,
                ..
            } => {
                assert_eq!(ty_members.len(), ty_decl.ty_members.len());
                for i in 0..ty_members.len() {
                    ty_members.data()[i]
                        .check_consistency_with_ty_member_decl(&ty_decl.ty_members.data()[i])
                }
                assert_eq!(members.len(), ty_decl.members.len());
                for i in 0..members.len() {
                    members[i].check_consistency_with_member_decl(&ty_decl.members[i])
                }
            }
            _ => panic!(),
        }
    }

    pub fn check_consistency_with_ty_member_decl(&self, ty_decl: &TyMemberDecl) {
        match self.variant {
            EntityDefnVariant::Method { .. } => match ty_decl {
                TyMemberDecl::Method(method_decl) => assert_eq!(
                    method_decl.opt_route.unwrap().variant,
                    self.base_route.variant
                ),
                TyMemberDecl::Field(_) => panic!(),
                TyMemberDecl::Call(_) => panic!(),
            },
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                field_ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            _ => panic!(),
        }
    }

    pub fn check_consistency_with_member_decl(&self, ty_decl: &MemberDecl) {
        match self.variant {
            EntityDefnVariant::Method { .. } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                field_ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            _ => panic!(),
        }
    }
}
