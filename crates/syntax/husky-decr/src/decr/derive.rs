use super::*;

#[salsa::tracked(jar = DecrJar)]
pub struct DeriveDecr {
    #[id]
    entity_path: EntityPath,
    traits: Vec<TraitExpr>,
    commas: Vec<CommaToken>,
}
