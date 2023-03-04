use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub fn value_raw_signature(
    db: &dyn RawSignatureDb,
    decl: ValueDecl,
) -> RawSignatureResult<ValueRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(ValueRawSignature::new(db))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct ValueRawSignature {}

impl ValueRawSignature {
    pub fn implicit_parameters(self, db: &dyn RawSignatureDb) -> &[ImplicitParameterRawSignature] {
        todo!()
        // self.implicit_parameter_decl_list(db)
        //     .as_ref()
        //     .map(|l| -> &[ImplicitParameterRawSignature] { &l })
        //     .unwrap_or(&[])
    }
}
