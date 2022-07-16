mod enum_variant;
mod impl_instantiate;
mod trait_impl;
mod vec;

use std::iter::Peekable;

use check_utils::{should, should_eq};
use entity_kind::{EnumVariantKind, FieldKind};
use husky_liason_semantics::OutputLiason;
use print_utils::p;
pub use trait_impl::*;
pub use vec::*;

use crate::*;
use defn_head::*;
pub use enum_variant::*;
use fold::LocalStack;
use husky_ast::AstIter;
use husky_atom::{
    context::{AtomContextKind, Symbol, SymbolKind},
    AtomContext, AtomContextStandalone,
};
use husky_entity_route::*;
use husky_text::*;
use map_collect::MapCollect;
use thin_vec::{thin_vec, ThinVec};
use vec_like::VecMap;
use vm::TySignature;
use word::{IdentArcDict, IdentDict, Paradigm};

#[derive(Debug, PartialEq, Eq)]
pub struct TyDecl {
    pub this_ty: EntityRoutePtr,
    pub generic_parameters: IdentDict<SpatialParameter>,
    pub ty_members: IdentDict<TyMemberDecl>,
    pub variants: IdentDict<EnumVariantDecl>,
    pub kind: TyKind,
    pub trait_impls: Vec<Arc<TraitImplDecl>>,
    pub members: Vec<MemberDecl>,
    pub opt_type_call: Option<Arc<FunctionDecl>>,
}

