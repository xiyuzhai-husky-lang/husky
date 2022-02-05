use std::sync::Arc;

use common::*;
use file::FileId;
use syntax_types::*;
use word::{BuiltinIdentifier, CustomIdentifier};

use crate::*;
use fold::FoldStorage;

use scope::{ScopeId, ScopeSource};

use super::scope_signature::*;

#[salsa::query_group(ScopeValidatorStorage)]
pub trait ScopeValidator: ast::AstQuery {
    fn scope_signature(&self, scope: ScopeId) -> SemanticResultArc<ScopeSignature>;

    fn validate_scope(&self, scope: ScopeId) -> SemanticResult<()>;
}

fn scope_signature(this: &dyn ScopeValidator, scope: ScopeId) -> SemanticResultArc<ScopeSignature> {
    let source = this.scope_source(scope)?;
    Ok(Arc::new(match source {
        ScopeSource::Builtin(scope) => match scope {
            BuiltinIdentifier::Void
            | BuiltinIdentifier::I32
            | BuiltinIdentifier::F32
            | BuiltinIdentifier::Debug
            | BuiltinIdentifier::Std
            | BuiltinIdentifier::Core
            | BuiltinIdentifier::Input => vec![].into(),
            BuiltinIdentifier::Fp
            | BuiltinIdentifier::Fn
            | BuiltinIdentifier::FnMut
            | BuiltinIdentifier::FnOnce => ScopeSignature::Variadic(1),
            BuiltinIdentifier::Vector => vec![SpaceParamKind::Type { traits: vec![] }].into(),
            BuiltinIdentifier::Array => vec![
                SpaceParamKind::Type { traits: vec![] },
                SpaceParamKind::Const,
            ]
            .into(),

            BuiltinIdentifier::Tuple => ScopeSignature::Variadic(1),
        },
        ScopeSource::WithinCustomModule {
            file_id,
            token_group_index,
        } => {
            let ast_text = this.ast_text(file_id)?;
            let item = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref().unwrap();
            match ast {
                ast::Ast::TypeDef { generics, .. } => ScopeSignature::Fixed(generics.clone()),
                ast::Ast::MainDef => todo!(),
                ast::Ast::FuncDef { decl, .. } => todo!(),
                ast::Ast::PatternDef => todo!(),
                ast::Ast::Use { .. } | ast::Ast::MembDef { .. } | ast::Ast::Stmt(_) => panic!(),
                ast::Ast::DatasetConfig => todo!(),
            }
        }
        ScopeSource::Module { file_id } => ScopeSignature::default(),
        ScopeSource::WithinBuiltinModule => todo!(),
    }))
}

fn validate_scope(this: &dyn ScopeValidator, scope: ScopeId) -> SemanticResult<()> {
    todo!()
}
