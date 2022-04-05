use word::ContextualIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureSignature {
    pub ty: EntityRoutePtr,
}

pub(crate) fn feature_signature(
    db: &dyn InferSignatureQueryGroup,
    scope: EntityRoutePtr,
) -> InferResultArc<FeatureSignature> {
    let source = db.entity_source(scope)?;
    match source {
        EntitySource::Builtin(data) => todo!(),
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
                AstKind::FeatureDecl { ident, ty } => {
                    Ok(Arc::new(FeatureSignature { ty: ty.scope }))
                }
                _ => todo!(),
            }
        }
        EntitySource::Module { file } => todo!(),
        EntitySource::Contextual { main, ident } => match ident {
            ContextualIdentifier::Input => Ok(Arc::new(FeatureSignature {
                ty: db.global_input_ty(main)?,
            })),
            ContextualIdentifier::ThisData => todo!(),
            ContextualIdentifier::ThisType => todo!(),
        },
    }
}
