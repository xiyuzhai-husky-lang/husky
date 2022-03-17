use scope::StaticScopeSignature;
use vm::{MembVarContract, VMMembVarSignature, VMTySignature};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TySignature {
    pub memb_vars: Vec<MembVarSignature>,
}

impl TySignature {
    pub fn memb_var_ty(&self, ident: CustomIdentifier) -> ScopePtr {
        self.memb_vars
            .iter()
            .find(|memb_var_sig| memb_var_sig.ident == ident)
            .unwrap()
            .ty
    }

    pub fn vm_ty_signature(&self) -> VMTySignature {
        VMTySignature::Struct {
            memb_vars: self
                .memb_vars
                .iter()
                .map(|memb_var| memb_var.into())
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembVarSignature {
    pub ident: CustomIdentifier,
    pub contract: MembVarContract,
    pub ty: ScopePtr,
}

impl Into<VMMembVarSignature> for &MembVarSignature {
    fn into(self) -> VMMembVarSignature {
        VMMembVarSignature {
            ident: self.ident,
            contract: self.contract,
        }
    }
}

pub(crate) fn ty_signature(
    this: &dyn InferQueryGroup,
    scope: ScopePtr,
) -> SemanticResultArc<TySignature> {
    let source = this.scope_source(scope)?;
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
            let ast_text = this.ast_text(file)?;
            let item = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.kind {
                AstKind::TypeDef { kind, .. } => {
                    match kind {
                        TyKind::Enum => err!("enum type doesn't have member variables"),
                        TyKind::Struct => (),
                    }
                    let mut memb_vars = vec![];
                    for subitem in item.children.unwrap() {
                        let subast = subitem.value.as_ref()?;
                        match subast.kind {
                            AstKind::MembDef {
                                ident,
                                memb_kind: MembKind::MembVar { contract, ty },
                            } => memb_vars.push(MembVarSignature {
                                ident,
                                contract,
                                ty,
                            }),
                            AstKind::MembDef {
                                ident,
                                memb_kind: MembKind::MembFunc { .. },
                            } => todo!(),
                            _ => panic!(),
                        }
                    }
                    Ok(Arc::new(TySignature { memb_vars }))
                }
                _ => panic!(),
            }
        }
        ScopeSource::Module { file } => todo!(),
    }
}
