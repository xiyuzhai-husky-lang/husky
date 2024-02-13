TokenSheetData {
    tokens: [
        TokenData::Keyword(
            Keyword::Mod,
        ),
        TokenData::Ident(
            `submodule_name`,
        ),
        TokenData::Keyword(
            Keyword::TypeEntity(
                Struct,
            ),
        ),
        TokenData::Ident(
            `A`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Bra(
                    Bracket::Curl,
                ),
            ),
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Ket(
                    Bracket::Curl,
                ),
            ),
        ),
        TokenData::Keyword(
            Keyword::Impl,
        ),
        TokenData::Ident(
            `A`,
        ),
        TokenData::Punctuation(
            Punctuation(
                PunctuationMapped::Colon,
            ),
        ),
        TokenData::Keyword(
            Keyword::Mod,
        ),
        TokenData::Ident(
            `submodule_name`,
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
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            3,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            7,
                        ),
                    ),
                    indent: 0,
                },
                TokenVerseData {
                    start: TokenVerseStart(
                        TokenIdx(
                            10,
                        ),
                    ),
                    indent: 4,
                },
            ],
        },
        nested_sequences: [],
    },
}