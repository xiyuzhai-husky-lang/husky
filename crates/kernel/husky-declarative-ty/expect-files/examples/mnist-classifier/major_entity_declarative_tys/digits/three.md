[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} mnist_classifier::fermi::FermiMatchResult`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} core::option::Option mnist::MnistLabel`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
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
                FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
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
                FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
]