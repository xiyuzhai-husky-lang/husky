mod enum_variant;
mod impl_instantiate;
mod trait_impl;
mod vec;

use std::iter::Peekable;

use husky_check_utils::should_eq;
use husky_entity_kind::{EnumVariantKind, FieldKind};
use husky_liason_semantics::OutputModifier;
use husky_print_utils::{msg_once, p};
use itertools::Itertools;
pub use trait_impl::*;
pub use vec::*;

use crate::*;
pub use enum_variant::*;
use husky_ast::AstIter;
use husky_atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use husky_defn_head::*;
use husky_entity_route::*;
use husky_text::*;
use husky_word::{IdentDict, Paradigm};
use map_collect::MapCollect;
use thin_vec::thin_vec;
use vec_like::VecMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TyDecl {
    pub this_ty: EntityRoutePtr,
    pub spatial_parameters: IdentDict<SpatialParameter>,
    pub ty_members: IdentDict<TyMemberDecl>,
    pub variants: IdentDict<EnumVariantDecl>,
    pub ty_kind: TyKind,
    pub trait_impls: Vec<Arc<TraitImplDecl>>,
    pub members: Vec<MemberDecl>,
    pub opt_type_call: Option<Arc<CallFormDecl>>,
}

impl TyDecl {
    fn from_static(
        db: &dyn DeclQueryGroup,
        static_defn: &EntityStaticDefn,
    ) -> InferResultArc<Self> {
        Ok(match static_defn.variant {
            EntityStaticDefnVariant::Ty {
                base_route,
                spatial_parameters: generic_parameters,
                trait_impls: static_trait_impls,
                ty_members: type_members,
                variants,
                kind,
                opt_type_call,
                ..
            } => {
                let generic_parameters = db.spatial_parameters_from_static(generic_parameters);
                let generic_arguments =
                    db.spatial_arguments_from_spatial_parameters(&generic_parameters);
                let symbols = db.symbols_from_spatial_parameters(&generic_parameters);
                let mut symbol_context = AtomContextStandalone {
                    db: db.upcast(),
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: (&symbols as &[Symbol]).into(),
                    kind: AtomContextKind::Normal,
                    opt_file: Some(db.intern_file(static_defn.dev_src.file.into())),
                };
                let base_ty = symbol_context.parse_entity_route(base_route).unwrap();
                let this_ty = db.intern_entity_route(EntityRoute {
                    variant: base_ty.variant.clone(),
                    temporal_arguments: thin_vec![],
                    spatial_arguments: generic_arguments,
                });
                symbol_context.opt_this_ty = Some(this_ty);
                let opt_type_call = if let Some(type_call) = opt_type_call {
                    Some(routine_decl_from_static(
                        db,
                        symbols.clone(),
                        this_ty,
                        type_call,
                    )?)
                } else {
                    None
                };
                let ty_members: IdentDict<_> = type_members
                    .iter()
                    .map(|member| {
                        TyMemberDecl::from_static(
                            db,
                            &mut symbol_context,
                            db.subroute(this_ty, db.it_word(member.name).custom(), thin_vec![]),
                            member,
                        )
                    })
                    .collect::<InferResult<_>>()?;
                let variants: IdentDict<_> = variants.map(|static_decl| {
                    EnumVariantDecl::from_static(db, static_decl, &mut symbol_context)
                });
                let trait_impls = static_trait_impls
                    .iter()
                    .map(|trait_impl| {
                        TraitImplDecl::from_static(db, trait_impl, &mut symbol_context)
                    })
                    .collect_vec();
                Self::new(
                    db,
                    this_ty,
                    generic_parameters,
                    ty_members,
                    variants,
                    kind,
                    trait_impls,
                    opt_type_call,
                )
            }
            _ => unreachable!(),
        })
    }

