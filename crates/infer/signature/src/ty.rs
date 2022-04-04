use ast::AstIter;
use scope::BuiltinScopeSignature;
use syntax_types::{MembAccessSignature, MembCallSignature, RawEnumVariantKind};
use vec_map::VecMap;
use vm::{MembAccessContract, VMTySignature};
use word::IdentMap;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Struct {
        memb_vars: IdentMap<MembAccessSignature>,
        memb_routines: IdentMap<MembCallSignature>,
    },
    Enum {
        variants: IdentMap<EnumVariantSignature>,
    },
    Record {
        memb_vars: IdentMap<MembAccessSignature>,
        memb_features: IdentMap<ScopePtr>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MembAccessKind {
    StructMembVar,
    StructMembFeature,
    RecordMemb,
}

impl TySignature {
    pub fn memb_access_ty_result(&self, ident: CustomIdentifier) -> InferResult<ScopePtr> {
        match self {
            TySignature::Struct { ref memb_vars, .. } => ok_or!(
                memb_vars.get(ident),
                format!("no such member variable {}", ident.0)
            )
            .map(|signature| signature.ty),
            TySignature::Enum { ref variants } => todo!(),
            TySignature::Record {
                memb_vars,
                memb_features,
            } => {
                if let Some(memb_var) = memb_vars.get(ident) {
                    Ok(memb_var.ty)
                } else if let Some(memb_feature) = memb_features.get(ident) {
                    Ok(*memb_feature)
                } else {
                    todo!()
                }
            }
        }
    }

    pub fn memb_access_signature(&self, ident: CustomIdentifier) -> MembAccessSignature {
        match self {
            TySignature::Struct { ref memb_vars, .. } => *memb_vars.get(ident).unwrap(),
            TySignature::Enum { ref variants } => todo!(),
            TySignature::Record {
                ref memb_vars,
                ref memb_features,
            } => {
                if let Some(memb_var) = memb_vars.get(ident) {
                    *memb_var
                } else if let Some(memb_feature) = memb_features.get(ident) {
                    MembAccessSignature {
                        contract: MembAccessContract::LazyOwn,
                        ty: *memb_feature,
                    }
                } else {
                    todo!()
                }
            }
        }
    }

    pub fn memb_access_kind(&self, memb_ident: CustomIdentifier) -> MembAccessKind {
        match self {
            TySignature::Struct {
                memb_vars,
                memb_routines,
            } => {
                if memb_vars.get(memb_ident).is_some() {
                    MembAccessKind::StructMembVar
                } else {
                    panic!("todo: memb feature of struct")
                }
            }
            TySignature::Enum { variants } => todo!(),
            TySignature::Record {
                memb_vars,
                memb_features,
            } => {
                if memb_vars.get(memb_ident).is_some() {
                    MembAccessKind::RecordMemb
                } else if memb_features.get(memb_ident).is_some() {
                    MembAccessKind::RecordMemb
                } else {
                    todo!()
                }
            }
        }
    }

    pub fn vm_ty_signature(&self) -> VMTySignature {
        match self {
            TySignature::Struct { memb_vars, .. } => {
                let mut vm_memb_vars = IdentMap::<MembAccessContract>::default();
                memb_vars.iter().for_each(|(ident, memb_var_sig)| {
                    vm_memb_vars.insert_new(*ident, memb_var_sig.contract)
                });
                VMTySignature::Struct {
                    memb_vars: vm_memb_vars,
                }
            }
            TySignature::Enum { variants } => todo!(),
            TySignature::Record {
                memb_vars,
                memb_features,
            } => todo!(),
        }
    }

    pub fn memb_call_signature(&self, ident: CustomIdentifier) -> InferResult<&MembCallSignature> {
        match self {
            TySignature::Struct {
                memb_routines: ref memb_calls,
                ..
            } => {
                derived_not_none!(memb_calls.get(ident))
            }
            TySignature::Enum { variants } => todo!(),
            TySignature::Record {
                memb_vars,
                memb_features,
            } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantSignature {
    Constant,
}

pub(crate) fn ty_signature(
    db: &dyn InferSignatureQueryGroup,
    scope: ScopePtr,
) -> InferResultArc<TySignature> {
    let source = db.scope_source(scope)?;
    match source {
        ScopeSource::Builtin(data) => Ok(Arc::new(match data.signature {
            BuiltinScopeSignature::Func(_) => todo!(),
            BuiltinScopeSignature::Module => todo!(),
            BuiltinScopeSignature::Ty { .. } => todo!(),
            BuiltinScopeSignature::Vec => todo!(),
        })),
        ScopeSource::WithinBuiltinModule => todo!(),
        ScopeSource::WithinModule {
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
                AstKind::TypeDecl { kind, .. } => match kind {
                    RawTyKind::Enum => enum_signature(derived_not_none!(item.children)?),
                    RawTyKind::Struct => struct_signature(item.children.unwrap()),
                    RawTyKind::Record => class_signature(item.children.unwrap()),
                },
                _ => panic!(),
            }
        }
        ScopeSource::Module { file } => todo!(),
        ScopeSource::Implicit { .. } => todo!(),
    }
}

pub(crate) fn enum_signature(children: AstIter) -> InferResultArc<TySignature> {
    let mut variants = VecMap::default();
    for subitem in children {
        match subitem.value.as_ref()?.kind {
            AstKind::EnumVariant {
                ident,
                ref raw_variant_kind,
            } => {
                let variant_sig = match raw_variant_kind {
                    RawEnumVariantKind::Constant => EnumVariantSignature::Constant,
                };
                variants.insert_new(ident, variant_sig)
            }
            _ => panic!(),
        }
    }
    Ok(Arc::new(TySignature::Enum { variants }))
}

pub(crate) fn struct_signature(children: AstIter) -> InferResultArc<TySignature> {
    let mut memb_vars = VecMap::default();
    let mut memb_routines = VecMap::default();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVar {
                ident,
                signature: MembAccessSignature { contract, ty },
            } => memb_vars.insert_new(ident, MembAccessSignature { contract, ty }),
            AstKind::MembRoutineDecl {
                ref memb_routine_head,
                ..
            } => memb_routines.insert_new(memb_routine_head.routine_name, memb_routine_head.into()),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TySignature::Struct {
        memb_vars,
        memb_routines,
    }))
}

pub(crate) fn class_signature(children: AstIter) -> InferResultArc<TySignature> {
    let mut memb_vars = VecMap::default();
    let mut memb_features = VecMap::default();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVar {
                ident,
                signature: MembAccessSignature { contract, ty },
            } => memb_vars.insert_new(ident, MembAccessSignature { contract, ty }),
            AstKind::MembFeatureDecl { ident, ty } => memb_features.insert_new(ident, ty),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TySignature::Record {
        memb_vars,
        memb_features,
    }))
}
