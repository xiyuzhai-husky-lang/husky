mod enum_ty;
mod impl_instantiate;
mod record;
mod struct_ty;
mod vec;

use atom::symbol_proxy::{Symbol, SymbolKind};
pub use enum_ty::*;
use fold::LocalStack;
use map_collect::MapCollect;
pub use record::*;
pub use struct_ty::*;
pub use vec::*;

use crate::*;
use ast::AstIter;
use defn_head::*;
use entity_route::*;
use vec_dict::VecDict;
use vm::TySignature;
use word::{IdentDict, RangedCustomIdentifier};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TyDecl {
    pub this_ty: EntityRoutePtr,
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub traits: Vec<EntityRoutePtr>,
    pub fields: IdentDict<FieldDecl>,
    pub methods: IdentDict<MethodDecl>,
    pub variants: IdentDict<EnumVariantDecl>,
    pub kind: TyKind,
}

impl TyDecl {
    fn from_static(db: &dyn DeclQueryGroup, static_decl: &StaticTyDecl) -> Self {
        let (generic_placeholders, generics, symbols) =
            db.parse_generics(static_decl.generic_placeholders);
        let base_ty = db
            .parse_entity(static_decl.base_ty, None, &symbols)
            .unwrap();
        let this_ty = db.intern_scope(EntityRoute {
            kind: base_ty.kind,
            generics,
        });
        Self {
            this_ty,
            generic_placeholders,
            traits: static_decl
                .traits
                .map(|t| db.parse_entity(*t, Some(this_ty), &symbols).unwrap()),
            fields: static_decl
                .fields
                .iter()
                .map(|field| FieldDecl::from_static(db, field))
                .collect(),
            methods: static_decl
                .methods
                .iter()
                .map(|method| MethodDecl::from_static(db, method, this_ty, &symbols))
                .collect(),
            variants: static_decl.variants.map(|static_decl| {
                EnumVariantDecl::from_static(db, static_decl, this_ty, &symbols)
            }),
            kind: static_decl.kind,
        }
    }

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
        ok_or!(
            self.methods.get(ranged_ident.ident),
            format!(
                "no method named `{}` found in type `{:?}`",
                &ranged_ident.ident, self.this_ty
            ),
            ranged_ident.range
        )
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
        EntitySource::Input { .. } => todo!(),
    }
}
