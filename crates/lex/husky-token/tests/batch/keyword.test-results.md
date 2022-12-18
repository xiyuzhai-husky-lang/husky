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
                Def,
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
                Func,
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
                Proc,
            ),
        ),
    },
]
```

## Test#3

input

```husky
trait
```

output

```husky
[
    Token {
        range: [1:1, 1:6),
        kind: Keyword(
            Trait,
        ),
    },
]
```
