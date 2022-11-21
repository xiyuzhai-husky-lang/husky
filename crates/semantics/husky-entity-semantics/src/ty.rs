mod member;
mod trait_impl;
mod type_call;

pub use member::*;
pub use type_call::*;

use super::*;
use husky_ast::*;
use husky_path::PathItd;
use husky_print_utils::{msg_once, p};
use husky_semantics_error::SemanticResult;
use husky_term::Ty;
use husky_text::*;
use husky_word::{CustomIdentifier, IdentDict};
use std::{iter::Peekable, sync::Arc};
use vec_like::VecMap;

impl EntityDefnVariant {
    pub(crate) fn ty_from_ast(
        db: &dyn EntityDefnQueryGroup,
        ty: Ty,
        head: &Ast,
        children: AstIter,
        arena: &ExprArena,
        file: PathItd,
    ) -> SemanticResult<EntityDefnVariant> {
        todo!();
        // let (kind, generic_parameters) = match head.variant {
        //     AstVariant::TypeDefnHead {
        //         kind,
        //         ref spatial_parameters,
        //         ..
        //     } => (kind, spatial_parameters.clone()),
        //     _ => panic!(),
        // };
        // let mut children = children.peekable();
        // let mut ty_members = IdentDict::default();
        // msg_once!("should pass this_ty for collect_trait_impls");
        // let trait_impls = Self::collect_trait_impls(db, ty, file, head.range);
        // msg_once!("todo");

        // let variants = match kind {
        //     TyKind::Enum => Self::collect_variants(db, file, ty, &mut children)?,
        //     _ => Default::default(),
        // };

        // Self::collect_original_fields(db, arena, file, ty, &mut children, &mut ty_members)?;

        // let opt_type_call = match kind {
        //     TyKind::Enum => None,
        //     TyKind::Record => Some(Arc::new(TypeCallDefn {
        //         parameters: Arc::new(ty_members.map(|ty_member| match ty_member.variant {
        //             EntityDefnVariant::TyField {
        //                 field_ty,
        //                 ref field_variant,
        //                 liason,
        //                 ..
        //             } => match field_variant {
        //                 FieldDefnVariant::RecordOriginal => Parameter::from_field(
        //                     db.upcast(),
        //                     RangedCustomIdentifier {
        //                         ident: ty_member.ident.custom(),
        //                         range: Default::default(),
        //                     },
        //                     liason,
        //                     field_ty,
        //                 ),
        //                 _ => {
        //                     p!(field_variant);
        //                     panic!()
        //                 }
        //             },
        //             _ => panic!(),
        //         })),
        //         output_ty: Ty {
        //             entity_path: EntityPathItd,
        //             range: Default::default(),
        //         },
        //         opt_linkage: None,
        //     })),
        //     TyKind::Struct => Some(Arc::new(TypeCallDefn {
        //         parameters: Arc::new(ty_members.map(|ty_member| match ty_member.variant {
        //             EntityDefnVariant::TyField {
        //                 field_ty: ty,
        //                 ref field_variant,
        //                 liason,
        //                 ..
        //             } => match field_variant {
        //                 FieldDefnVariant::StructOriginal => Parameter::from_field(
        //                     db.upcast(),
        //                     RangedCustomIdentifier {
        //                         ident: ty_member.ident.custom(),
        //                         range: Default::default(),
        //                     },
        //                     liason,
        //                     ty,
        //                 ),
        //                 _ => panic!(),
        //             },
        //             _ => panic!(),
        //         })),
        //         output_ty: Ty {
        //             entity_path: EntityPathItd,
        //             range: Default::default(),
        //         },
        //         opt_linkage: None,
        //     })),
        //     TyKind::Primitive => todo!(),
        //     TyKind::Vec => todo!(),
        //     TyKind::Array => todo!(),
        //     TyKind::Slice => todo!(),
        //     TyKind::CyclicSlice => todo!(),
        //     TyKind::Tuple => todo!(),
        //     TyKind::Mor => todo!(),
        //     TyKind::ThickFp => todo!(),
        //     TyKind::AssociatedAny => todo!(),
        //     TyKind::TargetOutputAny => todo!(),
        //     TyKind::ThisAny => todo!(),
        //     TyKind::SpatialPlaceholderAny => todo!(),
        //     TyKind::BoxAny => todo!(),
        //     TyKind::HigherKind => todo!(),
        //     TyKind::Ref => todo!(),
        //     TyKind::Option => todo!(),
        // };
        // Self::collect_other_ty_members(db, arena, file, ty, &mut children, &mut ty_members)?;
        // let visualizer = Self::visualizer_from_ast(db, arena, file, ty, &mut children);
        // Ok(EntityDefnVariant::new_ty(
        //     generic_parameters,
        //     ty_members,
        //     variants,
        //     kind,
        //     trait_impls,
        //     opt_type_call,
        //     visualizer,
        // ))
    }

