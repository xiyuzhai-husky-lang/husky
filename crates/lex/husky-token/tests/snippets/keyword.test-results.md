# Keyword

## Test#0

input

```husky
def
```

output

```husky
RangedTokenSheet {
    tokens: [
        Keyword(
            Paradigm(
                Def,
            ),
        ),
    ],
    token_ranges: [
        [1:1, 1:4),
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
RangedTokenSheet {
    tokens: [
        Keyword(
            Paradigm(
                Func,
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
proc
```

output

```husky
RangedTokenSheet {
    tokens: [
        Keyword(
            Paradigm(
                Proc,
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
trait
```

output

```husky
RangedTokenSheet {
    tokens: [
        Keyword(
            Trait,
        ),
    ],
    token_ranges: [
        [1:1, 1:6),
    ],
    group_starts: [
        0,
    ],
}
```
