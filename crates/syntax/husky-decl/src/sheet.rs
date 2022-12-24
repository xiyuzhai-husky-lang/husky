use crate::*;
use husky_entity_path::EntityPath;
use vec_like::VecPairMap;

#[derive(Debug, PartialEq, Eq)]
pub struct DeclSheet {
    decls: VecPairMap<EntityPath, DeclResult<Decl>>,
}

impl DeclSheet {
    pub fn new(decls: VecPairMap<EntityPath, DeclResult<Decl>>) -> Self {
        Self { decls }
    }

    pub fn decls(&self) -> &VecPairMap<EntityPath, DeclResult<Decl>> {
        &self.decls
    }
}
