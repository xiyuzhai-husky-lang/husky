# Literal

## Test#0

input

```husky
"haha"
```

output

```husky
RangedTokenSheet {
    tokens: [
        Literal(
            String(
                StringLiteral(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ],
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:7),
    ],
    comments: [],
}
```

## Test#1

input

```husky
"\n"
```

output

```husky
RangedTokenSheet {
    tokens: [
        Literal(
            String(
                StringLiteral(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ],
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    comments: [],
}
```

## Test#2

input

```husky
"\t"
```

output

```husky
RangedTokenSheet {
    tokens: [
        Literal(
            String(
                StringLiteral(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ],
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    comments: [],
}
```

## Test#3

input

```husky
"\\"
```

output

```husky
RangedTokenSheet {
    tokens: [
        Literal(
            String(
                StringLiteral(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ],
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    comments: [],
}
```

## Test#4

input

```husky
"\""
```

output

```husky
RangedTokenSheet {
    tokens: [
        Literal(
            String(
                StringLiteral(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ],
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    comments: [],
}
```

## Test#5

input

```husky
"\r"
```

output

```husky
RangedTokenSheet {
    tokens: [
        Literal(
            String(
                StringLiteral(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ],
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    comments: [],
}
```
