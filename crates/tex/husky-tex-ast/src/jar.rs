#[salsa::jar]
pub struct TexAstJar(crate::district::TexAstDistrict, crate::region::TexAstRegion);
