pub mod virtual_var_id;

use crate::*;
use serde::{Deserialize, Serialize};

pub trait IsVarId: std::fmt::Debug + Copy + Eq + std::hash::Hash + Send + Sync + 'static {}

pub trait IsVarIdFull: IsVarId + Serialize + for<'a> Deserialize<'a> {}

impl<VarId> IsVarIdFull for VarId where VarId: IsVarId + Serialize + for<'a> Deserialize<'a> {}
