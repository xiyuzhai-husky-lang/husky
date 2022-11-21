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
            range: [1:2, 1:3),
            base_scope_result: Uncertain,
        },
        Expr {
            variant: Opn {
                opn_variant: Prefix(
                    Minus,
                ),
                opds: 0..1,
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
            range: [1:2, 1:3),
            base_scope_result: Uncertain,
        },
        Expr {
            variant: Opn {
                opn_variant: Prefix(
                    Not,
                ),
                opds: 0..1,
            },
            range: [1:1, 1:3),
            base_scope_result: None,
        },
    ],
}
```
