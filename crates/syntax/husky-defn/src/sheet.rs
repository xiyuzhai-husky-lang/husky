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

    pub fn defns<'a>(&'a self) -> impl Iterator<Item = Defn> + 'a {
        self.defns.iter().map(|(_, defn)| *defn)
    }
}
