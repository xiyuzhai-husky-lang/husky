mod enum_variant;
mod impl_instantiate;
mod member;
mod method;
mod trait_impl;
mod vec;

use std::iter::Peekable;

use entity_syntax::{EnumVariantKind, RoutineKind};
pub use member::*;
pub use method::*;
use print_utils::p;
pub use trait_impl::*;
pub use vec::*;

use crate::*;
use ast::AstIter;
use atom::symbol_proxy::{Symbol, SymbolKind};
use defn_head::*;
use entity_route::*;
pub use enum_variant::*;
use fold::LocalStack;
use map_collect::MapCollect;
use vec_dict::VecDict;
use vm::TySignature;
use word::{IdentArcDict, IdentDict, RangedCustomIdentifier};

#[derive(Debug, PartialEq, Eq)]
pub struct TyDecl {
    pub this_ty: EntityRoutePtr,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub type_members: IdentDict<TypeMemberDecl>,
    pub variants: IdentDict<EnumVariantDecl>,
    pub kind: TyKind,
    pub trait_impls: Vec<Arc<TraitImplDecl>>,
    pub members: Vec<MemberDecl>,
}

impl TyDecl {
    fn from_static(db: &dyn DeclQueryGroup, static_decl: &StaticTyDecl) -> Arc<Self> {
        let generic_placeholders =
            db.parse_generic_placeholders_from_static(static_decl.generic_placeholders);
        let generic_arguments =
            db.generic_arguments_from_generic_placeholders(&generic_placeholders);
        let symbols = db.symbols_from_generic_placeholders(&generic_placeholders);
        let base_ty = db
            .parse_entity(static_decl.base_ty, None, &symbols)
            .unwrap();
        let this_ty = db.intern_entity_route(EntityRoute {
            kind: base_ty.kind,
            generic_arguments,
        });
        Self::new(
            db,
            this_ty,
            generic_placeholders,
            static_decl
                .members
                .iter()
                .map(|member| TypeMemberDecl::from_static(db, member, this_ty, &symbols))
                .collect(),
            static_decl.variants.map(|static_decl| {
                EnumVariantDecl::from_static(db, static_decl, this_ty, &symbols)
            }),
            static_decl.kind,
            static_decl
                .trait_impls
                .map(|trait_impl| TraitImplDecl::from_static(db, trait_impl, this_ty, &symbols)),
        )
    }

    fn from_ast(
        db: &dyn DeclQueryGroup,
        arena: &RawExprArena,
        ty_route: EntityRoutePtr,
        kind: TyKind,
        generic_placeholders: IdentDict<GenericPlaceholder>,
        children: AstIter,
    ) -> InferResultArc<Self> {
        let generic_arguments =
            db.generic_arguments_from_generic_placeholders(&generic_placeholders);
        let this_ty = db.intern_entity_route(EntityRoute {
            kind: ty_route.kind,
            generic_arguments,
        });
        let mut children = children.peekable();
        let mut type_members = IdentDict::default();
        let mut trait_impls = Vec::default();
        Self::collect_fields(&mut children, &mut type_members)?;
        Self::collect_member_calls(db, arena, &mut children, &mut type_members)?;
        let variants = Self::collect_variants(children)?;
        Ok(TyDecl::new(
            db,
            this_ty,
            generic_placeholders,
            type_members,
            variants,
            kind,
            trait_impls,
        ))
    }

