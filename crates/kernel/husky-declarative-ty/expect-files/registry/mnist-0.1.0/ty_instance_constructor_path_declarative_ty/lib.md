[
    (
        TypePath(`mnist::MnistLabel`, `Enum`),
        Err(
            DeclarativeTypeError::Original(
                OriginalDeclarativeTypeError::EnumTypeNoConstructor,
            ),
        ),
    ),
    (
        TypePath(`mnist::BinaryImage28`, `Extern`),
        Err(
            DeclarativeTypeError::Original(
                OriginalDeclarativeTypeError::ExternTypeHasNoConstructor,
            ),
        ),
    ),
    (
        TypePath(`mnist::BinaryGrid28`, `Extern`),
        Err(
            DeclarativeTypeError::Original(
                OriginalDeclarativeTypeError::ExternTypeHasNoConstructor,
            ),
        ),
    ),
]