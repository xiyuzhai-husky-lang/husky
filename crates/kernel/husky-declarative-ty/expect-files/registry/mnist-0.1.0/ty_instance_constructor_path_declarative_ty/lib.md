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
        TypePath(`mnist::BinaryImage28`, `Struct`),
        Ok(
            DeclarativeTerm(`fn(([DeclarativeTermLiteralTodo, ] core::raw_bits::r32) -> mnist::BinaryImage28`),
        ),
    ),
    (
        TypePath(`mnist::BinaryGrid28`, `Struct`),
        Ok(
            DeclarativeTerm(`fn(([DeclarativeTermLiteralTodo, ] core::raw_bits::r32) -> mnist::BinaryGrid28`),
        ),
    ),
]