    fn collect_fields(
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<TypeMemberDecl>,
    ) -> InferResult<()> {
        while let Some(child) = children.peek() {
            match child.value.as_ref()?.kind {
                AstKind::FieldDefn(ref field_defn_head) => {
                    children.next();
                    members.insert_new(TypeMemberDecl::Field(FieldDecl::from_ast(field_defn_head)))
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn collect_member_calls(
        db: &dyn DeclQueryGroup,
        arena: &RawExprArena,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<TypeMemberDecl>,
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
                AstKind::MembFeatureDefnHead { ident, ty } => todo!(),
                AstKind::MethodDefnHead(ref method_defn_head) => {
                    match method_defn_head.routine_kind {
                        RoutineKind::Proc => todo!(),
                        RoutineKind::Func => members.insert_new(TypeMemberDecl::Method(
                            MethodDecl::from_ast(method_defn_head),
                        )),
                        RoutineKind::Test => todo!(),
                    }
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
    ) -> InferResult<IdentDict<EnumVariantDecl>> {
        let mut variants = VecDict::default();
        for subitem in children {
            match subitem.value.as_ref()?.kind {
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: ref raw_variant_kind,
                } => variants.insert_new(EnumVariantDecl {
                    ident,
                    variant: match raw_variant_kind {
                        EnumVariantKind::Constant => EnumVariantDeclVariant::Constant,
                    },
                }),
                _ => panic!(),
            }
        }
        Ok(variants)
    }

    pub(crate) fn new(
        db: &dyn DeclQueryGroup,
        this_ty: EntityRoutePtr,
        generic_placeholders: IdentDict<GenericPlaceholder>,
        type_members: IdentDict<TypeMemberDecl>,
        variants: IdentDict<EnumVariantDecl>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDecl>>,
    ) -> Arc<Self> {
        let members = MemberDecl::collect_all(db, &type_members, &trait_impls);
        Arc::new(Self {
            this_ty,
            generic_placeholders,
            type_members,
            variants,
            kind,
            trait_impls,
            members,
        })
    }

    pub fn field_idx(&self, field_ident: CustomIdentifier) -> usize {
        todo!()
        // self.fields.position(field_ident).unwrap()
    }

    pub fn fields(&self) -> impl Iterator<Item = &FieldDecl> {
        self.type_members.iter().filter_map(|member| match member {
            TypeMemberDecl::Field(field_decl) => Some(field_decl as &FieldDecl),
            _ => None,
        })
    }
}

impl TyDecl {
    pub fn field_ty_result(
        &self,
        ranged_ident: RangedCustomIdentifier,
    ) -> InferResult<EntityRoutePtr> {
        match self.type_members.get(ranged_ident.ident) {
            Some(type_member_decl) => match type_member_decl {
                TypeMemberDecl::Field(field) => Ok(field.ty),
                TypeMemberDecl::Method(_) => todo!(),
                TypeMemberDecl::Call => todo!(),
            },
            None => todo!(),
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
        match self.type_members.get(ranged_ident.ident) {
            Some(member_decl) => match member_decl {
                TypeMemberDecl::Field(field) => Ok(field),
                TypeMemberDecl::Method(_) => todo!(),
                TypeMemberDecl::Call => todo!(),
            },
            None => todo!(),
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
        match self.type_members.get(field_ident).unwrap() {
            TypeMemberDecl::Field(field) => field.kind,
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

    pub fn method_decl(
        &self,
        ranged_ident: RangedCustomIdentifier,
        trait_uses: &[EntityRouteKind],
    ) -> InferResult<&Arc<MethodDecl>> {
        // the rule is:
        // first look in the type members,
        // then look in the trait members,
        // if multiple are found in the trait members,
        // report an infer error.
        if let Some(member_decl) = self.type_members.get(ranged_ident.ident) {
            match member_decl {
                TypeMemberDecl::Field(_) => todo!(),
                TypeMemberDecl::Method(method) => return Ok(method),
                TypeMemberDecl::Call => todo!(),
            }
        }
        let matched_methods: Vec<&Arc<MethodDecl>> = self
            .members
            .iter()
            .filter_map(|member| {
                if member.ident() == ranged_ident.ident {
                    match member {
                        MemberDecl::AssociatedType => todo!(),
                        MemberDecl::AssociatedCall => todo!(),
                        MemberDecl::TypeField(_) => todo!(),
                        MemberDecl::TypeMethod(_) => todo!(),
                        MemberDecl::TraitMethod {
                            trait_route,
                            method,
                        } => {
                            if is_trait_availabe(*trait_route, trait_uses) {
                                Some(method)
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    None
                }
            })
            .collect();
        if matched_methods.len() == 1 {
            return Ok(matched_methods[0]);
        } else {
            p!(ranged_ident);
            p!(self.type_members);
            p!(matched_methods.len());
            todo!()
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
}

pub(crate) fn ty_decl(db: &dyn DeclQueryGroup, ty_route: EntityRoutePtr) -> InferResultArc<TyDecl> {
    let source = db.entity_source(ty_route)?;
    match source {
        EntitySource::Builtin(data) => Ok(match data.decl {
            StaticEntityDecl::Func(_) => todo!(),
            StaticEntityDecl::Module => todo!(),
            StaticEntityDecl::Ty { .. } => todo!(),
            StaticEntityDecl::TyTemplate => {
                let vec_decl_template = db.vec_decl();
                vec_decl_template.instantiate(db, &ty_route.generic_arguments)
            }
            StaticEntityDecl::Trait { .. } => todo!(),
        }),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .fold_iter(token_group_index)
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
                            item.children.unwrap(),
                        )
                    }
                }
                _ => panic!(),
            }
        }
        EntitySource::Module { file } => todo!(),
        EntitySource::Input { .. } => todo!(),
    }
}

fn is_trait_availabe(trait_route: EntityRoutePtr, trait_uses: &[EntityRouteKind]) -> bool {
    match trait_route.kind {
        EntityRouteKind::Root { ident } => true,
        EntityRouteKind::Package { main, ident } => todo!(),
        EntityRouteKind::ChildScope { parent, ident } => todo!(),
        EntityRouteKind::Input { main } => todo!(),
        EntityRouteKind::Generic { ident, entity_kind } => todo!(),
        EntityRouteKind::ThisType => todo!(),
    }
}
