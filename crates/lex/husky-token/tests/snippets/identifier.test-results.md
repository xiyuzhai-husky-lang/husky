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
