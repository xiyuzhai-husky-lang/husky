use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn value_signature(db: &dyn SignatureDb, decl: ValueDecl) -> ValueSignature {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    ValueSignature::new(db)
}

#[salsa::interned(jar = SignatureJar)]
pub struct ValueSignature {}

impl ValueSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(|l| -> &[ImplicitParameterSignature] { &l })
        //     .unwrap_or(&[])
    }
}
