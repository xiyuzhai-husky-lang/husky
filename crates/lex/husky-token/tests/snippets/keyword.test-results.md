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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:4),
    ],
    comments: [],
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
    group_starts: [
        0,
    ],
    token_ranges: [
        [1:1, 1:6),
    ],
    comments: [],
}
```
