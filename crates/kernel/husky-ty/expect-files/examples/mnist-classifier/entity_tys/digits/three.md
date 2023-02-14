[
    (
        FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::three::uparc`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::three::downarc`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::three::back`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
]