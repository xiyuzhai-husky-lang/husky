[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} mnist_classifier::fermi::FermiMatchResult`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} mnist_classifier::fermi::FermiMatchResult`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
            ),
        ),
        Ok(
            DeclarativeTerm(`{val_type} core::option::Option mnist::MnistLabel`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
            ),
        ),
        Err(
            DeclarativeTypeError::Derived(
                DerivedDeclarativeTypeError::SignatureError,
            ),
        ),
    ),
]