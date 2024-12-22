#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SglangModel {
    Llama3_1_8bInstruct,
}

impl SglangModel {
    pub fn as_str(&self) -> &str {
        match self {
            SglangModel::Llama3_1_8bInstruct => "llama-3.1-8b-instruct",
        }
    }
}
