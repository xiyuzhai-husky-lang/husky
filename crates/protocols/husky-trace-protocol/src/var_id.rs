use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VarIdPresentation {
    data: String,
}

impl VarIdPresentation {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl VarIdPresentation {
    pub fn data(&self) -> &str {
        &self.data
    }
}
