# Keyword

## Test#0

input

```husky
def
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:4),
            kind: Keyword(
                Paradigm(
                    Def,
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
func
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:5),
            kind: Keyword(
                Paradigm(
                    Func,
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
proc
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:5),
            kind: Keyword(
                Paradigm(
                    Proc,
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
trait
```

output

```husky
TokenSheet {
    tokens: [
        Token {
            range: [1:1, 1:6),
            kind: Keyword(
                Trait,
            ),
        },
    ],
    group_starts: [
        0,
    ],
}
```
