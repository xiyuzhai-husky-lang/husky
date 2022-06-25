use super::*;

#[derive(Debug, Hash, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Label(pub usize);

impl From<u8> for Label {
    fn from(v: u8) -> Self {
        Self(v as usize)
    }
}
