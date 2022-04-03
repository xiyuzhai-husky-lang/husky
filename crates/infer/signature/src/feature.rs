use word::ImplicitIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSignature {
    pub ty: ScopePtr,
}

pub(crate) fn feature_signature(
    db: &dyn InferSignatureQueryGroup,
    scope: ScopePtr,
) -> InferResultArc<FeatureSignature> {
    let source = db.scope_source(scope)?;
    match source {
        ScopeSource::Builtin(data) => todo!(),
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
                AstKind::FeatureDecl { ident, ty } => {
                    Ok(Arc::new(FeatureSignature { ty: ty.scope }))
                }
                _ => todo!(),
            }
        }
        ScopeSource::Module { file } => todo!(),
        ScopeSource::Implicit { main, ident } => match ident {
            ImplicitIdentifier::Input => Ok(Arc::new(FeatureSignature {
                ty: db.global_input_ty(main)?,
            })),
        },
    }
}
