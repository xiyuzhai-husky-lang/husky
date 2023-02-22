[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
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
                FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Ref 'eval ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
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
                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
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
                FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
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
                FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Ref 'eval ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Form(
                FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
            ),
        ),
        Ok(
            Term(`Fp(Ref 'eval ConcaveComponent) -> Option f32`),
        ),
    ),
]