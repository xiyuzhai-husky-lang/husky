[
    (
        FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::one::hat`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
]