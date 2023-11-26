use crate::*;

#[salsa::jar(db = TokenDataDb)]
pub struct TokenDataJar(UnspecifiedFloatLiteral);
