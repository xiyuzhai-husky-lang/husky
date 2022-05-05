mod member;
mod type_call;

pub use member::*;
pub use type_call::*;

use super::*;
use ast::*;
use atom::{
    symbol::{Symbol, SymbolContextKind},
    SymbolContext,
};
use entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr};
use file::FilePtr;
use infer_decl::{DeclQueryGroup, MemberIdx};
use infer_total::InferQueryGroup;
use print_utils::{msg_once, p};
use semantics_eager::{FuncStmt, ProcStmt};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use std::{iter::Peekable, sync::Arc};
use vec_map::VecMap;
use word::{CustomIdentifier, IdentDict, RangedCustomIdentifier};

impl EntityDefnVariant {
    pub(crate) fn ty_from_ast(
        db: &dyn InferQueryGroup,
        ty: EntityRoutePtr,
        head: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        let (ident, kind, generic_placeholders) = match head.kind {
            AstKind::TypeDefnHead {
                ident,
                kind,
                ref generic_placeholders,
            } => (ident, kind, generic_placeholders.clone()),
            _ => panic!(),
        };
        let mut children = children.peekable();
        let mut ty_members = IdentDict::default();
        let mut trait_impls = Vec::new();
        msg_once!("todo");

        let variants = match kind {
            TyKind::Enum => Self::collect_variants(db, file, ty, &mut children)?,
            _ => Default::default(),
        };

        Self::collect_original_fields(db, arena, file, &mut children, &mut ty_members, ty)?;

        let opt_type_call = match kind {
            TyKind::Enum => None,
            TyKind::Record => Some(Arc::new(TyCallDefn {
                input_placeholders: Arc::new(ty_members.map(|ty_member| match ty_member.variant {
                    EntityDefnVariant::TypeField {
                        ty,
                        ref field_variant,
                        contract,
                    } => match field_variant {
                        FieldDefnVariant::RecordOriginal => InputPlaceholder {
                            ident,
                            contract: contract.constructor_input_contract(db.is_copyable(ty)),
                            ranged_ty: RangedEntityRoute {
                                route: ty,
                                range: Default::default(),
                            },
                        },
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
                source: TyCallSource::GenericRecord,
            })),
            TyKind::Struct => Some(Arc::new(TyCallDefn {
                input_placeholders: Arc::new(ty_members.map(|ty_member| match ty_member.variant {
                    EntityDefnVariant::TypeField {
                        ty,
                        ref field_variant,
                        contract,
                    } => match field_variant {
                        FieldDefnVariant::StructOriginal => InputPlaceholder {
                            ident,
                            contract: contract.constructor_input_contract(db.is_copyable(ty)),
                            ranged_ty: RangedEntityRoute {
                                route: ty,
                                range: Default::default(),
                            },
                        },
                        _ => panic!(),
                    },
                    _ => panic!(),
                })),
                output_ty: RangedEntityRoute {
                    route: ty,
                    range: Default::default(),
                },
                source: TyCallSource::GenericStruct,
            })),
            TyKind::Primitive => todo!(),
            TyKind::Vec => todo!(),
            TyKind::Array => todo!(),
            TyKind::Other => todo!(),
        };

        Self::collect_other_members(db, arena, file, ty, children, &mut ty_members)?;
        Ok(EntityDefnVariant::new_ty(
            generic_placeholders,
            ty_members,
            variants,
            kind,
            trait_impls,
            opt_type_call,
        ))
    }

    fn new_ty(
        generic_placeholders: IdentDict<GenericPlaceholder>,
        type_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
        opt_type_call: Option<Arc<TyCallDefn>>,
    ) -> Self {
        let members = collect_all_members(&type_members, &trait_impls);
        EntityDefnVariant::Type {
            generic_placeholders,
            ty_members: type_members,
            variants,
            kind,
            trait_impls,
            members,
            opt_type_call,
        }
    }

    pub(crate) fn ty_from_static(
        symbol_context: &SymbolContext,
        static_defn: &EntityStaticDefn,
    ) -> Self {
        match static_defn.variant {
            EntityStaticDefnVariant::Type {
                base_route,
                generic_placeholders,
                ref trait_impls,
                ref type_members,
                ref variants,
                kind,
                visualizer,
                opt_type_call,
            } => {
                let mut symbol_context = SymbolContext {
                    opt_package_main: symbol_context.opt_package_main,
                    db: symbol_context.db,
                    opt_this_ty: None,
                    symbols: (&[] as &[Symbol]).into(),
                    kind: SymbolContextKind::Normal,
                };
                let base_route = symbol_context.entity_route_from_str(base_route).unwrap();
                let generic_placeholders =
                    symbol_context.generic_placeholders_from_static(generic_placeholders);
                let generic_arguments = symbol_context
                    .generic_arguments_from_generic_placeholders(&generic_placeholders);
                let this_ty = symbol_context.db.intern_entity_route(EntityRoute {
                    kind: base_route.kind,
                    generic_arguments,
                });
                let symbols =
                    symbol_context.symbols_from_generic_placeholders(&generic_placeholders);
                symbol_context.symbols = symbols.into();
                symbol_context.opt_this_ty = Some(this_ty);
                let type_members = type_members.map(|type_member| {
                    EntityDefn::from_static(
                        &symbol_context,
                        symbol_context
                            .db
                            .intern_entity_route(EntityRoute::child_route(
                                this_ty,
                                symbol_context.db.intern_word(type_member.name).custom(),
                                vec![],
                            )),
                        type_member,
                    )
                });
                let variants = variants.map(|_| todo!());
                let kind = kind;
                let trait_impls = trait_impls
                    .map(|trait_impl| TraitImplDefn::from_static(&symbol_context, trait_impl));
                Self::new_ty(
                    generic_placeholders,
                    type_members,
                    variants,
                    kind,
                    trait_impls,
                    opt_type_call
                        .map(|type_call| TyCallDefn::from_static(&symbol_context, type_call)),
                )
            }
            _ => panic!(),
        }
    }

    fn collect_variants(
        db: &dyn InferQueryGroup,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        children: &mut Peekable<AstIter>,
    ) -> SemanticResult<IdentDict<Arc<EntityDefn>>> {
        let mut variants = VecMap::default();
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref()?;
            match ast.kind {
                AstKind::EnumVariantDefnHead {
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
                        db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child {
                                parent: ty_route,
                                ident: ident.ident,
                            },
                            generic_arguments: vec![],
                        }),
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
        db: &dyn InferQueryGroup,
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

    pub fn method(&self, member_idx: usize) -> &Arc<EntityDefn> {
        todo!()
        // match self.members[member_idx] {
        //     MemberDefn::TypeField(_) => todo!(),
        //     MemberDefn::TypeMethod(_) => todo!(),
        // }
    }
}

impl EntityDefn {
    pub fn method(&self, member_idx: MemberIdx) -> &Arc<EntityDefn> {
        match self.variant {
            EntityDefnVariant::Type { ref members, .. } => &members[member_idx.0 as usize],
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
