[
    (
        FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
        Ok(
            Term(`Fp(Ref TermLiteral::EvalLifetime ConcaveComponent) -> Option f32`),
        ),
    ),
    (
        FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
        Err(
            Derived(
                SignatureError,
            ),
        ),
    ),
]