    fn new_ty(
        generic_parameters: IdentDict<SpatialParameter>,
        ty_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
        opt_type_call: Option<Arc<TypeCallDefn>>,
        visualizer: Arc<Visualizer>,
    ) -> Self {
        let members = collect_all_members(&ty_members, &trait_impls);
        EntityDefnVariant::Ty {
            spatial_parameters: generic_parameters,
            ty_members,
            variants,
            ty_kind: kind,
            trait_impls,
            members,
            opt_type_call,
            visualizer,
        }
    }

    fn collect_variants(
        db: &dyn EntityDefnQueryGroup,
        file: PathItd,
        ty_route: Ty,
        children: &mut Peekable<AstIter>,
    ) -> SemanticResult<IdentDict<Arc<EntityDefn>>> {
        todo!()
        // let mut variants = VecMap::default();
        // while let Some(child) = children.peek() {
        //     let ast = child.value.as_ref().unwrap();
        //     match ast.variant {
        //         AstVariant::EnumVariantDefnHead {
        //             ident,
        //             variant_class: raw_variant_kind,
        //         } => {
        //             variants
        //                 .insert_new(EntityDefn::new(
        //                     db,
        //                     ident.ident.into(),
        //                     EntityDefnVariant::EnumVariant {
        //                         enum_variant_defn_variant: match raw_variant_kind {
        //                             EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
        //                         },
        //                     },
        //                     db.subroute(ty_route, ident.ident, thin_vec![]),
        //                     file,
        //                     ast.range,
        //                 ))
        //                 .unwrap();
        //             children.next();
        //         }
        //         _ => break,
        //     }
        // }
        // Ok(variants)
    }

    pub fn method(&self, _member_idx: usize) -> &Arc<EntityDefn> {
        todo!()
    }

    fn visualizer_from_ast(
        _db: &dyn EntityDefnQueryGroup,
        _arena: &ExprArena,
        _file: PathItd,
        _ty_route: Ty,
        _children: &mut Peekable<AstIter>,
    ) -> Arc<Visualizer> {
        todo!()
        // let item = if let Some(_) = children.peek() {
        //     children.next().unwrap()
        // } else {
        //     return Visualizer::void();
        // };
        // let ref ast = item.value.as_ref().unwrap();
        // match ast.variant {
        //     AstVariant::Visual => {
        //         let stmts = parse_lazy_stmts(
        //             db.upcast(),
        //             arena,
        //             item.opt_children.clone().unwrap(),
        //             file,
        //             Ty {
        //                 route: RootBuiltinIdentifier::VisualType.into(),
        //                 range: Default::default(),
        //             },
        //         )
        //         .unwrap();
        //         Arc::new(Visualizer {
        //             visual_ty: VisualTy::from_stmts(db, &stmts),
        //             variant: VisualizerVariant::Custom { stmts },
        //         })
        //     }
        //     _ => Visualizer::void(),
        // }
    }
}

impl EntityDefn {
    // pub fn method(&self, member_idx: MemberIdx) -> &Arc<EntityDefn> {
    //     match self.variant {
    //         EntityDefnVariant::Ty { ref members, .. } => &members[member_idx.0 as usize],
    //         EntityDefnVariant::EnumVariant { .. } => todo!(),
    //         EntityDefnVariant::Builtin => todo!(),
    //         _ => panic!(),
    //     }
    // }
    pub fn field(&self, field_ident: CustomIdentifier) -> &Arc<EntityDefn> {
        match self.variant {
            EntityDefnVariant::Ty { ref ty_members, .. } => {
                ty_members.get_entry(field_ident).unwrap()
            }
            EntityDefnVariant::EnumVariant { .. } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            _ => panic!(),
        }
    }
}

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub enum MethodKind {
//     Func { stmts: Arc<Vec<Arc<FuncStmt>>> },
//     Proc { stmts: Arc<Vec<Arc<ProcStmt>>> },
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDefnVariant {
    Constant,
}

impl EntityDefnVariant {
    pub fn enum_variant(
        _db: &dyn EntityDefnQueryGroup,
        _ident: RangedCustomIdentifier,
        enum_variant_kind: EnumVariantKind,
        _children: Option<AstIter>,
    ) -> EntityDefnVariant {
        EntityDefnVariant::EnumVariant {
            enum_variant_defn_variant: match enum_variant_kind {
                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
            },
        }
    }
}
