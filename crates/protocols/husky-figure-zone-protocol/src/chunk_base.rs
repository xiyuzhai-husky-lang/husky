pub mod text;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use shifted_unsigned_int::ShiftedU32;
use text::FigureTextChunkBaseId;

// TODO: maybe move this to other crates
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum FigureChunkBase {
    Text(FigureTextChunkBaseId),
}
