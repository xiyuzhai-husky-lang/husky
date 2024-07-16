```rust
[
    (
        TypePath(`malamute::Class`, `Enum`),
        [
            (
                TypeVariantPath(`malamute::Class::Known`),
                Ok(
                    DecTerm(`(independent (t: Type) -> fn((t) -> malamute::Class t`),
                ),
            ),
            (
                TypeVariantPath(`malamute::Class::Unknown`),
                Ok(
                    DecTerm(`(independent (t: Type) -> malamute::Class t`),
                ),
            ),
        ],
    ),
    (
        TypePath(`malamute::OneVsAll`, `Enum`),
        [
            (
                TypeVariantPath(`malamute::OneVsAll::Yes`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAll s a`),
                ),
            ),
            (
                TypeVariantPath(`malamute::OneVsAll::No`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAll s a`),
                ),
            ),
        ],
    ),
    (
        TypePath(`malamute::OneVsAllResult`, `Enum`),
        [
            (
                TypeVariantPath(`malamute::OneVsAllResult::ConfidentYes`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
                ),
            ),
            (
                TypeVariantPath(`malamute::OneVsAllResult::ConfidentNo`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
                ),
            ),
            (
                TypeVariantPath(`malamute::OneVsAllResult::Unconfident`),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
                ),
            ),
        ],
    ),
]
```