    fn from_ast(
        db: &dyn DeclQueryGroup,
        ty: EntityRoutePtr,
        kind: TyKind,
        generic_parameters: IdentDict<SpatialParameter>,
        children: AstIter,
    ) -> InferQueryResultArc<Self> {
        let generic_arguments = db.spatial_arguments_from_spatial_parameters(&generic_parameters);
        let this_ty = db.intern_entity_route(EntityRoute {
            variant: ty.variant.clone(),
            temporal_arguments: thin_vec![],
            spatial_arguments: generic_arguments,
        });
        let mut children = children.peekable();
        let mut ty_members = IdentDict::default();
        let variants = match kind {
            TyKind::Enum => Self::collect_variants(&mut children)?,
            _ => Default::default(),
        };
        Self::collect_original_fields(&mut children, &mut ty_members)?;
        Self::collect_other_ty_members(db, this_ty, &mut children, &mut ty_members)?;
        Self::collect_visual(&mut children)?;
        if children.peek().is_some() {
            throw_derived!("expect no children after collecting_visual")
        }
        let trai_impls =
            TraitImplDecl::implicit_trait_impls(db, this_ty, kind, &ty_members, &variants)?;
        let opt_type_call = match kind {
            TyKind::Enum => None,
            TyKind::Record | TyKind::Struct => {
                let mut primary_parameters = IdentDict::default();
                let mut keyword_parameters = IdentDict::default();
                for ty_member in ty_members.iter() {
                    match ty_member {
                        TyMemberDecl::Field(ref field_decl) => match field_decl.field_kind {
                            FieldKind::StructRegular | FieldKind::RecordRegular => {
                                primary_parameters
                                    .insert(ParameterDecl::from_field(db, field_decl)?)
                            }
                            FieldKind::StructDefault => keyword_parameters
                                .insert(ParameterDecl::from_field(db, field_decl)?),
                            FieldKind::StructDerived => break,
                            FieldKind::StructMemo => break,
                            FieldKind::RecordProperty => break,
                        },
                        TyMemberDecl::Method(_) | TyMemberDecl::Call(_) => break,
                    }
                }
                msg_once!("variadics");
                Some(Arc::new(CallFormDecl {
                    opt_route: Some(ty),
                    spatial_parameters: generic_parameters.clone(),
                    primary_parameters,
                    variadic_parameters: VariadicParametersDecl::None,
                    keyword_parameters,
                    output: OutputDecl::new(db, OutputModifier::Transfer, ty)?,
                    opt_this_liason: None,
                    is_lazy: match kind {
                        TyKind::Record => true,
                        TyKind::Struct => false,
                        _ => panic!(),
                    },
                }))
            }
            TyKind::Primitive => todo!(),
            TyKind::Vec => panic!(),
            TyKind::Array => todo!(),
            TyKind::Slice => todo!(),
            TyKind::CyclicSlice => todo!(),
            TyKind::Tuple => todo!(),
            TyKind::Mor => todo!(),
            TyKind::ThickFp => todo!(),
            TyKind::AssociatedAny => todo!(),
            TyKind::TargetOutputAny => todo!(),
            TyKind::ThisAny => todo!(),
            TyKind::SpatialPlaceholderAny => todo!(),
            TyKind::BoxAny => todo!(),
            TyKind::HigherKind => todo!(),
            TyKind::Ref => todo!(),
            TyKind::Option => todo!(),
        };
        Ok(TyDecl::new(
            db,
            this_ty,
            generic_parameters,
            ty_members,
            variants,
            kind,
            trai_impls,
            opt_type_call,
        ))
    }

    fn collect_variants(
        children: &mut Peekable<AstIter>,
    ) -> InferQueryResult<IdentDict<EnumVariantDecl>> {
        let mut variants = VecMap::default();
        while let Some(child) = children.peek() {
            match child.value.as_ref()?.variant {
                AstVariant::EnumVariantDefnHead {
                    ident,
                    variant_class: ref raw_variant_kind,
                } => {
                    variants
                        .insert_new(EnumVariantDecl {
                            ident: ident.ident,
                            variant: match raw_variant_kind {
                                EnumVariantKind::Constant => EnumVariantDeclVariant::Constant,
                            },
                        })
                        .unwrap();
                    children.next();
                }
                _ => unreachable!(),
            }
        }
        Ok(variants)
    }

