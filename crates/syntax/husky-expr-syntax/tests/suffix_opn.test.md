# Suffix Opn

## Test#0

input

```husky
x++
```

output

```husky
IdxArena {
    data: [
        RawExpr {
            variant: Atom(
                Symbol(
                    Symbol {
                        ident: Custom(
                            CustomIdentifier(
                                "x",
                            ),
                        ),
                        kind: Unrecognized,
                    },
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: Uncertain,
        },
        RawExpr {
            variant: Opn {
                opn_variant: Suffix(
                    Incr,
                ),
                opds: 0..1,
            },
            range: [1:1, 1:4),
            base_scope_result: None,
        },
    ],
}
```

## Test#1

input

```husky
y--
```

output

```husky
IdxArena {
    data: [
        RawExpr {
            variant: Atom(
                Symbol(
                    Symbol {
                        ident: Custom(
                            CustomIdentifier(
                                "y",
                            ),
                        ),
                        kind: Unrecognized,
                    },
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: Uncertain,
        },
        RawExpr {
            variant: Opn {
                opn_variant: Suffix(
                    Decr,
                ),
                opds: 0..1,
            },
            range: [1:1, 1:4),
            base_scope_result: None,
        },
    ],
}
```
