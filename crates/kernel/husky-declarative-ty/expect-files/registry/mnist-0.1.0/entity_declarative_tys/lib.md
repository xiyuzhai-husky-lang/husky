[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist::MnistLabel`, `Enum`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist::BinaryImage28`, `Struct`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::TypeOntologyDeclError {
                    path: TypePath(`mnist::BinaryImage28`, `Struct`),
                },
            ),
        ),
    ),
]