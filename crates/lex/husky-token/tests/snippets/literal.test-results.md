# Literal

## Test#0

input

```husky
"haha"
```

output

```husky
[
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
]
```

## Test#1

input

```husky
"\n"
```

output

```husky
[
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
]
```

## Test#2

input

```husky
"\t"
```

output

```husky
[
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
]
```

## Test#3

input

```husky
"\\"
```

output

```husky
[
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
]
```

## Test#4

input

```husky
"\""
```

output

```husky
[
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
]
```

## Test#5

input

```husky
"\r"
```

output

```husky
[
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
]
```
