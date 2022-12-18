# Binary Opn

## Test#0

input

```husky
1 + 1
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    Integer(
                        1,
                    ),
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: None,
        },
        Expr {
            variant: Atom(
                Literal(
                    Integer(
                        1,
                    ),
                ),
            ),
            range: [1:5, 1:6),
            base_scope_result: None,
        },
        Expr {
            variant: Opn {
                opn_variant: Binary(
                    PureClosed(
                        Add,
                    ),
                ),
                opds: ArenaIdxRange(
                    0..2,
                ),
            },
            range: [1:1, 1:6),
            base_scope_result: None,
        },
    ],
}
```

## Test#1

input

```husky
1 - 1
```

output

```husky
Arena {
    data: [
        Expr {
            variant: Atom(
                Literal(
                    Integer(
                        1,
                    ),
                ),
            ),
            range: [1:1, 1:2),
            base_scope_result: None,
        },
        Expr {
            variant: Atom(
                Literal(
                    Integer(
                        1,
                    ),
                ),
            ),
            range: [1:5, 1:6),
            base_scope_result: None,
        },
        Expr {
            variant: Opn {
                opn_variant: Binary(
                    PureClosed(
                        Sub,
                    ),
                ),
                opds: ArenaIdxRange(
                    0..2,
                ),
            },
            range: [1:1, 1:6),
            base_scope_result: None,
        },
    ],
}
```
