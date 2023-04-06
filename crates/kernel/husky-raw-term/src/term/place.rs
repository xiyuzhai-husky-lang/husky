use super::*;

/// maybe this is comparable with viewtype or viewt@ype in ATS?
#[salsa::interned(db = RawTermDb, jar = RawTermJar, constructor = new)]
pub struct RawTermPlace {}
