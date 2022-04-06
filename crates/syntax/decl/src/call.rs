use entity_route::BuiltinEntityDecl;
use print_utils::msg_once;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallDecl {
    pub inputs: Vec<InputSignature>,
    pub output: EntityRoutePtr,
}

impl CallDecl {
    fn new_vec(ty: EntityRoutePtr) -> Self {
        msg_once!("new vec compiled");
        Self {
            inputs: Vec::new(),
            output: ty,
        }
    }
}

pub(crate) fn call_decl(
    db: &dyn DeclQueryGroup,
    scope: EntityRoutePtr,
) -> InferResultArc<CallDecl> {
    let source = db.entity_source(scope)?;
    return match source {
        EntitySource::Builtin(data) => Ok(Arc::new(match data.decl {
            BuiltinEntityDecl::Func(ref signature) => func_call_decl_from_raw(db, signature),
            BuiltinEntityDecl::Vec => CallDecl::new_vec(scope),
            _ => panic!(),
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
                AstKind::RoutineDefnHead {
                    ref routine_kind,
                    routine_head: ref decl,
                } => Ok(Arc::new(CallDecl {
                    inputs: decl
                        .input_placeholders
                        .iter()
                        .map(|input_placeholder| InputSignature {
                            contract: input_placeholder.contract,
                            ty: input_placeholder.ranged_ty.route,
                        })
                        .collect(),
                    output: decl.output.route,
                })),
                // type constructor
                AstKind::TypeDefnHead {
                    ref kind,
                    generic_placeholders: ref generics,
                    ..
                } => match kind {
                    RawTyKind::Enum => todo!(),
                    RawTyKind::Struct => {
                        let mut inputs = vec![];
                        for subitem in item.children.unwrap() {
                            let subast = subitem.value.as_ref()?;
                            match subast.kind {
                                AstKind::MembVarDefn { ident, signature } => {
                                    inputs.push(InputSignature {
                                        contract: signature.contract.constructor_input(),
                                        ty: signature.ty,
                                    })
                                }
                                _ => (),
                            }
                        }
                        msg_once!("struct type call compiled");
                        Ok(Arc::new(CallDecl {
                            inputs,
                            output: scope,
                        }))
                    }
                    RawTyKind::Record => {
                        let mut inputs = vec![];
                        for subitem in item.children.unwrap() {
                            let subast = subitem.value.as_ref()?;
                            match subast.kind {
                                AstKind::MembVarDefn { ident, signature } => {
                                    inputs.push(InputSignature {
                                        contract: signature.contract.constructor_input(),
                                        ty: signature.ty,
                                    })
                                }
                                _ => (),
                            }
                        }
                        Ok(Arc::new(CallDecl {
                            inputs,
                            output: scope,
                        }))
                    }
                    RawTyKind::Primitive => todo!(),
                    RawTyKind::Vec => todo!(),
                    RawTyKind::Array => todo!(),
                    RawTyKind::Other => todo!(),
                },
                _ => panic!(),
            }
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Contextual { .. } => todo!(),
    };

    fn func_call_decl_from_raw(this: &dyn DeclQueryGroup, signature: &StaticFuncDecl) -> CallDecl {
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
        CallDecl { inputs, output }
    }
}
