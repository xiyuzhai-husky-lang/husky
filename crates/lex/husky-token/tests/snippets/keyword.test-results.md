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
