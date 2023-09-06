use crate::*;

pub struct TokenRegion {}

pub fn token_region(path: RegionPath, db: &dyn EntitySynTreeDb) -> TokenRegion {
    match path {
        RegionPath::Snippet(_) => todo!(),
        RegionPath::Decr(_) => todo!(),
        RegionPath::Decl(_) => todo!(),
        RegionPath::Defn(_) => todo!(),
    }
}
