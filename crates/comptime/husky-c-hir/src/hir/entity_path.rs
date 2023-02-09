use super::*;

pub enum CEntityPathHir {
    Enum(CIdentifier),
    Struct(CIdentifier),
    Function(CIdentifier),
    Alias(CIdentifier),
}

pub struct CEntityPathHirMenu {}
