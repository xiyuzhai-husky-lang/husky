[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} mnist_classifier::fermi::FermiMatchResult`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
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
                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} core::option::Option mnist::MnistLabel`),
        ),
    ),
]