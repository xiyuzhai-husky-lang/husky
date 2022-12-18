# Prefix Opn

## Test#0

input

```husky
-x
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Unrecognized(
                    Identifier(
                        Word(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                ),
            ),
            range: [1:2, 1:3),
            base_scope_result: Uncertain,
        },
        Expr {
            variant: Opn {
                opn_variant: Prefix(
                    Minus,
                ),
                opds: ArenaIdxRange(
                    0..1,
                ),
            },
            range: [1:1, 1:3),
            base_scope_result: None,
        },
    ],
}
```

## Test#1

input

```husky
!x
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Unrecognized(
                    Identifier(
                        Word(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                ),
            ),
            range: [1:2, 1:3),
            base_scope_result: Uncertain,
        },
        Expr {
            variant: Opn {
                opn_variant: Prefix(
                    Not,
                ),
                opds: ArenaIdxRange(
                    0..1,
                ),
            },
            range: [1:1, 1:3),
            base_scope_result: None,
        },
    ],
}
```
