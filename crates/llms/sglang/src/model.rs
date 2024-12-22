use enum_index::IsEnumIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IsEnumIndex)]
#[repr(u8)]
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
