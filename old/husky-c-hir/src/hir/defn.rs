mod ty;

use super::*;

pub enum CDefnHir {
    Type(CTypeDefnHir),
    Form(CFormDefnHir),
}

pub enum CTypeDefnHir {
    Struct(CStructDefnHir),
    Enum(CEnumDefnHir),
    Union(CUnionDefnHir),
}

pub struct CStructDefnHir {}

pub struct CEnumDefnHir {}

pub struct CUnionDefnHir {}

pub enum CFormDefnHir {
    Function(CFunctionDefnHir),
    Value(CValDefnHir),
    Alias(CAliasDefnHir),
}

#[salsa::tracked(db = CHirDb, jar = CHirJar)]
pub struct CFunctionDefnHir {}

#[salsa::tracked(db = CHirDb, jar = CHirJar)]
pub struct CValDefnHir {}

#[salsa::tracked(db = CHirDb, jar = CHirJar)]
pub struct CAliasDefnHir {}
