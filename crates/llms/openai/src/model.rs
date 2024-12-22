#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenaiModel {
    Gpt4o,
}

impl OpenaiModel {
    pub fn as_str(&self) -> &str {
        match self {
            OpenaiModel::Gpt4o => "gpt-4o",
        }
    }
}