impl TyDecl {
    fn from_static(db: &dyn DeclQueryGroup, static_defn: &EntityStaticDefn) -> Arc<Self> {
        match static_defn.variant {
            EntityStaticDefnVariant::Ty {
                base_route,
                spatial_parameters: generic_parameters,
                static_trait_impls,
                ty_members: type_members,
                variants,
                kind,
                opt_type_call,
                ..
            } => {
                let generic_parameters = db.generic_parameters_from_static(generic_parameters);
                let generic_arguments =
                    db.generic_arguments_from_generic_parameters(&generic_parameters);
                let symbols = db.symbols_from_generic_parameters(&generic_parameters);
                let mut symbol_context = AtomContextStandalone {
                    opt_package_main: None,
                    db: db.upcast(),
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: (&symbols as &[Symbol]).into(),
                    kind: AtomContextKind::Normal,
                };
                let base_ty = symbol_context.parse_entity_route(base_route).unwrap();
                let this_ty = db.intern_entity_route(EntityRoute {
                    kind: base_ty.kind,
                    temporal_arguments: thin_vec![],
                    spatial_arguments: generic_arguments,
                });
                symbol_context.opt_this_ty = Some(this_ty);
                let opt_type_call = opt_type_call.map(|type_call| {
                    routine_decl_from_static(db, symbols.clone(), this_ty, type_call)
                });
                let ty_members: IdentDict<_> = type_members
                    .iter()
                    .map(|member| TyMemberDecl::from_static(db, member, &mut symbol_context))
                    .collect();
                let variants: IdentDict<_> = variants.map(|static_decl| {
                    EnumVariantDecl::from_static(db, static_decl, &mut symbol_context)
                });
                let mut trait_impls =
                    TraitImplDecl::implicit_trait_impls(db, this_ty, kind, &ty_members, &variants)
                        .unwrap();
                trait_impls.extend(static_trait_impls.iter().map(|trait_impl| {
                    TraitImplDecl::from_static(db, trait_impl, &mut symbol_context)
                }));
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
            _ => panic!(""),
        }
    }

    fn from_ast(
        db: &dyn DeclQueryGroup,
        arena: &RawExprArena,
        ty: EntityRoutePtr,
        kind: TyKind,
        generic_parameters: IdentDict<SpatialParameter>,
        children: AstIter,
    ) -> InferQueryResultArc<Self> {
        let generic_arguments = db.generic_arguments_from_generic_parameters(&generic_parameters);
        let this_ty = db.intern_entity_route(EntityRoute {
            kind: ty.kind,
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
        Self::collect_visual(&mut children);
        if children.peek().is_some() {
            throw_derived!("expect no children after collecting_visual")
        }
        let mut trai_impls =
            TraitImplDecl::implicit_trait_impls(db, this_ty, kind, &ty_members, &variants)?;
        let opt_type_call = match kind {
            TyKind::Enum => None,
            TyKind::Record | TyKind::Struct => {
                let mut primary_parameters = IdentDict::default();
                let mut keyword_parameters = IdentDict::default();
                for ty_member in ty_members.iter() {
                    match ty_member {
                        TyMemberDecl::Field(ref field_decl) => match field_decl.field_kind {
                            FieldKind::StructOriginal | FieldKind::RecordOriginal => {
                                primary_parameters
                                    .insert(ParameterDecl::from_field(db, field_decl)?)
                            }
                            FieldKind::StructDefault => keyword_parameters
                                .insert(ParameterDecl::from_field(db, field_decl)?),
                            FieldKind::StructDerivedEager => break,
                            FieldKind::StructDerivedLazy => break,
                            FieldKind::RecordDerived => break,
                        },
                        TyMemberDecl::Method(_) | TyMemberDecl::Call(_) => break,
                    }
                }
                Some(Arc::new(FunctionDecl {
                    route: ty,
                    spatial_parameters: generic_parameters.clone(),
                    primary_parameters,
                    keyword_parameters,
                    output: OutputDecl {
                        ty,
                        liason: OutputLiason::Transfer,
                    },
                }))
            }
            TyKind::Primitive => todo!(),
            TyKind::Vec => panic!(),
            TyKind::Array => todo!(),
            TyKind::Other => todo!(),
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
                    variants.insert_new(EnumVariantDecl {
                        ident: ident.ident,
                        variant: match raw_variant_kind {
                            EnumVariantKind::Constant => EnumVariantDeclVariant::Constant,
                        },
                    });
                    children.next();
                }
                _ => panic!(),
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
                    field_ast_kind: field_kind,
                    ..
                } => {
                    match field_kind {
                        FieldAstKind::StructOriginal | FieldAstKind::RecordOriginal => (),
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
                        Paradigm::EagerProcedural => todo!(),
                        Paradigm::EagerFunctional => throw_query_derived!(members.insert_new(
                            TyMemberDecl::Method(MethodDecl::from_ast(ast, MethodKind::Type,))
                        )),
                        Paradigm::LazyFunctional => todo!(),
                    },
                    None => throw_query_derived!(members.insert_new(TyMemberDecl::Call(
                        FunctionDecl::from_ast(
                            db.make_subroute(this_ty, ident.ident, thin_vec![]),
                            ast,
                        )
                    ))),
                },
                AstVariant::Use { .. } => todo!(),
                AstVariant::FieldDefnHead {
                    field_ast_kind: field_kind,
                    ..
                } => match field_kind {
                    FieldAstKind::StructOriginal => todo!("no original at this point"),
                    FieldAstKind::RecordOriginal => todo!("no original at this point"),
                    _ => throw_query_derived!(
                        members.insert_new(TyMemberDecl::Field(FieldDecl::from_ast(ast)))
                    ),
                },
                AstVariant::Visual => break,
                AstVariant::TypeDefnHead { .. }
                | AstVariant::MainDefn
                | AstVariant::CallFormDefnHead { .. }
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
        opt_type_call: Option<Arc<FunctionDecl>>,
    ) -> Arc<Self> {
        let members = MemberDecl::collect_all(db, &ty_members, &trait_impls);
        Arc::new(Self {
            this_ty,
            generic_parameters,
            ty_members,
            variants,
            kind,
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
                FieldKind::StructOriginal
                | FieldKind::StructDefault
                | FieldKind::StructDerivedEager => Some(field_decl as &FieldDecl),
                FieldKind::StructDerivedLazy => None,
                FieldKind::RecordOriginal => todo!(),
                FieldKind::RecordDerived => todo!(),
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
                        "No such field `{}` in type `{:?}`",
                        &ranged_ident.ident, self.this_ty
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
            None => Err(derived!(format!("no such field"))),
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
        trait_uses: &[EntityRouteKind],
    ) -> InferResult<&Arc<MethodDecl>> {
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
        let matched_methods: Vec<&Arc<MethodDecl>> = self
            .members
            .iter()
            .enumerate()
            .filter_map(|(member_idx, member)| {
                if member.ident() == ranged_ident.ident {
                    match member {
                        MemberDecl::AssociatedType => todo!(),
                        MemberDecl::AssociatedCall => todo!(),
                        MemberDecl::TypeField(_) => todo!(),
                        MemberDecl::TypeMethod(_) => todo!(),
                        MemberDecl::TraitMethodImpl {
                            trait_route,
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
            // p!(self.this_ty);
            // p!(self.trait_impls);
            // p!(self.members);
            // println!(
            //     "no method named `{}` for type `{}`",
            //     &ranged_ident.ident, self.this_ty
            // );
            // panic!();
            throw!(
                format!(
                    "no method named `{}` for type `{}`",
                    &ranged_ident.ident, self.this_ty
                ),
                ranged_ident.range
            )
        } else {
            p!(self.this_ty, matched_methods);
            todo!()
        }
    }

    pub fn member_idx(&self, member_route: EntityRoutePtr) -> MemberIdx {
        match member_route.kind {
            EntityRouteKind::Child { parent, ident } => {
                should_eq!(self.this_ty, parent);
                self.ty_members.position(ident).unwrap().into()
            }
            EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                should_eq!(self.this_ty, ty);
                todo!()
            }
            _ => panic!(),
        }
    }

    pub fn trait_impl(&self, trai_route: EntityRoutePtr) -> Option<&Arc<TraitImplDecl>> {
        self.trait_impls
            .iter()
            .find(|trai_impl| trai_impl.trait_route == trai_route)
    }

    pub fn trai_member_impl(
        &self,
        trai: EntityRoutePtr,
        ident: CustomIdentifier,
    ) -> Option<&TraitMemberImplDecl> {
        self.trait_impl(trai)?.member(ident)
    }
}

pub(crate) fn ty_decl(
    db: &dyn DeclQueryGroup,
    ty_route: EntityRoutePtr,
) -> InferQueryResultArc<TyDecl> {
    let source = db.entity_locus(ty_route)?;
    match source {
        EntityLocus::StaticModuleItem(static_defn) => Ok(match static_defn.variant {
            EntityStaticDefnVariant::Function { .. } => todo!(),
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Ty { .. } => {
                let base_decl = TyDecl::from_static(db, static_defn);
                if ty_route.spatial_arguments.len() > 0 {
                    assert_eq!(
                        ty_route.spatial_arguments.len(),
                        base_decl.generic_parameters.len()
                    );
                    base_decl.instantiate(db, &ty_route.spatial_arguments)
                } else {
                    base_decl
                }
            }
            EntityStaticDefnVariant::Trait { .. } => todo!(),
            EntityStaticDefnVariant::Method { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
        }),
        EntityLocus::WithinBuiltinModule => todo!(),
        EntityLocus::WithinModule {
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
                    if ty_route.spatial_arguments.len() > 0 {
                        todo!()
                    } else {
                        TyDecl::from_ast(
                            db,
                            &ast_text.arena,
                            ty_route,
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
        EntityLocus::Module { file } => todo!(),
        EntityLocus::Input { .. } => todo!(),
        EntityLocus::StaticTypeMember => todo!(),
        EntityLocus::StaticTypeAsTraitMember => todo!(),
    }
}

fn is_trait_availabe(trait_route: EntityRoutePtr, trait_uses: &[EntityRouteKind]) -> bool {
    match trait_route.kind {
        EntityRouteKind::Root { ident } => true,
        EntityRouteKind::Package { main, ident } => todo!(),
        EntityRouteKind::Child { parent, ident } => todo!(),
        EntityRouteKind::Input { main } => todo!(),
        EntityRouteKind::Generic { ident, entity_kind } => todo!(),
        EntityRouteKind::ThisType => todo!(),
        EntityRouteKind::TypeAsTraitMember {
            ty: parent,
            trai,
            ident,
        } => todo!(),
    }
}

pub(crate) fn method_decl_from_static(
    db: &dyn DeclQueryGroup,
    mut symbols: Vec<Symbol>,
    static_defn: &EntityStaticDefn,
) -> Arc<MethodDecl> {
    match static_defn.variant {
        EntityStaticDefnVariant::Method {
            this_liason,
            parameters,
            output_ty,
            output_liason,
            spatial_parameters: generic_parameters,
            // ref kind,
            ..
        } => {
            let generic_parameters = db.generic_parameters_from_static(generic_parameters);
            symbols.extend(db.symbols_from_generic_parameters(&generic_parameters));
            let mut symbol_context = AtomContextStandalone {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: symbols.into(),
                kind: AtomContextKind::Normal,
            };
            let parameters = parameters.map(|parameter| ParameterDecl {
                ty: symbol_context.parse_entity_route(parameter.ty).unwrap(),
                liason: parameter.liason,
                ident: db.custom_ident(parameter.name),
            });
            let output_ty = symbol_context.parse_entity_route(output_ty).unwrap();
            // assert!(matches!(kind, MethodStaticDefnVariant::TypeMethod { .. }));
            Arc::new(MethodDecl {
                spatial_parameters: generic_parameters,
                parameters,
                output: OutputDecl {
                    liason: output_liason,
                    ty: output_ty,
                },
                this_liason,
                ident: db.intern_word(static_defn.name).custom(),
                kind: MethodKind::Type,
                is_lazy: false,
            })
        }
        _ => panic!(""),
    }
}
