# Literal

## Test#0

input

```husky
"haha"
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Literal(
            LiteralTokenData::String(
                StringLiteralTokenData {
                    data: "haha",
                },
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
"\n"
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Literal(
            LiteralTokenData::String(
                StringLiteralTokenData {
                    data: "\n",
                },
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

## Test#2

input

```husky
"\t"
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Literal(
            LiteralTokenData::String(
                StringLiteralTokenData {
                    data: "\t",
                },
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

## Test#3

input

```husky
"\\"
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Literal(
            LiteralTokenData::String(
                StringLiteralTokenData {
                    data: "\\",
                },
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

## Test#4

input

```husky
"\""
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Literal(
            LiteralTokenData::String(
                StringLiteralTokenData {
                    data: "\"",
                },
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

## Test#5

input

```husky
"\r"
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Literal(
            LiteralTokenData::String(
                StringLiteralTokenData {
                    data: "\r",
                },
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
