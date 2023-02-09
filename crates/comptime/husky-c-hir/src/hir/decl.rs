pub enum CDeclHir {
    Type(CTypeDeclHir),
    Form(CFormDeclHir),
}

pub enum CTypeDeclHir {
    Struct(CStructDeclHir),
    Enum(CEnumDeclHir),
    Union(CUnionDeclHir),
}

pub struct CStructDeclHir {}

pub struct CEnumDeclHir {}

pub struct CUnionDeclHir {}

pub enum CFormDeclHir {
    Function(CFunctionDeclHir),
    Value(CValueDeclHir),
}

pub struct CFunctionDeclHir {}

pub struct CValueDeclHir {}
