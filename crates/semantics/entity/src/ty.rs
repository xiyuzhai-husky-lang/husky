mod member;

pub use member::*;
use print_utils::msg_once;

use std::{iter::Peekable, sync::Arc};

use super::*;
use ast::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use infer_total::InferQueryGroup;
use semantics_eager::{FuncStmt, ProcStmt};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use vec_dict::{HasKey, VecDict};
use vm::{FieldContract, InputContract};
use word::{CustomIdentifier, IdentDict};

#[derive(Debug, PartialEq, Eq)]
pub struct TyDefn {
    pub type_members: IdentDict<TypeMemberDefn>,
    pub variants: IdentDict<EnumVariantDefn>,
    pub kind: TyKind,
    pub trait_impls: Vec<Arc<TraitImplDefn>>,
    pub members: Vec<MemberDefn>,
}

impl TyDefn {
    pub(crate) fn from_ast(
        db: &dyn InferQueryGroup,
        head: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResultArc<TyDefn> {
        let (ident, kind, generic_placeholders) = match head.kind {
            AstKind::TypeDefnHead {
                ident,
                kind,
                ref generic_placeholders,
            } => (ident, kind, generic_placeholders.clone()),
            _ => panic!(),
        };
        let mut children = children.peekable();
        let mut type_members = IdentDict::default();
        let mut trait_impls = Vec::new();
        msg_once!("todo");
        Self::collect_fields(&mut children, &mut type_members)?;
        Self::collect_member_calls(db, arena, file, &mut children, &mut type_members)?;
        let variants = Self::collect_variants(children)?;
        Ok(TyDefn::new(type_members, variants, kind, trait_impls))
    }

    fn new(
        type_members: IdentDict<TypeMemberDefn>,
        variants: IdentDict<EnumVariantDefn>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
    ) -> Arc<Self> {
        let members = MemberDefn::collect_all(&type_members, &trait_impls);
        Arc::new(Self {
            type_members,
            variants,
            kind,
            trait_impls,
            members,
        })
    }

    fn collect_fields(
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<TypeMemberDefn>,
    ) -> SemanticResult<()> {
        while let Some(child) = children.peek() {
            match child.value.as_ref()?.kind {
                AstKind::FieldDefn(ref field_defn_head) => {
                    children.next();
                    members.insert_new(TypeMemberDefn::Field(FieldDefn::from_ast(field_defn_head)?))
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn collect_member_calls(
        db: &dyn InferQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<TypeMemberDefn>,
    ) -> SemanticResult<()> {
        while let Some(child) = children.next() {
            match child.value.as_ref()?.kind {
                AstKind::TypeDefnHead {
                    ident,
                    kind,
                    ref generic_placeholders,
                } => todo!(),
                AstKind::MainDefn => todo!(),
                AstKind::RoutineDefnHead(_) => todo!(),
                AstKind::PatternDefnHead => todo!(),
                AstKind::FeatureDecl { ident, ty } => todo!(),
                AstKind::MembFeatureDefnHead { ident, ty } => todo!(),
                AstKind::MethodDefnHead(ref head) => {
                    let variant = match head.routine_kind {
                        RoutineKind::Proc => todo!(),
                        RoutineKind::Func => {
                            let stmts = semantics_eager::parse_decl_stmts(
                                &head.input_placeholders,
                                db,
                                arena,
                                child.children.unwrap(),
                                file,
                            )?;
                            MethodDefnVariant::Func { stmts }
                        }
                        RoutineKind::Test => todo!(),
                    };
                    members.insert_new(TypeMemberDefn::Method(Arc::new(MethodDefn {
                        ident: head.ident,
                        input_placeholders: head.input_placeholders.clone(),
                        output: head.output,
                        this_contract: head.this_contract,
                        variant,
                    })))
                }
                AstKind::Use { ident, scope } => todo!(),
                AstKind::FieldDefn(_) => todo!(),
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::Stmt(_) => todo!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => todo!(),
            }
        }
        Ok(())
    }

    fn collect_variants(
        mut children: Peekable<AstIter>,
    ) -> SemanticResult<IdentDict<EnumVariantDefn>> {
        let mut variants = VecDict::default();
        for child in children {
            match child.value.as_ref()?.kind {
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: raw_variant_kind,
                } => {
                    variants.insert_new(EnumVariantDefn {
                        ident,
                        variant: match raw_variant_kind {
                            EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
                        },
                    });
                }
                _ => panic!(),
            }
        }
        Ok(variants)
    }

    fn record_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<TyDefn> {
        todo!()
        // let mut fields = VecDict::default();
        // for subitem in children {
        //     match subitem.value.as_ref()?.kind {
        //         AstKind::Use { ident, scope } => (),
        //         AstKind::RoutineDefnHead(_) => todo!(),
        //         AstKind::FieldDefn(ref field_var_defn) => fields.insert_new(field_var_defn.clone()),
        //         AstKind::MembFeatureDefnHead { ident, ty } => {
        //             let stmts = semantics_lazy::parse_lazy_stmts(
        //                 &[],
        //                 db,
        //                 arena,
        //                 subitem.children.unwrap(),
        //                 file,
        //             )?;
        //             fields.insert_new(FieldDefn {
        //                 ident,
        //                 output_ty: ty,
        //                 stmts,
        //             });
        //         }
        //         _ => panic!(),
        //     }
        // }
        // Ok(TyKind::Record { fields })
    }

    pub fn method(&self, member_idx: usize) -> &Arc<MethodDefn> {
        match self.members[member_idx] {
            MemberDefn::TypeField(_) => todo!(),
            MemberDefn::TypeMethod(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MethodKind {
    Func { stmts: Arc<Vec<Arc<FuncStmt>>> },
    Proc { stmts: Arc<Vec<Arc<ProcStmt>>> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumVariantDefn {
    pub ident: CustomIdentifier,
    pub variant: EnumVariantDefnVariant,
}

impl HasKey<CustomIdentifier> for EnumVariantDefn {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDefnVariant {
    Constant,
}

impl EntityDefnVariant {
    pub fn enum_variant(
        db: &dyn EntityDefnQueryGroup,
        ident: CustomIdentifier,
        enum_variant_kind: EnumVariantKind,
        children: Option<AstIter>,
    ) -> EntityDefnVariant {
        EntityDefnVariant::EnumVariant(EnumVariantDefn {
            ident,
            variant: match enum_variant_kind {
                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
            },
        })
    }
}
