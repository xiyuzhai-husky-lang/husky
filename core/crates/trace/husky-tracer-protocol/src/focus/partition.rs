use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Partition {
    pub name: String,
    pub ncol: usize,
    pub variant: PartitionVariant,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum PartitionVariant {
    Label(Label),
    LabelSet(Vec<Label>),
}

impl Partition {
    pub fn contains(&self, label: Label) -> bool {
        todo!()
    }
}
