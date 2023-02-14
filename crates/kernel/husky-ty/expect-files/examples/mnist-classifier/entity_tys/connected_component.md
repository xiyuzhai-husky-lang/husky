[
    (
        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
        Ok(
            Term(`Type`),
        ),
    ),
    (
        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime RawContour) -> Option f32`),
        ),
    ),
    (
        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Function`),
        Ok(
            Term(`Fp(r32, r32) -> r32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
]