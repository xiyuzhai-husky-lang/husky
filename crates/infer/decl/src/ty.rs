mod enum_ty;
mod impl_instantiate;
mod record;
mod struct_ty;
mod vec;

pub use enum_ty::*;
pub use record::*;
pub use struct_ty::*;
pub use vec::*;

use crate::*;
use ast::AstIter;
use entity_route::*;
use print_utils::msg_once;
use syntax_types::EnumVariantKind;
use vec_map::VecDict;
use vm::{MembAccessContract, TySignature};
use word::{IdentDict, RangedCustomIdentifier};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TyDecl {
    pub this_type: EntityRoutePtr,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub traits: Vec<EntityRoutePtr>,
    pub fields: IdentDict<FieldDecl>,
    pub methods: IdentDict<MethodDecl>,
    pub variants: IdentDict<EnumVariantDecl>,
    pub kind: TyKind,
}

impl TyDecl {
    fn from_static(db: &dyn DeclQueryGroup, static_decl: &StaticTyDecl) -> Self {
        todo!()
    }

    // fn new(
    //     db: &dyn DeclQueryGroup,
    //     entity_route_kind: EntityRouteKind,
    //     generic_placeholders: IdentDict<GenericPlaceholder>,
    //     traits: Vec<EntityRoutePtr>,
    //     decl_kind: TyDeclKind,
    // ) -> Self {
    //     let mut methods = IdentDict::default();
    //     for trait_route in &traits {
    //         let trait_decl = db.trait_decl(*trait_route).unwrap();
    //         methods.extends(&trait_decl.members)
    //     }
    //     match decl_kind {
    //         TyKind::Struct {
    //             fields: ref field_vars,
    //             methods: ref field_routines,
    //         } => {
    //             for (field_ident, field_access_decl) in field_vars.iter() {
    //                 methods.insert_new(
    //                     *field_ident,
    //                     MembDecl {
    //                         variant: FieldDeclVariant::Var(field_access_decl.clone()),
    //                     },
    //                 )
    //             }
    //             for (field_ident, field_call_decl) in field_routines.iter() {
    //                 methods.insert_new(
    //                     *field_ident,
    //                     MembDecl {
    //                         variant: FieldDeclVariant::Routine(field_call_decl.clone()),
    //                     },
    //                 )
    //             }
    //         }
    //         TyKind::Enum { ref variants } => todo!(),
    //         TyKind::Record {
    //             ref fields,
    //             ref derived_fields,
    //         } => {
    //             for field in fields.iter() {
    //                 todo!()
    //             }
    //             for derived_field in derived_fields.iter() {
    //                 todo!()
    //             }
    //         }
    //         TyKind::Vec { element_ty } => add_vec_methods(db, element_ty, &mut methods),
    //     };
    //     TyDecl {
    //         traits,
    //         members: methods,
    //         kind: decl_kind,
    //         this_type: db.intern_scope(EntityRoute {
    //             kind: entity_route_kind,
    //             generics: generic_placeholders
    //                 .iter()
    //                 .map(|(ident, placeholder)| {
    //                     GenericArgument::Scope(db.intern_scope(EntityRoute {
    //                         kind: EntityRouteKind::Generic {
    //                             ident: *ident,
    //                             entity_kind: placeholder.entity_kind(),
    //                         },
    //                         generics: Vec::new(),
    //                     }))
    //                 })
    //                 .collect(),
    //         }),
    //         generic_placeholders,
    //     }
    // }

    pub fn field_idx(&self, field_ident: CustomIdentifier) -> usize {
        self.fields.position(field_ident).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldAccessKind {
    StructMembVar,
    StructMembFeature,
    RecordMemb,
}

impl TyDecl {
    pub fn field_access_ty_result(
        &self,
        ranged_ident: RangedCustomIdentifier,
    ) -> InferResult<EntityRoutePtr> {
        todo!()
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

    pub fn field_decl(&self, field_ident: CustomIdentifier) -> FieldDecl {
        self.fields[field_ident]
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

    pub fn field_access_kind(&self, field_ident: CustomIdentifier) -> FieldAccessKind {
        todo!()
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

    pub fn method_decl(&self, ranged_ident: RangedCustomIdentifier) -> InferResult<&MethodDecl> {
        todo!()
        // match self.members.get(ranged_ident.ident) {
        //     Some(field_decl) => match field_decl.variant {
        //         FieldDeclVariant::Var(_) => todo!(),
        //         FieldDeclVariant::Routine(ref signature) => Ok(signature),
        //     },
        //     None => err!(
        //         format!(
        //             "no method named `{}` found in type `{:?}`",
        //             &ranged_ident.ident, self.this_type
        //         ),
        //         ranged_ident.range
        //     ),
        // }
    }
}

pub(crate) fn ty_decl(
    db: &dyn DeclQueryGroup,
    entity_route: EntityRoutePtr,
) -> InferResultArc<TyDecl> {
    let source = db.entity_source(entity_route)?;
    match source {
        EntitySource::Builtin(data) => Ok(Arc::new(match data.decl {
            StaticEntityDecl::Func(_) => todo!(),
            StaticEntityDecl::Module => todo!(),
            StaticEntityDecl::Ty { .. } => todo!(),
            StaticEntityDecl::TyTemplate => {
                let vec_decl_template = db.vec_decl();
                vec_decl_template.instantiate(db, &entity_route.generics)
            }
            StaticEntityDecl::Trait { .. } => todo!(),
        })),
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
                } => match kind {
                    TyKind::Enum => enum_decl(
                        db,
                        entity_route.kind.clone(),
                        generic_placeholders.clone(),
                        derived_not_none!(item.children)?,
                    ),
                    TyKind::Struct => struct_decl(
                        db,
                        entity_route.kind.clone(),
                        generic_placeholders.clone(),
                        item.children.unwrap(),
                    ),
                    TyKind::Record => record_decl(
                        db,
                        entity_route.kind.clone(),
                        generic_placeholders.clone(),
                        item.children.unwrap(),
                    ),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Other => todo!(),
                },
                _ => panic!(),
            }
        }
        EntitySource::Module { file } => todo!(),
        EntitySource::Contextual { .. } => todo!(),
    }
}
