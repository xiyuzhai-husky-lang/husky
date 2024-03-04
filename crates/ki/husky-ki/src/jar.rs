use crate::{version_stamp::ValVersionStamp, *};

#[salsa::jar]
pub struct ValJar(Ki, ValPattern, ValRuntimeConstant, ValVersionStamp);
