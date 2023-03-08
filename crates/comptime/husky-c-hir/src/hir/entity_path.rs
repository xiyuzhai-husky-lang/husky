use super::*;

pub enum CEntityPathHir {
    Enum(CIdent),
    Struct(CIdent),
    Function(CIdent),
    Alias(CIdent),
}

pub struct CEntityPathHirMenu {}
