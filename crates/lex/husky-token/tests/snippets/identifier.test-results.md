# Identifier

## Test#0

input

```husky
self
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Pronoun(
                SelfValue,
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
Self
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Pronoun(
                SelfType,
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
