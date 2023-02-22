[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
            ),
        ),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Ref 'eval ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
            ),
        ),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
]