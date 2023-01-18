use super::*;
use husky_word::Identifier;

#[salsa::tracked(jar = SignatureJar)]
pub fn unit_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: UnitStructTypeDecl,
) -> UnitStructTypeSignature {
    // implementation
    UnitStructTypeSignature::new(db,todo!())
}

#[salsa::tracked(jar = SignatureJar)]
pub struct UnitStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
}

impl UnitStructTypeSignature {}
