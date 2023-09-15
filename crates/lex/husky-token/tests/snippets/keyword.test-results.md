# Keyword

## Test#0

input

```husky
def
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Fugitive(
                Def,
            ),
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
    ],
    indents: [
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
TokenSheetData {
    tokens: [
        TokenData::Ident(
            `func`,
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
    ],
    indents: [
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
TokenSheetData {
    tokens: [
        TokenData::Ident(
            `proc`,
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
    ],
    indents: [
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
TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Trait,
        ),
    ],
    token_group_starts: [
        TokenGroupStart(
            TokenIdx(
                1,
            ),
        ),
    ],
    indents: [
        0,
    ],
}
```
