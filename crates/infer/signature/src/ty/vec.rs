use super::*;

pub(crate) fn vec_signature_template(db: &dyn InferSignatureQueryGroup) -> Arc<TySignature> {
    let element_ty = db.intern_scope(Scope {
        kind: ScopeKind::Generic {
            ident: db.custom_ident("T"),
        },
        generics: vec![],
    });
    let mut members = IdentMap::default();
    members.insert_new(
        db.custom_ident("push"),
        MembSignature {
            kind: MembSignatureKind::Routine,
        },
    );
    Arc::new(TySignature {
        generics: vec![GenericArgument::Scope(element_ty)],
        members,
        kind: todo!(),
        traits: todo!(),
    })
}
