use super::TracePath;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceTracePathData {
    biological_parent_path: TracePath,
    place_access: PlaceAccess,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlaceAccess {
    StructField,
    EnumVariantField,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceTraceData {}
