use crate::*;

#[salsa::jar(db = LinkageDb)]
pub struct LinkageJar(crate::linkage::Linkage);
