#[salsa::jar]
pub struct LxAstJar(crate::district::LxAstDistrict, crate::region::LxAstRegion);
