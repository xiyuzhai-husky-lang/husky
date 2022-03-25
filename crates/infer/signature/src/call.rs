use print_utils::msg_once;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallSignature {
    pub inputs: Vec<InputSignature>,
    pub output: ScopePtr,
    pub compiled: Option<Compiled>,
}

pub(crate) fn call_signature(
    db: &dyn InferSignatureQueryGroup,
    scope: ScopePtr,
) -> InferResultArc<CallSignature> {
    let source = db.scope_source(scope)?;
    return match source {
        ScopeSource::Builtin(data) => Ok(Arc::new(match data.signature {
            scope::StaticScopeSignature::Func(ref signature) => {
                func_call_signature_from_raw(db, signature)
            }
            _ => panic!(),
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
                AstKind::RoutineDecl {
                    ref routine_kind,
                    routine_head: ref decl,
                } => Ok(Arc::new(CallSignature {
                    inputs: decl
                        .input_placeholders
                        .iter()
                        .map(|input_placeholder| InputSignature {
                            contract: input_placeholder.contract,
                            ty: input_placeholder.ranged_ty.scope,
                        })
                        .collect(),
                    output: decl.output.scope,
                    compiled: None,
                })),
                // type constructor
                AstKind::TypeDecl {
                    ref kind,
                    ref generics,
                    ..
                } => match kind {
                    RawTyKind::Enum => todo!(),
                    RawTyKind::Struct => {
                        let mut inputs = vec![];
                        for subitem in item.children.unwrap() {
                            let subast = subitem.value.as_ref()?;
                            match subast.kind {
                                AstKind::MembVar { ident, signature } => {
                                    inputs.push(InputSignature {
                                        contract: signature.contract.constructor_input(),
                                        ty: signature.ty,
                                    })
                                }
                                _ => (),
                            }
                        }
                        msg_once!("struct type call compiled");
                        Ok(Arc::new(CallSignature {
                            inputs,
                            output: scope,
                            compiled: None,
                        }))
                    }
                    RawTyKind::Class => {
                        let mut inputs = vec![];
                        for subitem in item.children.unwrap() {
                            let subast = subitem.value.as_ref()?;
                            match subast.kind {
                                AstKind::MembVar { ident, signature } => {
                                    inputs.push(InputSignature {
                                        contract: signature.contract.constructor_input(),
                                        ty: signature.ty,
                                    })
                                }
                                _ => (),
                            }
                        }
                        Ok(Arc::new(CallSignature {
                            inputs,
                            output: scope,
                            compiled: None,
                        }))
                    }
                },
                _ => panic!(),
            }
        }
        ScopeSource::Module { file: file_id } => todo!(),
    };

    fn func_call_signature_from_raw(
        this: &dyn InferSignatureQueryGroup,
        signature: &StaticFuncSignature,
    ) -> CallSignature {
        let inputs = signature
            .inputs
            .iter()
            .map(|sig| {
                Ok(InputSignature {
                    ty: this.parse_ty(sig.ty)?,
                    contract: sig.contract,
                })
            })
            .collect::<AstResult<Vec<InputSignature>>>()
            .unwrap();
        let output = this.parse_ty(signature.output).unwrap();
        CallSignature {
            inputs,
            output,
            compiled: signature.compiled,
        }
    }
}
