use super::*;

/// maybe this is comparable with viewtype or viewt@ype in ATS?
#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermQualifiedType {
    pub qual: Qual,
    pub base_ty: RawTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Qual {}
