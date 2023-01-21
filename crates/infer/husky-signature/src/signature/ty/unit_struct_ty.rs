use super::*;
use husky_word::Identifier;

#[salsa::tracked(jar = SignatureJar)]
pub fn unit_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: UnitStructTypeDecl,
) -> UnitStructTypeSignature {
    UnitStructTypeSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct UnitStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl UnitStructTypeSignature {}
