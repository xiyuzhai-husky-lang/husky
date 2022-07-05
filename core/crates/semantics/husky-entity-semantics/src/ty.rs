mod member;
mod type_call;

use check_utils::should;
pub use member::*;
pub use type_call::*;

use super::*;
use husky_ast::*;
use husky_atom::{
    context::{AtomContextKind, Symbol},
    AtomContext,
};
use husky_eager_semantics::{FuncStmt, ProcStmt};
use husky_entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr};
use husky_file::FilePtr;
use husky_lazy_semantics::{LazyStmt, XmlExprVariant};
use husky_text::*;
use infer_decl::{DeclQueryGroup, MemberIdx};
use infer_total::InferQueryGroup;
use print_utils::{emsg_once, p};
use semantics_error::SemanticResult;
use std::{iter::Peekable, sync::Arc};
use vec_map::VecMap;
use word::{CustomIdentifier, IdentDict};

impl EntityDefnVariant {
    pub(crate) fn ty_from_ast(
        db: &dyn EntityDefnQueryGroup,
        ty: EntityRoutePtr,
        head: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        let (ident, kind, generic_parameters) = match head.variant {
            AstVariant::TypeDefnHead {
                ident,
                kind,
                spatial_parameters: ref generic_parameters,
            } => (ident, kind, generic_parameters.clone()),
            _ => panic!(),
        };
        let mut children = children.peekable();
        let mut ty_members = IdentDict::default();
        let mut trait_impls = Vec::new();
        emsg_once!("todo");

        let variants = match kind {
            TyKind::Enum => Self::collect_variants(db, file, ty, &mut children)?,
            _ => Default::default(),
        };

        Self::collect_original_fields(db, arena, file, ty, &mut children, &mut ty_members)?;

        let opt_type_call = match kind {
            TyKind::Enum => None,
            TyKind::Record => Some(Arc::new(TypeCallDefn {
                parameters: Arc::new(ty_members.map(|ty_member| match ty_member.variant {
                    EntityDefnVariant::TyField {
                        ty,
                        ref field_variant,
                        liason,
                        ..
                    } => match field_variant {
                        FieldDefnVariant::RecordOriginal => Parameter::from_member(
                            RangedCustomIdentifier {
                                ident: ty_member.ident.custom(),
                                range: Default::default(),
                            },
                            liason,
                            ty,
                            db.is_copyable(ty).unwrap(),
                        ),
                        _ => {
                            p!(field_variant);
                            panic!()
                        }
                    },
                    _ => panic!(),
                })),
                output_ty: RangedEntityRoute {
                    route: ty,
                    range: Default::default(),
                },
                opt_linkage: None,
            })),
            TyKind::Struct => Some(Arc::new(TypeCallDefn {
                parameters: Arc::new(ty_members.map(|ty_member| match ty_member.variant {
                    EntityDefnVariant::TyField {
                        ty,
                        ref field_variant,
                        liason,
                        ..
                    } => match field_variant {
                        FieldDefnVariant::StructOriginal => Parameter::from_member(
                            RangedCustomIdentifier {
                                ident: ty_member.ident.custom(),
                                range: Default::default(),
                            },
                            liason,
                            ty,
                            db.is_copyable(ty).unwrap(),
                        ),
                        _ => panic!(),
                    },
                    _ => panic!(),
                })),
                output_ty: RangedEntityRoute {
                    route: ty,
                    range: Default::default(),
                },
                opt_linkage: None,
            })),
            TyKind::Primitive => todo!(),
            TyKind::Vec => todo!(),
            TyKind::Array => todo!(),
            TyKind::Other => todo!(),
        };
        Self::collect_other_ty_members(db, arena, file, ty, &mut children, &mut ty_members)?;
        let opt_visual_stmts = Self::collect_visual_source(db, arena, file, ty, &mut children);
        Ok(EntityDefnVariant::new_ty(
            generic_parameters,
            ty_members,
            variants,
            kind,
            trait_impls,
            opt_type_call,
            None,
            opt_visual_stmts,
        ))
    }

    fn new_ty(
        generic_parameters: IdentDict<SpatialParameter>,
        ty_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
        opt_type_call: Option<Arc<TypeCallDefn>>,
        opt_static_visual_ty: Option<StaticVisualTy>,
        opt_visual_stmts: Option<Avec<LazyStmt>>,
    ) -> Self {
        let members = collect_all_members(&ty_members, &trait_impls);
        EntityDefnVariant::Ty {
            generic_parameters,
            ty_members,
            variants,
            kind,
            trait_impls,
            members,
            opt_type_call,
            opt_static_visual_ty,
            opt_visual_stmts,
        }
    }

