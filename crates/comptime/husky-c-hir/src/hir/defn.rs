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
    Value(CValueDefnHir),
}

pub struct CFunctionDefnHir {}
pub struct CValueDefnHir {}
