use vec_like::VecPairMap;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DefnSheet {
    defns: VecPairMap<EntityPath, Defn>,
}

impl DefnSheet {
    pub fn new(defns: VecPairMap<EntityPath, Defn>) -> Self {
        Self { defns }
    }

    pub fn defns(&self) -> &VecPairMap<EntityPath, Defn> {
        &self.defns
    }
}
