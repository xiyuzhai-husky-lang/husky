use std::sync::Arc;

use ast::{Ast, AstKind, AstResult, RawExprArena, RawExprKind, RawStmt, RawStmtKind};
use common::*;
use fold::FoldStorage;
use scope::{
    FuncSignature, InputPlaceholder, InputSignature, ScopeKind, ScopePtr, ScopeRoute,
    StaticFuncSignature,
};
use scope_query::ScopeQueryGroup;
use syntax_types::{ListOpr, Opr};
use word::{BuiltinIdentifier, ImplicitIdentifier};

use semantics_error::*;

#[salsa::query_group(InferQueryGroupStorage)]
pub trait InferQueryGroup: ScopeQueryGroup + ast::AstQueryGroup {
    fn func_signature(&self, scope: ScopePtr) -> SemanticResult<Arc<FuncSignature>>;
    fn scope_ty(&self, scope: ScopePtr) -> SemanticResult<ScopePtr>;
    fn input_ty(&self, main_file: file::FilePtr) -> SemanticResult<ScopePtr>;
}

fn func_signature(
    this: &dyn InferQueryGroup,
    scope: ScopePtr,
) -> SemanticResult<Arc<FuncSignature>> {
    let source = this.scope_source(scope)?;
    return match source {
        scope::ScopeSource::Builtin(data) => Ok(Arc::new(match data.signature {
            scope::StaticScopeSignature::Func(ref signature) => {
                func_signature_from_raw(this, signature)
            }
            _ => panic!(),
        })),
        scope::ScopeSource::WithinBuiltinModule => todo!(),
        scope::ScopeSource::WithinModule {
            file: file_id,
            token_group_index,
        } => {
            let ast_text = this.ast_text(file_id)?;
            let ast = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap()
                .value
                .as_ref()?;
            match ast.kind {
                AstKind::RoutineDef {
                    routine_kind: ref kind,
                    routine_head: ref decl,
                } => Ok(Arc::new(FuncSignature {
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
                _ => panic!(),
            }
        }
        scope::ScopeSource::Module { file: file_id } => todo!(),
    };

    fn func_signature_from_raw(
        this: &dyn InferQueryGroup,
        signature: &StaticFuncSignature,
    ) -> FuncSignature {
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
        FuncSignature {
            inputs,
            output,
            compiled: signature.compiled,
        }
    }
}

fn scope_ty(this: &dyn InferQueryGroup, scope: ScopePtr) -> SemanticResult<ScopePtr> {
    match scope {
        ScopePtr::Builtin(ident) => match ident {
            BuiltinIdentifier::Void => todo!(),
            BuiltinIdentifier::I32 => todo!(),
            BuiltinIdentifier::F32 => todo!(),
            BuiltinIdentifier::B32 => todo!(),
            BuiltinIdentifier::B64 => todo!(),
            BuiltinIdentifier::Bool => todo!(),
            BuiltinIdentifier::True | BuiltinIdentifier::False => {
                Ok(ScopePtr::Builtin(BuiltinIdentifier::Bool))
            }
            BuiltinIdentifier::Vector => todo!(),
            BuiltinIdentifier::Tuple => todo!(),
            BuiltinIdentifier::Debug => todo!(),
            BuiltinIdentifier::Std => todo!(),
            BuiltinIdentifier::Core => todo!(),
            BuiltinIdentifier::Fp => todo!(),
            BuiltinIdentifier::Fn => todo!(),
            BuiltinIdentifier::FnMut => todo!(),
            BuiltinIdentifier::FnOnce => todo!(),
            BuiltinIdentifier::Array => todo!(),
            BuiltinIdentifier::DatasetType => todo!(),
        },
        ScopePtr::Custom(scope) => match scope.route {
            ScopeRoute::Implicit { main, ident } => match ident {
                ImplicitIdentifier::Input => input_ty(this, main),
            },
            _ => todo!(),
        },
    }
}

fn input_ty(this: &dyn InferQueryGroup, main_file: file::FilePtr) -> SemanticResult<ScopePtr> {
    let ast_text = this.ast_text(main_file)?;
    for item in ast_text.folded_results.fold_iter(0) {
        match item.value.as_ref()?.kind {
            AstKind::DatasetConfig => {
                return input_ty_from_ast(
                    this,
                    &ast_text.arena,
                    not_none!(item.children).last().unwrap().value.as_ref()?,
                )
            }
            _ => (),
        }
    }
    err!("dataset config not found, so input type can't be inferred")
}

fn input_ty_from_ast(
    this: &dyn InferQueryGroup,
    arena: &RawExprArena,
    ast: &Ast,
) -> SemanticResult<ScopePtr> {
    match ast.kind {
        AstKind::Stmt(RawStmt {
            kind: RawStmtKind::Return(idx),
            ..
        }) => match arena[idx].kind {
            RawExprKind::Opn {
                opr: Opr::List(ListOpr::Call),
                ref opds,
            } => match arena[opds][0].kind {
                RawExprKind::Scope {
                    scope,
                    kind: ScopeKind::Routine,
                    ..
                } => {
                    let signature = this.func_signature(scope)?;
                    let dataset_type = signature.output;
                    match dataset_type.route {
                        ScopeRoute::Builtin {
                            ident: BuiltinIdentifier::DatasetType,
                        } => match dataset_type.generics[0] {
                            scope::GenericArgument::Const(_) => todo!(),
                            scope::GenericArgument::Scope(input_ty) => Ok(input_ty),
                        },
                        _ => panic!(),
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        },
        _ => todo!(),
    }
}
