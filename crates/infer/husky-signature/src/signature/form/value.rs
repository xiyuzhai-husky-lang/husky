use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn value_signature(db: &dyn SignatureDb, decl: ValueDecl) -> ValueSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), None);
    // implementation
    ValueSignature::new(db, engine.finish())
}

#[salsa::tracked(jar = SignatureJar)]
pub struct ValueSignature {
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl ValueSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(|l| -> &[ImplicitParameterSignature] { &l })
        //     .unwrap_or(&[])
    }
}
