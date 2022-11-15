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
        kind: Keyword(
            Paradigm(
                LazyFunctional,
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
        kind: Keyword(
            Paradigm(
                EagerFunctional,
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
        kind: Keyword(
            Paradigm(
                EagerProcedural,
            ),
        ),
    },
]
```
