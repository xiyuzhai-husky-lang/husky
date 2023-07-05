[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} mnist_classifier::fermi::FermiMatchResult`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} core::option::Option mnist::MnistLabel`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::one::upmost`, `Fn`),
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
                FugitivePath(`mnist_classifier::digits::one::downmost`, `Fn`),
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
                FugitivePath(`mnist_classifier::digits::one::hat`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
]