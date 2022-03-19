use ast::AstIter;
use fold::{FoldIter, FoldedList};
use scope::StaticScopeSignature;
use syntax_types::{MembVarSignature, RawEnumVariantKind};
use vec_map::VecMap;
use vm::{MembVarContract, VMTySignature};

use crate::*;

pub type IdentMap<T> = VecMap<CustomIdentifier, T>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignature {
    Struct {
        memb_vars: IdentMap<MembVarSignature>,
    },
    Enum {
        variants: IdentMap<EnumVariantSignature>,
    },
}

impl TySignature {
    pub fn memb_var_ty(&self, ident: CustomIdentifier) -> ScopePtr {
        match self {
            TySignature::Struct { ref memb_vars } => memb_vars.get(ident).unwrap().ty,
            TySignature::Enum { ref variants } => todo!(),
        }
    }

    pub fn vm_ty_signature(&self) -> VMTySignature {
        match self {
            TySignature::Struct { memb_vars } => {
                let mut vm_memb_vars = IdentMap::<MembVarContract>::default();
                memb_vars.iter().for_each(|(ident, memb_var_sig)| {
                    vm_memb_vars.insert_new(*ident, memb_var_sig.contract)
                });
                VMTySignature::Struct {
                    memb_vars: vm_memb_vars,
                }
            }
            TySignature::Enum { variants } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantSignature {
    Constant,
}

pub(crate) fn ty_signature(
    db: &dyn InferQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<TySignature> {
    let source = db.scope_source(scope)?;
    match source {
        ScopeSource::Builtin(data) => Ok(Arc::new(match data.signature {
            StaticScopeSignature::Func(_) => todo!(),
            StaticScopeSignature::Module => todo!(),
            StaticScopeSignature::Ty(_) => todo!(),
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
                AstKind::TypeDef { kind, .. } => match kind {
                    RawTyKind::Enum => enum_signature(not_none!(item.children)),
                    RawTyKind::Struct => struct_signature(item.children.unwrap()),
                },
                _ => panic!(),
            }
        }
        ScopeSource::Module { file } => todo!(),
    }
}

fn enum_signature(children: AstIter) -> SemanticResultArc<TySignature> {
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

fn struct_signature(children: AstIter) -> SemanticResultArc<TySignature> {
    let mut memb_vars = VecMap::default();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVar {
                ident,
                signature: MembVarSignature { contract, ty },
            } => memb_vars.insert_new(ident, MembVarSignature { contract, ty }),
            AstKind::MembRoutineDecl(_) => todo!(),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TySignature::Struct { memb_vars }))
}
