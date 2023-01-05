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
                StringLiteral {
                    data: "haha",
                },
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:7),
    ],
    group_starts: [
        0,
    ],
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
                StringLiteral {
                    data: "\n",
                },
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    group_starts: [
        0,
    ],
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
                StringLiteral {
                    data: "\t",
                },
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    group_starts: [
        0,
    ],
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
                StringLiteral {
                    data: "\\",
                },
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    group_starts: [
        0,
    ],
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
                StringLiteral {
                    data: "\"",
                },
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    group_starts: [
        0,
    ],
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
                StringLiteral {
                    data: "\r",
                },
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:5),
    ],
    group_starts: [
        0,
    ],
}
```
