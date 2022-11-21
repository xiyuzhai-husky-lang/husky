# Sporadic

## Test#0

input

```husky
haha
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
                                "haha",
                            ),
                        ),
                        kind: Unrecognized,
                    },
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: Uncertain,
        },
    ],
}
```

## Test#1

input

```husky
bt
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
                                "bt",
                            ),
                        ),
                        kind: Unrecognized,
                    },
                ),
            ),
            range: [1:1, 1:3),
            base_scope_result: Uncertain,
        },
    ],
}
```

## Test#2

input

```husky
what
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
                                "what",
                            ),
                        ),
                        kind: Unrecognized,
                    },
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: Uncertain,
        },
    ],
}
```
