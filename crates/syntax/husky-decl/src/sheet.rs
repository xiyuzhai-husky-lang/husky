use crate::*;
use husky_entity_path::EntityPath;
use vec_like::VecPairMap;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclSheet {
    decls: VecPairMap<EntityPath, Decl>,
}
