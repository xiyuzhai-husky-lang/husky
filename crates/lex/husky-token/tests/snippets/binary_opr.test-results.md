# Binary Opr

## Test#0

input

```husky
+
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Closed(
                        BinaryClosedOpr::Add,
                    ),
                ),
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
-
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Minus,
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
*
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Star,
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
/
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Closed(
                        BinaryClosedOpr::Div,
                    ),
                ),
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
**
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Closed(
                        BinaryClosedOpr::Power,
                    ),
                ),
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
|
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Vertical,
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

## Test#6

input

```husky
||
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::DoubleVertical,
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

## Test#7

input

```husky
&&
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::ShortCircuitLogic(
                        BinaryShortcuitLogicOpr::And,
                    ),
                ),
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

## Test#8

input

```husky
==
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Comparison(
                        BinaryComparisonOpr::Eq,
                    ),
                ),
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

## Test#9

input

```husky
!=
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Comparison(
                        BinaryComparisonOpr::Neq,
                    ),
                ),
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

## Test#10

input

```husky
>
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::RaOrGt,
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

## Test#11

input

```husky
>=
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Comparison(
                        BinaryComparisonOpr::Geq,
                    ),
                ),
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

## Test#12

input

```husky
<
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::LaOrLt,
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

## Test#13

input

```husky
<=
```

output

```husky
TokenSheetData {
    tokens: [
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Binary(
                    SynBinaryOpr::Comparison(
                        BinaryComparisonOpr::Leq,
                    ),
                ),
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
