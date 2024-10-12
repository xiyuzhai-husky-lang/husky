use super::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VirtualVarId;

impl IsVarId for () {
    fn generalize(self) -> Option<Self> {
        None
    }
}
