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
    token_verses: TokenVerses {
        main_sequence: MainTokenVerseSequence {
            verses_data: [
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            1,
                        ),
                    ),
                    indent: 0,
                },
            ],
        },
        nested_sequences: [],
    },
}
```
