# Keyword

## Test#0

input

```husky
def
```

output

```husky
[
    Token {
        range: [1:1, 1:4),
        kind: Identifier(
            Identifier(
                Id {
                    value: 1,
                },
            ),
        ),
    },
]
```

## Test#1

input

```husky
func
```

output

```husky
[
    Token {
        range: [1:1, 1:5),
        kind: Identifier(
            Identifier(
                Id {
                    value: 1,
                },
            ),
        ),
    },
]
```

## Test#2

input

```husky
proc
```

output

```husky
[
    Token {
        range: [1:1, 1:5),
        kind: Identifier(
            Identifier(
                Id {
                    value: 1,
                },
            ),
        ),
    },
]
```
