use vec_like::VecPairMap;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct DefnSheet {
    defns: VecPairMap<EntityPath, Defn>,
}