    fn collect_original_fields(
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<TyMemberDecl>,
    ) -> InferQueryResult<()> {
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref()?;
            match ast.variant {
                AstVariant::FieldDefnHead {
                    ast_field_kind: field_kind,
                    ..
                } => {
                    match field_kind {
                        AstFieldKind::StructOriginal | AstFieldKind::RecordOriginal => (),
                        _ => break,
                    }
                    children.next();
                    throw_query_derived!(
                        members.insert_new(TyMemberDecl::Field(FieldDecl::from_ast(ast)))
                    )
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn collect_other_ty_members(
        db: &dyn DeclQueryGroup,
        this_ty: EntityRoutePtr,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<TyMemberDecl>,
    ) -> InferQueryResult<()> {
        while let Some(child) = children.peek() {
            let ast = &child.value.as_ref()?;
            match ast.variant {
                AstVariant::CallFormDefnHead {
                    opt_this_liason,
                    paradigm,
                    ident,
                    ..
                } => match opt_this_liason {
                    Some(_) => match paradigm {
                        Paradigm::EagerProcedural | Paradigm::EagerFunctional => {
                            throw_query_derived!(members.insert_new(TyMemberDecl::Method(
                                CallFormDecl::from_ast(
                                    db,
                                    db.subroute(this_ty, ident.ident, thin_vec![]),
                                    ast,
                                )?
                            )))
                        }
                        Paradigm::LazyFunctional => todo!(),
                    },
                    None => throw_query_derived!(members.insert_new(TyMemberDecl::Call(
                        CallFormDecl::from_ast(
                            db,
                            db.subroute(this_ty, ident.ident, thin_vec![]),
                            ast,
                        )?
                    ))),
                },
                AstVariant::Use { .. } => todo!(),
                AstVariant::FieldDefnHead {
                    ast_field_kind: field_kind,
                    ..
                } => match field_kind {
                    AstFieldKind::StructOriginal => todo!("no original at this point"),
                    AstFieldKind::RecordOriginal => todo!("no original at this point"),
                    _ => throw_query_derived!(
                        members.insert_new(TyMemberDecl::Field(FieldDecl::from_ast(ast)))
                    ),
                },
                AstVariant::Visual => break,
                AstVariant::TypeDefnHead { .. }
                | AstVariant::MainDefnHead
                | AstVariant::FeatureDefnHead { .. }
                | AstVariant::DatasetConfigDefnHead
                | AstVariant::Stmt(_)
                | AstVariant::EnumVariantDefnHead { .. }
                | AstVariant::Submodule { .. } => todo!(),
            }
            children.next();
        }
        Ok(())
    }

    fn collect_visual(children: &mut Peekable<AstIter>) -> InferQueryResult<()> {
        if let Some(child) = children.peek() {
            match child.value.as_ref()?.variant {
                AstVariant::Visual => {
                    children.next();
                }
                _ => (),
            }
        }
        Ok(())
    }

    pub(crate) fn new(
        db: &dyn DeclQueryGroup,
        this_ty: EntityRoutePtr,
        generic_parameters: IdentDict<SpatialParameter>,
        ty_members: IdentDict<TyMemberDecl>,
        variants: IdentDict<EnumVariantDecl>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDecl>>,
        opt_type_call: Option<Arc<CallFormDecl>>,
    ) -> Arc<Self> {
        let members = MemberDecl::collect_all(db, &ty_members, &trait_impls);
        Arc::new(Self {
            this_ty,
            spatial_parameters: generic_parameters,
            ty_members,
            variants,
            ty_kind: kind,
            trait_impls,
            members,
            opt_type_call,
        })
    }

    pub fn field_idx(&self, field_ident: CustomIdentifier) -> usize {
        self.ty_members.position(field_ident).unwrap()
    }

    pub fn eager_fields(&self) -> impl Iterator<Item = &FieldDecl> {
        self.ty_members.iter().filter_map(|member| match member {
            TyMemberDecl::Field(field_decl) => match field_decl.field_kind {
                FieldKind::StructRegular | FieldKind::StructDefault | FieldKind::StructDerived => {
                    Some(field_decl as &FieldDecl)
                }
                FieldKind::StructMemo => None,
                FieldKind::RecordRegular => todo!(),
                FieldKind::RecordProperty => todo!(),
            },
            _ => None,
        })
    }

    pub fn field_ty_result(
        &self,
        ranged_ident: RangedCustomIdentifier,
    ) -> InferResult<EntityRoutePtr> {
        match self.ty_members.get_entry(ranged_ident.ident) {
            Some(type_member_decl) => match type_member_decl {
                TyMemberDecl::Field(field) => Ok(field.ty),
                TyMemberDecl::Method(_) => throw!(
                    format!(
                        "expect a field, but `{}` is a method in type `{:?}`",
                        &ranged_ident.ident, self.this_ty
                    ),
                    ranged_ident.range
                ),
                TyMemberDecl::Call(_) => throw!(
                    format!(
                        "expect a field, but `{}` is an associated call in type `{:?}`",
                        &ranged_ident.ident, self.this_ty
                    ),
                    ranged_ident.range
                ),
            },
            None => {
                throw!(
                    format!(
                        "no field `{}` in type `{}`",
                        &ranged_ident.ident,
                        self.this_ty.ident()
                    ),
                    ranged_ident.range
                )
            }
        }
    }

    pub fn field_decl(&self, ranged_ident: RangedCustomIdentifier) -> InferResultArcRef<FieldDecl> {
        match self.ty_members.get_entry(ranged_ident.ident) {
            Some(member_decl) => match member_decl {
                TyMemberDecl::Field(field) => Ok(field),
                TyMemberDecl::Method(_) => {
                    Err(derived!(format!("expect a field, but got method instead")))
                }
                TyMemberDecl::Call(_) => todo!(),
            },
            None => Err(derived!(format!(
                "no field `{}` in type `{}`",
                &ranged_ident.ident,
                self.this_ty.ident()
            ))),
        }
    }

    pub fn field_kind(&self, field_ident: CustomIdentifier) -> FieldKind {
        match self.ty_members.get_entry(field_ident).unwrap() {
            TyMemberDecl::Field(field) => field.field_kind,
            _ => panic!(""),
        }
    }

    pub fn method(
        &self,
        ranged_ident: RangedCustomIdentifier,
        trait_uses: &[EntityRouteVariant],
    ) -> InferResult<&Arc<CallFormDecl>> {
        // the rule is:
        // first look in the type members,
        // then look in the trait members,
        // if multiple are found in the trait members,
        // report an infer error.
        if let Some(member) = self.ty_members.get_entry(ranged_ident.ident) {
            match member {
                TyMemberDecl::Field(_) => throw!(
                    format!(
                        "`{}` is a field not a method in type `{:?}`",
                        &ranged_ident.ident, self.this_ty
                    ),
                    ranged_ident.range
                ),
                TyMemberDecl::Method(method) => return Ok(method),
                TyMemberDecl::Call(_) => todo!(),
            }
        }
        let matched_methods: Vec<&Arc<CallFormDecl>> = self
            .members
            .iter()
            .filter_map(|member| {
                if member.ident() == ranged_ident.ident {
                    match member {
                        MemberDecl::AssociatedType => todo!(),
                        MemberDecl::AssociatedCall => todo!(),
                        MemberDecl::TypeField(_) => todo!(),
                        MemberDecl::TypeMethod(_) => todo!(),
                        MemberDecl::TraitMethodImpl {
                            trai: trait_route,
                            method,
                        } => {
                            if is_trait_availabe(*trait_route, trait_uses) {
                                Some(method)
                            } else {
                                None
                            }
                        }
                        MemberDecl::TraitAssociatedTypeImpl { .. } => todo!(),
                        MemberDecl::TraitAssociatedConstSizeImpl { .. } => todo!(),
                        MemberDecl::TypeAssociatedCall(_) => todo!(),
                    }
                } else {
                    None
                }
            })
            .collect();
        if matched_methods.len() == 1 {
            return Ok(matched_methods[0]);
        } else if matched_methods.len() == 0 {
            throw!(
                format!(
                    "no method named `{}` for type `{}`, available members are `{:?}`",
                    &ranged_ident.ident,
                    self.this_ty,
                    self.members
                        .iter()
                        .map(|member| member.ident())
                        .collect::<Vec<_>>()
                ),
                ranged_ident.range
            )
        } else {
            p!(self.this_ty, matched_methods);
            todo!()
        }
    }

    pub fn member_idx(&self, member_route: EntityRoutePtr) -> MemberIdx {
        match member_route.variant {
            EntityRouteVariant::Child { parent, ident } => {
                should_eq!(self.this_ty, parent);
                self.ty_members.position(ident).unwrap().into()
            }
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => {
                should_eq!(self.this_ty, ty);
                self.members
                    .iter()
                    .position(|member| match member {
                        MemberDecl::TraitMethodImpl {
                            trai: trait_route,
                            method,
                        } => *trait_route == trai && method.ident() == ident,
                        _ => false,
                    })
                    .unwrap()
                    .into()
            }
            _ => unreachable!(),
        }
    }

    pub fn trait_impl(&self, trai: EntityRoutePtr) -> Option<&Arc<TraitImplDecl>> {
        self.trait_impls
            .iter()
            .find(|trai_impl| trai_impl.trai() == trai)
    }

    pub fn trai_member_impl(
        &self,
        trai: EntityRoutePtr,
        ident: CustomIdentifier,
    ) -> Option<&TraitMemberImplDecl> {
        self.trait_impl(trai)?.member(ident)
    }
}

pub(crate) fn ty_decl(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> InferQueryResultArc<TyDecl> {
    assert!(ty.is_intrinsic());
    let source = db.entity_source(ty)?;
    match source {
        EntitySource::StaticModuleItem(static_defn) => Ok(match static_defn.variant {
            EntityStaticDefnVariant::Function { .. } => todo!(),
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Ty { .. } => {
                let base_decl = TyDecl::from_static(db, static_defn)?;
                if ty.spatial_arguments.len() > 0 {
                    base_decl.instantiate(db, &ty.spatial_arguments)
                } else {
                    base_decl
                }
            }
            EntityStaticDefnVariant::Trait { .. } => todo!(),
            EntityStaticDefnVariant::Method { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
            EntityStaticDefnVariant::EnumVariant => todo!(),
        }),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.variant {
                AstVariant::TypeDefnHead {
                    kind,
                    spatial_parameters: ref generic_parameters,
                    ..
                } => {
                    if ty.spatial_arguments.len() > 0 {
                        todo!()
                    } else {
                        TyDecl::from_ast(
                            db,
                            ty,
                            kind,
                            generic_parameters.clone(),
                            query_derived_not_none!(item.opt_children)?,
                        )
                    }
                }
                _ => {
                    p!(ast);
                    panic!()
                }
            }
        }
        EntitySource::Module { .. } => todo!(),
        EntitySource::TargetInput { .. } => todo!(),
        EntitySource::StaticTypeMember(_) => todo!(),
        EntitySource::StaticTraitMember(_) => todo!(),
        EntitySource::StaticTypeAsTraitMember => todo!(),
        EntitySource::Any { .. } => todo!(),
        EntitySource::StaticEnumVariant(_) => todo!(),
        EntitySource::ThisType { .. } => todo!(),
    }
}

fn is_trait_availabe(trait_route: EntityRoutePtr, trait_uses: &[EntityRouteVariant]) -> bool {
    match trait_route.variant {
        EntityRouteVariant::Root { .. } => true,
        EntityRouteVariant::Package { .. } => todo!(),
        EntityRouteVariant::Child { .. } => todo!(),
        EntityRouteVariant::TargetInputValue => todo!(),
        EntityRouteVariant::Any { .. } => todo!(),
        EntityRouteVariant::ThisType { .. } => todo!(),
        EntityRouteVariant::TypeAsTraitMember { .. } => todo!(),
        EntityRouteVariant::TargetOutputType => todo!(),
    }
}
