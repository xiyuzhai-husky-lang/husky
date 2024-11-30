use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FigureTextChunkBaseData {
    pub text: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct FigureTextChunkBaseId(ShiftedU32);

impl Serialize for FigureTextChunkBaseId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.0.into())
    }
}

impl<'de> Deserialize<'de> for FigureTextChunkBaseId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = usize::deserialize(deserializer)?;
        Ok(FigureTextChunkBaseId(ShiftedU32::new(value as u32)))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FigureTextChunkBaseArena {
    data: Vec<FigureTextChunkBaseData>,
}

impl FigureTextChunkBaseArena {
    pub fn alloc(&mut self, text: FigureTextChunkBaseData) -> FigureTextChunkBaseId {
        let id = FigureTextChunkBaseId(ShiftedU32::new(self.data.len() as u32));
        self.data.push(text);
        id
    }
}

impl std::ops::Index<FigureTextChunkBaseId> for FigureTextChunkBaseArena {
    type Output = FigureTextChunkBaseData;

    fn index(&self, index: FigureTextChunkBaseId) -> &Self::Output {
        &self.data[index.0.index()]
    }
}
