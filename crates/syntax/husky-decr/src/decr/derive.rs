use super::*;

#[salsa::tracked(db = DecrDb, jar = DecrJar)]
pub struct DeriveDecr {
    #[id]
    entity_path: EntityPath,
    traits: Vec<TraitExpr>,
    commas: Vec<CommaToken>,
}
