use crate::*;
use husky_print_utils::*;
use infer_decl::{MemberDecl, TyDecl, TyMemberDecl};

macro_rules! informative_assert_eq {
    ($this: expr, $left_key: expr, $left: expr, $right_key: expr, $right: expr) => {
        if $left_key != $right_key {
            panic!(
                r#"entity defn and decl are not consistent for {}
{BLUE}left{RESET} := {YELLOW}{}{RESET} = {:#?},
{RED}right{RESET} := {YELLOW}{}{RESET} = {:#?}"#,
                $this.base_route,
                stringify!($left),
                $left,
                stringify!($right),
                $right
            )
        }
    };
}

impl EntityDefn {
    pub(crate) fn verify(&self, db: &dyn EntityDefnQueryGroup) {
        match self.variant {
            EntityDefnVariant::Ty { .. } => {
                self.verify_consistency_with_ty_decl(&db.ty_decl(self.base_route).unwrap())
            }
            _ => (),
        }
    }

    fn verify_consistency_with_ty_decl(&self, ty_decl: &TyDecl) {
        match self.variant {
            EntityDefnVariant::Ty {
                ref ty_members,
                ref members,
                ..
            } => {
                assert_eq!(ty_members.len(), ty_decl.ty_members.len());
                for i in 0..ty_members.len() {
                    ty_members.data()[i]
                        .verify_consistency_with_ty_member_decl(&ty_decl.ty_members.data()[i])
                }
                informative_assert_eq!(
                    self,
                    members.len(),
                    members
                        .iter()
                        .map(|member| member.base_route)
                        .collect::<Vec<_>>(),
                    ty_decl.members.len(),
                    ty_decl
                        .members
                        .iter()
                        .map(|member| member.info())
                        .collect::<Vec<_>>()
                );
                for i in 0..members.len() {
                    members[i].verify_consistency_with_member_decl(&ty_decl.members[i])
                }
            }
            _ => panic!(),
        }
    }

    pub fn verify_consistency_with_ty_member_decl(&self, ty_decl: &TyMemberDecl) {
        match self.variant {
            EntityDefnVariant::Method { .. } => match ty_decl {
                TyMemberDecl::Method(method_decl) => method_decl
                    .opt_route
                    .unwrap()
                    .verify_consistency_with_base_route(self.base_route),
                TyMemberDecl::Field(_) => panic!(),
                TyMemberDecl::Call(_) => panic!(),
            },
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                field_ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => match ty_decl {
                TyMemberDecl::Field(field_decl) => {
                    field_decl.ty.verify_consistency_with_base_route(field_ty);
                    assert_eq!(self.ident, field_decl.ident.into())
                }
                _ => panic!(),
            },
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::Func { .. } => msg_once!("todo"),
            EntityDefnVariant::Proc { .. } => msg_once!("todo"),
            _ => panic!("unexpected EntityDefnVariant {:?}", self.variant),
        }
    }

    pub fn verify_consistency_with_member_decl(&self, member_decl: &MemberDecl) {
        match self.variant {
            EntityDefnVariant::Method {
                method_defn_kind, ..
            } => match method_defn_kind {
                MethodDefnKind::TypeMethod { ty } => match member_decl {
                    MemberDecl::TypeMethod(decl) => decl
                        .opt_route
                        .unwrap()
                        .verify_consistency_with_base_route(self.base_route),
                    _ => panic!(),
                },
                MethodDefnKind::TraitMethod { trai } => todo!(),
                MethodDefnKind::TraitMethodImpl { trai } => match member_decl {
                    MemberDecl::TraitMethodImpl { trai, method } => method
                        .opt_route
                        .unwrap()
                        .verify_consistency_with_base_route(self.base_route),
                    _ => panic!(),
                },
            },
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                field_ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => match member_decl {
                MemberDecl::TypeField(field_decl) => {
                    field_decl.ty.verify_consistency_with_base_route(field_ty);
                    assert_eq!(self.ident, field_decl.ident.into())
                }
                _ => panic!(),
            },
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => match member_decl {
                MemberDecl::TraitAssociatedTypeImpl { ident, ty: decl_ty } => {
                    decl_ty.verify_consistency_with_base_route(ty)
                }
                _ => panic!(),
            },
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::Func { .. } => msg_once!("todo"),
            EntityDefnVariant::Proc { .. } => msg_once!("todo"),
            _ => panic!(),
        }
    }
}
