use super::*;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum PrimitiveVisual {
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
}
