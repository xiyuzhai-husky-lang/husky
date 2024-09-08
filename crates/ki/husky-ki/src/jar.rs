use crate::{version_stamp::KiVersionStamp, *};

#[salsa::jar]
pub struct KiJar(
    Ki,
    KiPattern,
    KiRuntimeConstant,
    KiVersionStamp,
    crate::genki::Genki,
    crate::genki::GenkiRuntimeConstant,
);
