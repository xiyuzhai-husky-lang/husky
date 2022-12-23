Ok(
    TokenSheet {
        tokens: [
            Token {
                range: [1:1, 1:2),
                kind: Special(
                    PoundSign,
                ),
            },
            Token {
                range: [1:2, 1:3),
                kind: Special(
                    Bra(
                        Box,
                    ),
                ),
            },
            Token {
                range: [1:3, 1:7),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [1:7, 1:8),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [1:8, 1:10),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [1:10, 1:11),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [1:11, 1:12),
                kind: Special(
                    Ket(
                        Box,
                    ),
                ),
            },
            Token {
                range: [2:1, 2:4),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [2:5, 2:9),
                kind: Keyword(
                    Type(
                        Type,
                    ),
                ),
            },
            Token {
                range: [2:10, 2:12),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [2:12, 2:13),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [4:1, 4:5),
                kind: Keyword(
                    Impl,
                ),
            },
            Token {
                range: [4:6, 4:8),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [4:8, 4:9),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [5:5, 5:8),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [5:9, 5:11),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [5:12, 5:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [5:15, 5:16),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [5:16, 5:17),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [5:18, 5:20),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [5:21, 5:23),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [5:23, 5:24),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [7:1, 7:5),
                kind: Keyword(
                    Impl,
                ),
            },
            Token {
                range: [7:6, 7:10),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [7:10, 7:12),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [7:12, 7:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [7:15, 7:17),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [7:17, 7:20),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [7:20, 7:21),
                kind: Special(
                    LAngle,
                ),
            },
            Token {
                range: [7:21, 7:23),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [7:23, 7:24),
                kind: Special(
                    RAngle,
                ),
            },
            Token {
                range: [7:25, 7:28),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [7:29, 7:31),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [7:31, 7:32),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [8:5, 8:8),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [8:9, 8:11),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [8:12, 8:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [8:15, 8:16),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [8:16, 8:21),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [8:21, 8:22),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [8:23, 8:25),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [8:25, 8:26),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [8:27, 8:29),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [8:30, 8:32),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 19,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [8:32, 8:33),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [10:1, 10:2),
                kind: Special(
                    PoundSign,
                ),
            },
            Token {
                range: [10:2, 10:3),
                kind: Special(
                    Bra(
                        Box,
                    ),
                ),
            },
            Token {
                range: [10:3, 10:7),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [10:7, 10:8),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [10:8, 10:11),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [10:11, 10:12),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [10:12, 10:13),
                kind: Special(
                    Ket(
                        Box,
                    ),
                ),
            },
            Token {
                range: [11:1, 11:4),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [11:5, 11:9),
                kind: Keyword(
                    Type(
                        Type,
                    ),
                ),
            },
            Token {
                range: [11:10, 11:13),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [11:13, 11:14),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [13:1, 13:5),
                kind: Keyword(
                    Impl,
                ),
            },
            Token {
                range: [13:6, 13:9),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [13:9, 13:10),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [14:5, 14:8),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [14:9, 14:11),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [14:12, 14:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [14:15, 14:16),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [14:16, 14:17),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [14:18, 14:20),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [14:21, 14:24),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [14:24, 14:25),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [16:1, 16:5),
                kind: Keyword(
                    Impl,
                ),
            },
            Token {
                range: [16:6, 16:10),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [16:10, 16:12),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [16:12, 16:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [16:15, 16:17),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [16:17, 16:20),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [16:20, 16:21),
                kind: Special(
                    LAngle,
                ),
            },
            Token {
                range: [16:21, 16:24),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [16:24, 16:25),
                kind: Special(
                    RAngle,
                ),
            },
            Token {
                range: [16:26, 16:29),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [16:30, 16:33),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [16:33, 16:34),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [17:5, 17:8),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [17:9, 17:11),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [17:12, 17:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [17:15, 17:16),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [17:16, 17:21),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [17:21, 17:22),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [17:23, 17:26),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [17:26, 17:27),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [17:28, 17:30),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [17:31, 17:34),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [17:34, 17:35),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [19:1, 19:2),
                kind: Special(
                    PoundSign,
                ),
            },
            Token {
                range: [19:2, 19:3),
                kind: Special(
                    Bra(
                        Box,
                    ),
                ),
            },
            Token {
                range: [19:3, 19:7),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [19:7, 19:8),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [19:8, 19:11),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [19:11, 19:12),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [19:12, 19:13),
                kind: Special(
                    Ket(
                        Box,
                    ),
                ),
            },
            Token {
                range: [20:1, 20:4),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [20:5, 20:9),
                kind: Keyword(
                    Type(
                        Type,
                    ),
                ),
            },
            Token {
                range: [20:10, 20:13),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [20:13, 20:14),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [22:1, 22:5),
                kind: Keyword(
                    Impl,
                ),
            },
            Token {
                range: [22:6, 22:9),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [22:9, 22:10),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [23:5, 23:8),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [23:9, 23:11),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [23:12, 23:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [23:15, 23:16),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [23:16, 23:17),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [23:18, 23:20),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [23:21, 23:24),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [23:24, 23:25),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [25:1, 25:5),
                kind: Keyword(
                    Impl,
                ),
            },
            Token {
                range: [25:6, 25:10),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [25:10, 25:12),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [25:12, 25:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [25:15, 25:17),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [25:17, 25:20),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [25:20, 25:21),
                kind: Special(
                    LAngle,
                ),
            },
            Token {
                range: [25:21, 25:24),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [25:24, 25:25),
                kind: Special(
                    RAngle,
                ),
            },
            Token {
                range: [25:26, 25:29),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [25:30, 25:33),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [25:33, 25:34),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [26:5, 26:7),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [26:8, 26:11),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [26:11, 26:12),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [26:12, 26:17),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [26:17, 26:18),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [26:19, 26:22),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [26:22, 26:23),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [26:24, 26:26),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [26:27, 26:30),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [26:30, 26:31),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [28:1, 28:2),
                kind: Special(
                    PoundSign,
                ),
            },
            Token {
                range: [28:2, 28:3),
                kind: Special(
                    Bra(
                        Box,
                    ),
                ),
            },
            Token {
                range: [28:3, 28:7),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 18,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [28:7, 28:8),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [28:8, 28:11),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [28:11, 28:12),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [28:12, 28:13),
                kind: Special(
                    Ket(
                        Box,
                    ),
                ),
            },
            Token {
                range: [29:1, 29:5),
                kind: Keyword(
                    Type(
                        Type,
                    ),
                ),
            },
            Token {
                range: [29:6, 29:9),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [29:9, 29:10),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [31:1, 31:5),
                kind: Keyword(
                    Impl,
                ),
            },
            Token {
                range: [31:6, 31:9),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [31:9, 31:10),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [32:5, 32:8),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [32:9, 32:11),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [32:12, 32:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 20,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [32:15, 32:16),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [32:16, 32:17),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [32:18, 32:20),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [32:21, 32:24),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [32:24, 32:25),
                kind: Special(
                    Semicolon,
                ),
            },
            Token {
                range: [34:1, 34:5),
                kind: Keyword(
                    Impl,
                ),
            },
            Token {
                range: [34:6, 34:10),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 21,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [34:10, 34:12),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [34:12, 34:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [34:15, 34:17),
                kind: Special(
                    BinaryOpr(
                        ScopeResolution,
                    ),
                ),
            },
            Token {
                range: [34:17, 34:20),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 22,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [34:20, 34:21),
                kind: Special(
                    LAngle,
                ),
            },
            Token {
                range: [34:21, 34:24),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [34:24, 34:25),
                kind: Special(
                    RAngle,
                ),
            },
            Token {
                range: [34:26, 34:29),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 23,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [34:30, 34:33),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [34:33, 34:34),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [35:5, 35:8),
                kind: Decorator(
                    Pub,
                ),
            },
            Token {
                range: [35:9, 35:11),
                kind: Keyword(
                    Paradigm(
                        Fn,
                    ),
                ),
            },
            Token {
                range: [35:12, 35:15),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 24,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [35:15, 35:16),
                kind: Special(
                    Bra(
                        Par,
                    ),
                ),
            },
            Token {
                range: [35:16, 35:21),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 25,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [35:21, 35:22),
                kind: Special(
                    Colon,
                ),
            },
            Token {
                range: [35:23, 35:26),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [35:26, 35:27),
                kind: Special(
                    Ket(
                        Par,
                    ),
                ),
            },
            Token {
                range: [35:28, 35:30),
                kind: Special(
                    BinaryOpr(
                        Curry,
                    ),
                ),
            },
            Token {
                range: [35:31, 35:34),
                kind: Identifier(
                    Identifier(
                        Word(
                            Id {
                                value: 28,
                            },
                        ),
                    ),
                ),
            },
            Token {
                range: [35:34, 35:35),
                kind: Special(
                    Semicolon,
                ),
            },
        ],
        group_starts: [
            0,
            7,
            11,
            14,
            22,
            34,
            45,
            52,
            56,
            59,
            67,
            79,
            90,
            97,
            101,
            104,
            112,
            124,
            134,
            141,
            144,
            147,
            155,
            167,
        ],
    },
)