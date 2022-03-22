use super::*;

pub(crate) fn ty_signature(
    db: &dyn InferSignatureQueryGroup,
    scope: ScopePtr,
) -> InferResultArc<TySignature> {
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
                    RawTyKind::Enum => enum_signature(not_none_or_derived!(item.children)),
                    RawTyKind::Struct => struct_signature(item.children.unwrap()),
                },
                _ => panic!(),
            }
        }
        ScopeSource::Module { file } => todo!(),
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
                signature: MembVarSignature { contract, ty },
            } => memb_vars.insert_new(ident, MembVarSignature { contract, ty }),
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
