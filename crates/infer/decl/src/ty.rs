mod enum_variant;
mod impl_instantiate;
mod trait_impl;
mod vec;

use std::iter::Peekable;

use check_utils::should_eq;
use entity_kind::{EnumVariantKind, FieldKind, RoutineContextKind};
use print_utils::p;
pub use trait_impl::*;
pub use vec::*;

use crate::*;
use ast::AstIter;
use atom::{
    symbol::{Symbol, SymbolContextKind, SymbolKind},
    SymbolContext,
};
use defn_head::*;
use entity_route::*;
pub use enum_variant::*;
use fold::LocalStack;
use map_collect::MapCollect;
use text::*;
use vec_map::VecMap;
use vm::{OutputContract, TySignature};
use word::{IdentArcDict, IdentDict};

#[derive(Debug, PartialEq, Eq)]
pub struct TyDecl {
    pub this_ty: EntityRoutePtr,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub ty_members: IdentDict<TyMemberDecl>,
    pub variants: IdentDict<EnumVariantDecl>,
    pub kind: TyKind,
    pub trai_impls: Vec<Arc<TraitImplDecl>>,
    pub members: Vec<MemberDecl>,
    pub opt_type_call: Option<Arc<CallDecl>>,
}

impl TyDecl {
    fn from_static(db: &dyn DeclQueryGroup, static_defn: &EntityStaticDefn) -> Arc<Self> {
        match static_defn.variant {
            EntityStaticDefnVariant::Type {
                base_route,
                generic_placeholders,
                trait_impls,
                type_members,
                variants,
                kind,
                opt_type_call,
                ..
            } => {
                let generic_placeholders =
                    db.generic_placeholders_from_static(generic_placeholders);
                let generic_arguments =
                    db.generic_arguments_from_generic_placeholders(&generic_placeholders);
                let symbols = db.symbols_from_generic_placeholders(&generic_placeholders);
                let mut symbol_context = SymbolContext {
                    opt_package_main: None,
                    db: db.upcast(),
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: (&symbols as &[Symbol]).into(),
                    kind: SymbolContextKind::Normal,
                };
                let base_ty = symbol_context.entity_route_from_str(base_route).unwrap();
                let this_ty = db.intern_entity_route(EntityRoute {
                    kind: base_ty.kind,
                    generic_arguments,
                });
                symbol_context.opt_this_ty = Some(this_ty);
                let opt_type_call = opt_type_call
                    .map(|type_call| routine_decl_from_static(db, symbols.clone(), type_call));
                let trait_impls = trait_impls
                    .map(|trait_impl| TraitImplDecl::from_static(db, trait_impl, &symbol_context));
                Self::new(
                    db,
                    this_ty,
                    generic_placeholders,
                    type_members
                        .iter()
                        .map(|member| TyMemberDecl::from_static(db, member, &symbol_context))
                        .collect(),
                    variants.map(|static_decl| {
                        EnumVariantDecl::from_static(db, static_decl, &symbol_context)
                    }),
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
        generic_placeholders: IdentDict<GenericPlaceholder>,
        children: AstIter,
    ) -> InferResultArc<Self> {
        let generic_arguments =
            db.generic_arguments_from_generic_placeholders(&generic_placeholders);
        let this_ty = db.intern_entity_route(EntityRoute {
            kind: ty.kind,
            generic_arguments,
        });
        let mut children = children.peekable();
        let mut ty_members = IdentDict::default();
        let mut trai_impls = Vec::default();
        let variants = match kind {
            TyKind::Enum => Self::collect_variants(&mut children)?,
            _ => Default::default(),
        };
        Self::collect_original_fields(&mut children, &mut ty_members)?;
        Self::collect_other_members(children, &mut ty_members)?;
        let opt_type_call = match kind {
            TyKind::Enum => None,
            TyKind::Record | TyKind::Struct => {
                let mut inputs = vec![];
                for ty_member in ty_members.iter() {
                    match ty_member {
                        TyMemberDecl::Field(ref field_decl) => match field_decl.kind {
                            FieldKind::StructOriginal | FieldKind::RecordOriginal => {
                                inputs.push(InputDecl {
                                    contract: field_decl
                                        .contract
                                        .constructor_input_contract(db.is_copyable(field_decl.ty)),
                                    ty: field_decl.ty,
                                    ident: field_decl.ident,
                                })
                            }
                            FieldKind::StructDerived | FieldKind::RecordDerived => break,
                        },
                        TyMemberDecl::Method(_) | TyMemberDecl::Call => break,
                    }
                }
                Some(Arc::new(CallDecl {
                    inputs,
                    output: OutputDecl {
                        ty,
                        contract: OutputContract::Transfer,
                    },
                    generic_placeholders: generic_placeholders.clone(),
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
            generic_placeholders,
            ty_members,
            variants,
            kind,
            trai_impls,
            opt_type_call,
        ))
    }

    fn collect_variants(
        children: &mut Peekable<AstIter>,
    ) -> InferResult<IdentDict<EnumVariantDecl>> {
        let mut variants = VecMap::default();
        while let Some(child) = children.peek() {
            match child.value.as_ref()?.kind {
                AstKind::EnumVariantDefnHead {
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
    ) -> InferResult<()> {
        while let Some(child) = children.peek() {
            match child.value.as_ref()?.kind {
                AstKind::FieldDefnHead(ref field_defn_head) => {
                    match field_defn_head.kind {
                        FieldKind::StructOriginal | FieldKind::RecordOriginal => (),
                        FieldKind::StructDerived | FieldKind::RecordDerived => break,
                    }
                    children.next();
                    members.insert_new(TyMemberDecl::Field(FieldDecl::from_ast(field_defn_head)))
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn collect_other_members(
        mut children: Peekable<AstIter>,
        members: &mut IdentDict<TyMemberDecl>,
    ) -> InferResult<()> {
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
                AstKind::TypeMethodDefnHead(ref method_defn_head) => {
                    match method_defn_head.routine_kind {
                        RoutineContextKind::Proc => todo!(),
                        RoutineContextKind::Func => members.insert_new(TyMemberDecl::Method(
                            MethodDecl::from_ast(method_defn_head, MethodKind::Type),
                        )),
                        RoutineContextKind::Test => todo!(),
                    }
                }
                AstKind::Use { .. } => todo!(),
                AstKind::FieldDefnHead(ref field_defn_head) => match field_defn_head.kind {
                    FieldKind::StructOriginal => todo!("no original at this point"),
                    FieldKind::RecordOriginal => todo!("no original at this point"),
                    FieldKind::StructDerived | FieldKind::RecordDerived => members
                        .insert_new(TyMemberDecl::Field(FieldDecl::from_ast(field_defn_head))),
                },
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::Stmt(_) => todo!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => todo!(),
                AstKind::Submodule { ident, source_file } => todo!(),
            }
        }
        Ok(())
    }

    pub(crate) fn new(
        db: &dyn DeclQueryGroup,
        this_ty: EntityRoutePtr,
        generic_placeholders: IdentDict<GenericPlaceholder>,
        type_members: IdentDict<TyMemberDecl>,
        variants: IdentDict<EnumVariantDecl>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDecl>>,
        opt_type_call: Option<Arc<CallDecl>>,
    ) -> Arc<Self> {
        let members = MemberDecl::collect_all(db, &type_members, &trait_impls);
        Arc::new(Self {
            this_ty,
            generic_placeholders,
            ty_members: type_members,
            variants,
            kind,
            trai_impls: trait_impls,
            members,
            opt_type_call,
        })
    }

    pub fn field_idx(&self, field_ident: CustomIdentifier) -> usize {
        self.ty_members.position(field_ident).unwrap()
    }

    pub fn fields(&self) -> impl Iterator<Item = &FieldDecl> {
        self.ty_members.iter().filter_map(|member| match member {
            TyMemberDecl::Field(field_decl) => Some(field_decl as &FieldDecl),
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
                TyMemberDecl::Call => todo!(),
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
        // match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         ..
        //     } => ok_or!(
        //         field_vars.get(ranged_ident.ident),
        //         format!("no such member variable {}", &ranged_ident.ident),
        //         ranged_ident.range
        //     )
        //     .map(|signature| signature.ty),
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => {
        //         if let Some(field_var) = field_vars.get(ranged_ident.ident) {
        //             Ok(field_var.ty)
        //         } else if let Some(field_feature) = field_features.get(ranged_ident.ident) {
        //             Ok(*field_feature)
        //         } else {
        //             todo!()
        //         }
        //     }
        //     TyKind::Vec { element_ty } => todo!(),
        // }
    }

    pub fn field_decl(&self, ranged_ident: RangedCustomIdentifier) -> InferResultArcRef<FieldDecl> {
        match self.ty_members.get_entry(ranged_ident.ident) {
            Some(member_decl) => match member_decl {
                TyMemberDecl::Field(field) => Ok(field),
                TyMemberDecl::Method(_) => {
                    Err(derived!(format!("expect a field, but got method instead")))
                }
                TyMemberDecl::Call => todo!(),
            },
            None => Err(derived!(format!("no such field"))),
        }
        // self.fields[field_ident]
        // match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         ..
        //     } => *field_vars.get(ranged_ident.ident).unwrap(),
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => {
        //         if let Some(field_var) = field_vars.get(ranged_ident.ident) {
        //             *field_var
        //         } else if let Some(field_feature) = field_features.get(ranged_ident.ident) {
        //             FieldDecl {
        //                 contract: MembAccessContract::LazyOwn,
        //                 ty: *field_feature,
        //             }
        //         } else {
        //             todo!()
        //         }
        //     }
        //     TyKind::Vec { element_ty } => todo!(),
        // }
    }

    pub fn field_kind(&self, field_ident: CustomIdentifier) -> FieldKind {
        match self.ty_members.get_entry(field_ident).unwrap() {
            TyMemberDecl::Field(field) => field.kind,
            _ => panic!(""),
        }
        // match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         methods: ref field_routines,
        //     } => {
        //         if field_vars.get(field_ident).is_some() {
        //             FieldAccessKind::StructMembVar
        //         } else {
        //             panic!("todo: memb feature of struct")
        //         }
        //     }
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => {
        //         if field_vars.get(field_ident).is_some() {
        //             FieldAccessKind::RecordMemb
        //         } else if field_features.get(field_ident).is_some() {
        //             FieldAccessKind::RecordMemb
        //         } else {
        //             todo!()
        //         }
        //     }
        //     TyKind::Vec { element_ty } => todo!(),
        // }
    }

    pub fn signature(&self) -> TySignature {
        todo!()
        // match self.kind {
        //     TyKind::Struct {
        //         fields: ref field_vars,
        //         ..
        //     } => {
        //         let mut vm_field_vars = IdentDict::<MembAccessContract>::default();
        //         field_vars.iter().for_each(|(ident, field_var_sig)| {
        //             vm_field_vars.insert_new(*ident, field_var_sig.contract)
        //         });
        //         TySignature::Struct {
        //             field_vars: vm_field_vars,
        //         }
        //     }
        //     TyKind::Enum { ref variants } => todo!(),
        //     TyKind::Record {
        //         fields: ref field_vars,
        //         derived_fields: ref field_features,
        //     } => todo!(),
        //     TyKind::Vec { element_ty } => TySignature::Vec,
        // }
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
                TyMemberDecl::Field(_) => todo!(),
                TyMemberDecl::Method(method) => return Ok(method),
                TyMemberDecl::Call => todo!(),
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
                    }
                } else {
                    None
                }
            })
            .collect();
        if matched_methods.len() == 1 {
            return Ok(matched_methods[0]);
        } else {
            throw!(
                format!(
                    "no method named `{}` for type `{:?}`",
                    &ranged_ident.ident, self.this_ty
                ),
                ranged_ident.range
            )
        }
        // ok_or!(
        //     self.type_members.get(ranged_ident.ident),
        //     format!(
        //         "no method named `{}` found in type `{:?}`",
        //         &ranged_ident.ident, self.this_ty
        //     ),
        //     ranged_ident.range
        // )
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

    pub fn trai_impl(&self, trai_route: EntityRoutePtr) -> Option<&Arc<TraitImplDecl>> {
        self.trai_impls
            .iter()
            .find(|trai_impl| trai_impl.trait_route == trai_route)
    }

    pub fn trai_member_impl(
        &self,
        trai: EntityRoutePtr,
        ident: CustomIdentifier,
    ) -> Option<&TraitMemberImplDecl> {
        self.trai_impl(trai)?.member(ident)
    }
}

pub(crate) fn ty_decl(db: &dyn DeclQueryGroup, ty_route: EntityRoutePtr) -> InferResultArc<TyDecl> {
    let source = db.entity_source(ty_route)?;
    match source {
        EntitySource::StaticModuleItem(static_defn) => Ok(match static_defn.variant {
            EntityStaticDefnVariant::Routine { .. } => todo!(),
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Type { .. } => {
                let base_decl = TyDecl::from_static(db, static_defn);
                if ty_route.generic_arguments.len() > 0 {
                    assert_eq!(
                        ty_route.generic_arguments.len(),
                        base_decl.generic_placeholders.len()
                    );
                    base_decl.instantiate(db, &ty_route.generic_arguments)
                } else {
                    base_decl
                }
            }
            EntityStaticDefnVariant::Trait { .. } => todo!(),
            EntityStaticDefnVariant::Method { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TypeField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => todo!(),
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
            match ast.kind {
                AstKind::TypeDefnHead {
                    kind,
                    ref generic_placeholders,
                    ..
                } => {
                    if ty_route.generic_arguments.len() > 0 {
                        todo!()
                    } else {
                        TyDecl::from_ast(
                            db,
                            &ast_text.arena,
                            ty_route,
                            kind,
                            generic_placeholders.clone(),
                            derived_not_none!(item.opt_children)?,
                        )
                    }
                }
                _ => panic!(),
            }
        }
        EntitySource::Module { file } => todo!(),
        EntitySource::Input { .. } => todo!(),
        EntitySource::StaticTypeMember => todo!(),
        EntitySource::StaticTypeAsTraitMember => todo!(),
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
            this_contract,
            input_placeholders: inputs,
            output_ty,
            output_contract,
            generic_placeholders,
            kind,
        } => {
            let generic_placeholders = db.generic_placeholders_from_static(generic_placeholders);
            symbols.extend(db.symbols_from_generic_placeholders(&generic_placeholders));
            let symbol_context = SymbolContext {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: None,
                opt_this_contract: None,
                symbols: symbols.into(),
                kind: SymbolContextKind::Normal,
            };
            let inputs = inputs.map(|input| InputDecl {
                ty: symbol_context.entity_route_from_str(input.ty).unwrap(),
                contract: input.contract,
                ident: db.custom_ident(input.name),
            });
            let output_ty = symbol_context.entity_route_from_str(output_ty).unwrap();
            assert!(matches!(kind, MethodStaticDefnKind::TypeMethod { .. }));
            Arc::new(MethodDecl {
                generic_placeholders,
                inputs,
                output: OutputDecl {
                    contract: output_contract,
                    ty: output_ty,
                },
                this_contract,
                ident: db.intern_word(static_defn.name).custom(),
                kind: MethodKind::Type,
            })
        }
        _ => panic!(""),
    }
}
