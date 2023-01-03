# Literal

## Test#0

input

```husky
"haha"
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:7),
            kind: Literal(
                String(
                    StringLiteral {
                        data: "haha",
                    },
                ),
            ),
        },
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
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:5),
            kind: Literal(
                String(
                    StringLiteral {
                        data: "\n",
                    },
                ),
            ),
        },
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
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:5),
            kind: Literal(
                String(
                    StringLiteral {
                        data: "\t",
                    },
                ),
            ),
        },
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
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:5),
            kind: Literal(
                String(
                    StringLiteral {
                        data: "\\",
                    },
                ),
            ),
        },
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
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:5),
            kind: Literal(
                String(
                    StringLiteral {
                        data: "\"",
                    },
                ),
            ),
        },
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
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:5),
            kind: Literal(
                String(
                    StringLiteral {
                        data: "\r",
                    },
                ),
            ),
        },
    ],
    group_starts: [
        0,
    ],
}
```
