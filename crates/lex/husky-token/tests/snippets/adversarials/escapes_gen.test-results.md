# Escapes Gen

## Test#0

input

```husky
"\a"
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Error(
            TokenDataError::UnexpectedCharAfterBackslash(
                'a',
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
