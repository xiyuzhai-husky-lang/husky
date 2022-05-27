use word::ContextualIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureDecl {
    pub ty: EntityRoutePtr,
}

pub(crate) fn feature_decl(
    db: &dyn DeclQueryGroup,
    scope: EntityRoutePtr,
) -> InferResultArc<FeatureDecl> {
    let source = db.entity_locus(scope)?;
    match source {
        EntityLocus::StaticModuleItem(data) => todo!(),
        EntityLocus::WithinBuiltinModule => todo!(),
        EntityLocus::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.variant {
                AstKind::FeatureDecl { ident, ty } => Ok(Arc::new(FeatureDecl { ty: ty.route })),
                _ => todo!(),
            }
        }
        EntityLocus::Module { file } => todo!(),
        EntityLocus::Input { main } => Ok(Arc::new(FeatureDecl {
            ty: db.eval_input_ty(main)?,
        })),
        EntityLocus::StaticTypeMember => todo!(),
        EntityLocus::StaticTypeAsTraitMember => todo!(),
    }
}
