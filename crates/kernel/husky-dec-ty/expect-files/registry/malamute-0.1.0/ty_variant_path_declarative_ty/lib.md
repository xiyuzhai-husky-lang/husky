```rust
[
    (
        TypePath(`malamute::Class`, `Enum`),
        [
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 2,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`(independent (t: Type) -> fn((t) -> malamute::Class t`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 3,
                        },
                    ),
                ),
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
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 5,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAll s a`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 6,
                        },
                    ),
                ),
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
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 8,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 9,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
                ),
            ),
            (
                TypeVariantPath(
                    ItemPathId(
                        Id {
                            value: 10,
                        },
                    ),
                ),
                Ok(
                    DecTerm(`(independent (s: Type) -> (independent (a: s) -> malamute::OneVsAllResult s a`),
                ),
            ),
        ],
    ),
]
```