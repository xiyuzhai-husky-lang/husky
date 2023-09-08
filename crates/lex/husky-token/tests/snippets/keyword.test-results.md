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
        Token::Keyword(
            Keyword::Fugitive(
                Def,
            ),
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Ident(
            `func`,
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Ident(
            `proc`,
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
        Token::Keyword(
            Keyword::Trait,
        ),
    ],
    token_group_bases: [
        TokenGroupStartingTokenIdx(
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