    pub(crate) fn ty_from_static(
        symbol_context: &mut dyn AtomContext,
        static_defn: &EntityStaticDefn,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::Ty {
                base_route,
                spatial_parameters: generic_parameters,
                static_trait_impls: ref trait_impls,
                ref ty_members,
                ref variants,
                kind,
                visual_ty,
                opt_type_call,
            } => {
                let mut symbol_context = AtomContextStandalone {
                    opt_package_main: symbol_context.opt_package_main(),
                    db: symbol_context.entity_syntax_db(),
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: (&[] as &[Symbol]).into(),
                    kind: AtomContextKind::Normal,
                };
                let base_route = symbol_context.parse_entity_route(base_route).unwrap();
                let generic_parameters =
                    symbol_context.generic_parameters_from_static(generic_parameters);
                let generic_arguments =
                    symbol_context.generic_arguments_from_generic_parameters(&generic_parameters);
                let this_ty = symbol_context.db.intern_entity_route(EntityRoute {
                    kind: base_route.kind,
                    temporal_arguments: thin_vec![],
                    spatial_arguments: generic_arguments,
                });
                let symbols = symbol_context.symbols_from_generic_parameters(&generic_parameters);
                symbol_context.symbols = symbols.into();
                symbol_context.opt_this_ty = Some(this_ty);
                let ty_members = ty_members.map(|ty_member| {
                    let route = symbol_context.db.intern_entity_route(EntityRoute::subroute(
                        this_ty,
                        symbol_context.db.intern_word(ty_member.name).custom(),
                        thin_vec![],
                    ));
                    EntityDefn::from_static(&mut symbol_context, route, ty_member)
                });
                let variants = variants.map(|_| todo!());
                let kind = kind;
                let trait_impls = trait_impls
                    .map(|trait_impl| TraitImplDefn::from_static(&mut symbol_context, trait_impl));
                Self::new_ty(
                    generic_parameters,
                    ty_members,
                    variants,
                    kind,
                    trait_impls,
                    opt_type_call
                        .map(|type_call| TypeCallDefn::from_static(&mut symbol_context, type_call)),
                    Some(visual_ty),
                    None,
                )
            }
            _ => panic!(),
        }
    }

    fn collect_variants(
        db: &dyn EntityDefnQueryGroup,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        children: &mut Peekable<AstIter>,
    ) -> SemanticResult<IdentDict<Arc<EntityDefn>>> {
        let mut variants = VecMap::default();
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref().unwrap();
            match ast.variant {
                AstVariant::EnumVariantDefnHead {
                    ident,
                    variant_class: raw_variant_kind,
                } => {
                    variants.insert_new(EntityDefn::new(
                        ident.ident.into(),
                        EntityDefnVariant::EnumVariant {
                            ident,
                            variant: match raw_variant_kind {
                                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
                            },
                        },
                        db.make_subroute(ty_route, ident.ident, thin_vec![]),
                        file,
                        ast.range,
                    ));
                    children.next();
                }
                _ => break,
            }
        }
        Ok(variants)
    }

    fn record_from_ast(
        db: &dyn EntityDefnQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        todo!()
        // let mut fields = VecMap::default();
        // for subitem in children {
        //     match subitem.value.as_ref()?.kind {
        //         AstKind::Use { .. } => (),
        //         AstKind::RoutineDefnHead(_) => todo!(),
        //         AstKind::FieldDefn(ref field_defn) => fields.insert_new(field_defn.clone()),
        //         AstKind::MembFeatureDefnHead { ident, ty } => {
        //             let stmts = husky_lazy_semantics::parse_lazy_stmts(
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

    pub fn method(&self, member_idx: usize) -> &Arc<EntityDefn> {
        todo!()
        // match self.members[member_idx] {
        //     MemberDefn::TypeField(_) => todo!(),
        //     MemberDefn::TypeMethod(_) => todo!(),
        // }
    }

    fn collect_visual_source(
        db: &dyn EntityDefnQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        children: &mut Peekable<AstIter>,
    ) -> Option<Avec<LazyStmt>> {
        let item = if let Some(_) = children.peek() {
            children.next().unwrap()
        } else {
            return None;
        };
        let ref ast = item.value.as_ref().unwrap();
        match ast.variant {
            AstVariant::Visual => Some(
                parse_lazy_stmts(
                    db.upcast(),
                    arena,
                    item.opt_children.clone().unwrap(),
                    file,
                    RangedEntityRoute {
                        route: RootIdentifier::VisualType.into(),
                        range: Default::default(),
                    },
                )
                .unwrap(),
            ),
            _ => None,
        }
    }
}

impl EntityDefn {
    pub fn method(&self, member_idx: MemberIdx) -> &Arc<EntityDefn> {
        match self.variant {
            EntityDefnVariant::Ty { ref members, .. } => &members[member_idx.0 as usize],
            EntityDefnVariant::EnumVariant { ident, ref variant } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            _ => panic!(),
        }
    }
    pub fn field(&self, field_ident: CustomIdentifier) -> &Arc<EntityDefn> {
        match self.variant {
            EntityDefnVariant::Ty { ref ty_members, .. } => {
                ty_members.get_entry(field_ident).unwrap()
            }
            EntityDefnVariant::EnumVariant { ident, ref variant } => todo!(),
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
        db: &dyn EntityDefnQueryGroup,
        ident: RangedCustomIdentifier,
        enum_variant_kind: EnumVariantKind,
        children: Option<AstIter>,
    ) -> EntityDefnVariant {
        EntityDefnVariant::EnumVariant {
            ident,
            variant: match enum_variant_kind {
                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
            },
        }
    }
}
