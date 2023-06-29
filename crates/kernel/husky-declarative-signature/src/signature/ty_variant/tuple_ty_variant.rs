use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TupleTypeVariantDeclarativeSignatureTemplate {
    pub ty: DeclarativeTerm,
}

impl TupleTypeVariantDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        decl: TupleTypeVariantDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let ty = todo!();
        Ok(Self::new(db, ty))
    }
}
