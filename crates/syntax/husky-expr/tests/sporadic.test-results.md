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
                Unrecognized(
                    Identifier(
                        Id {
                            value: 1,
                        },
                    ),
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
                Unrecognized(
                    Identifier(
                        Id {
                            value: 1,
                        },
                    ),
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
                Unrecognized(
                    Identifier(
                        Id {
                            value: 1,
                        },
                    ),
                ),
            ),
            range: [1:1, 1:5),
            base_scope_result: Uncertain,
        },
    ],
}
```
