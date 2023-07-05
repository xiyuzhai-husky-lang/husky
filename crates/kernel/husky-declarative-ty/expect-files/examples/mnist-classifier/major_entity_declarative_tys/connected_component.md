[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::hole_tmpl`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
            ),
        ),
        Ok(
            DeclarativeTerm(`Type`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::horizontal_extend`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
]