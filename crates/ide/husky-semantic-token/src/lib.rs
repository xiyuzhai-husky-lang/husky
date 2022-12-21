use husky_entity_kind::EntityKind;
use husky_text::TextRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsSemanticToken {
    pub kind: SemanticTokenKind,
    pub range: TextRange,
    pub token_type: u32,
    pub token_modifiers_bitset: u32,
}

impl AbsSemanticToken {
    pub fn new(kind: SemanticTokenKind, range: TextRange) -> Self {
        Self {
            kind,
            range,
            token_type: kind.token_type(),
            token_modifiers_bitset: kind.token_modifiers_bitset(),
        }
    }

    pub fn to_semantic_tokens(abs_semantic_tokens: &[AbsSemanticToken]) -> Vec<SemanticToken> {
        let mut semantic_tokens = vec![];
        let mut last_line = 0;
        let mut last_start = 0;
        for abs_semantic_token in abs_semantic_tokens {
            let new_line = abs_semantic_token.range.start.i();
            let new_start = abs_semantic_token.range.start.j();
            let length = abs_semantic_token.range.end.j() - new_start;
            let delta_line = new_line - last_line;
            let delta_start = if new_line > last_line {
                new_start
            } else {
                new_start - last_start
            };
            semantic_tokens.push(SemanticToken {
                delta_line,
                delta_start,
                length,
                token_type: abs_semantic_token.token_type,
                token_modifiers_bitset: abs_semantic_token.token_modifiers_bitset,
            });
            last_line = new_line;
            last_start = new_start
        }
        semantic_tokens
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticTokenKind {
    Keyword(Keyword),
    Field,
    Special,
    Parameter,
    Variable,
    ThisValue,
    FrameVariable,
    Entity(EntityKind),
    GenericPlaceholder,
    EnumVariant,
    Method,
    Literal,
    XmlTagKind,
    WordPattern,
}

impl SemanticTokenKind {
    pub fn token_type(self) -> u32 {
        todo!()
        // get_type_index(match self {
        //     SemanticTokenKind::Keyword(_) => SemanticTokenType::KEYWORD,
        //     SemanticTokenKind::Field => SemanticTokenType::PROPERTY,
        //     SemanticTokenKind::Special => SemanticTokenType::OPERATOR,
        //     SemanticTokenKind::Variable => SemanticTokenType::VARIABLE,
        //     SemanticTokenKind::ThisValue => SemanticTokenType::VARIABLE,
        //     SemanticTokenKind::FrameVariable => SemanticTokenType::VARIABLE,
        //     SemanticTokenKind::Entity(husky_entity_kind) => match husky_entity_kind {
        //         EntityKind::Module => SemanticTokenType::NAMESPACE,
        //         EntityKind::Type(_) => SemanticTokenType::TYPE,
        //         EntityKind::Trait => SemanticTokenType::TYPE,
        //         EntityKind::Member(member_kind) => match member_kind {
        //             MemberKind::Method { .. } => SemanticTokenType::METHOD,
        //             MemberKind::Call => todo!(),
        //             MemberKind::TraitAssociatedType => todo!(),
        //             MemberKind::TraitAssociatedConstSize => todo!(),
        //             MemberKind::Field => todo!(),
        //             MemberKind::TraitAssociatedAny => panic!(),
        //         },
        //         EntityKind::Function { .. } => SemanticTokenType::FUNCTION,
        //         EntityKind::Feature => SemanticTokenType::VARIABLE,
        //         EntityKind::EnumVariant => SemanticTokenType::VARIABLE,
        //         EntityKind::Main => panic!(),
        //         EntityKind::Crate(_) => todo!(),
        //     },
        //     SemanticTokenKind::GenericPlaceholder => SemanticTokenType::TYPE_PARAMETER,
        //     SemanticTokenKind::Parameter => SemanticTokenType::PARAMETER,
        //     SemanticTokenKind::EnumVariant => SemanticTokenType::ENUM_MEMBER,
        //     SemanticTokenKind::Method => SemanticTokenType::METHOD,
        //     SemanticTokenKind::Literal => SemanticTokenType::NUMBER,
        //     SemanticTokenKind::XmlTagKind => SemanticTokenType::FUNCTION,
        //     SemanticTokenKind::WordPattern => SemanticTokenType::ENUM_MEMBER,
        // })
    }

    pub fn token_modifiers_bitset(self) -> u32 {
        let mut result = ModifierSet(0);
        match self {
            SemanticTokenKind::Keyword(keyword) => match keyword {
                Keyword::Stmt(stmt_keyword) => match stmt_keyword {
                    StmtKeyword::If
                    | StmtKeyword::Elif
                    | StmtKeyword::Else
                    | StmtKeyword::Match
                    | StmtKeyword::For
                    | StmtKeyword::ForExt
                    | StmtKeyword::While
                    | StmtKeyword::Do
                    | StmtKeyword::Break
                    | StmtKeyword::Return
                    | StmtKeyword::Require => result |= CONTROL_FLOW,
                    StmtKeyword::Let | StmtKeyword::Var | StmtKeyword::Assert => (),
                },
                _ => (),
            },
            _ => (),
        }
        result.0
    }
}

use husky_token::{Keyword, StmtKeyword};
use lsp_types::{
    Range, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensEdit,
};

macro_rules! define_semantic_token_types {
    ($(($ident:ident, $string:literal)),*$(,)?) => {
        $(pub  const $ident: SemanticTokenType = SemanticTokenType::new($string);)*

        pub const SUPPORTED_TYPES: &[SemanticTokenType] = &[
            SemanticTokenType::COMMENT,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::REGEXP,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::TYPE,
            SemanticTokenType::STRUCT,
            SemanticTokenType::CLASS,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::ENUM,
            SemanticTokenType::ENUM_MEMBER,
            SemanticTokenType::TYPE_PARAMETER,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::METHOD,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::MACRO,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::PARAMETER,
            $($ident),*
        ];
    };
}

define_semantic_token_types![
    (ANGLE, "angle"),
    (ARITHMETIC, "arithmetic"),
    (ATTRIBUTE, "attribute"),
    (BITWISE, "bitwise"),
    (BOOLEAN, "boolean"),
    (BRACE, "brace"),
    (BRACKET, "bracket"),
    (BUILTIN_ATTRIBUTE, "builtinAttribute"),
    (BUILTIN_TYPE, "builtinType"),
    (CHAR, "character"),
    (COLON, "colon"),
    (COMMA, "comma"),
    (COMPARISON, "comparison"),
    (CONST_PARAMETER, "constParameter"),
    (DOT, "dot"),
    (ESCAPE_SEQUENCE, "escapeSequence"),
    (FORMAT_SPECIFIER, "formatSpecifier"),
    (GENERIC, "generic"),
    (LABEL, "label"),
    (LIFETIME, "lifetime"),
    (LOGICAL, "logical"),
    (OPERATOR, "operator"),
    (PARENTHESIS, "parenthesis"),
    (PUNCTUATION, "punctuation"),
    (SELF_KEYWORD, "selfKeyword"),
    (SEMICOLON, "semicolon"),
    (TYPE_ALIAS, "typeAlias"),
    (UNION, "union"),
    (UNRESOLVED_REFERENCE, "unresolvedReference"),
];

macro_rules! define_semantic_token_modifiers {
    ($(($ident:ident, $string:literal)),*$(,)?) => {
        $(pub const $ident: SemanticTokenModifier = SemanticTokenModifier::new($string);)*

        pub const SUPPORTED_MODIFIERS: &[SemanticTokenModifier] = &[
            SemanticTokenModifier::DOCUMENTATION,
            SemanticTokenModifier::DECLARATION,
            SemanticTokenModifier::DEFINITION,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::ABSTRACT,
            SemanticTokenModifier::DEPRECATED,
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::DEFAULT_LIBRARY,
            $($ident),*
        ];
    };
}

define_semantic_token_modifiers![
    (ASYNC, "async"),
    (ATTRIBUTE_MODIFIER, "attribute"),
    (CALLABLE, "callable"),
    (CONSTANT, "constant"),
    (CONSUMING, "consuming"),
    (CONTROL_FLOW, "controlFlow"),
    (CRATE_ROOT, "crateRoot"),
    (INJECTED, "injected"),
    (INTRA_DOC_LINK, "intraDocLink"),
    (LIBRARY, "library"),
    (MUTABLE, "mutable"),
    (PUBLIC, "public"),
    (REFERENCE, "reference"),
    (TRAIT_MODIFIER, "trait"),
    (UNSAFE, "unsafe"),
];

#[derive(Default)]
pub(crate) struct ModifierSet(pub(crate) u32);

impl std::ops::BitOrAssign<SemanticTokenModifier> for ModifierSet {
    fn bitor_assign(&mut self, rhs: SemanticTokenModifier) {
        let idx = SUPPORTED_MODIFIERS
            .iter()
            .position(|it| it == &rhs)
            .unwrap();
        self.0 |= 1 << idx;
    }
}

/// Tokens are encoded relative to each other.
///
/// This is a direct port of <https://github.com/microsoft/vscode-languageserver-node/blob/f425af9de46a0187adb78ec8a46b9b2ce80c5412/server/src/sematicTokens.proposed.ts#L45>
pub struct SemanticTokensBuilder {
    id: String,
    prev_line: u32,
    prev_char: u32,
    data: Vec<lsp_types::SemanticToken>,
}

impl SemanticTokensBuilder {
    pub fn new(id: String) -> Self {
        SemanticTokensBuilder {
            id,
            prev_line: 0,
            prev_char: 0,
            data: Default::default(),
        }
    }

    /// Push a new token onto the builder
    pub fn push(&mut self, range: Range, token_index: u32, modifier_bitset: u32) {
        let mut push_line = range.start.line as u32;
        let mut push_char = range.start.character as u32;

        if !self.data.is_empty() {
            push_line -= self.prev_line;
            if push_line == 0 {
                push_char -= self.prev_char;
            }
        }

        // A token cannot be multiline
        let token_len = range.end.character - range.start.character;

        let token = lsp_types::SemanticToken {
            delta_line: push_line,
            delta_start: push_char,
            length: token_len as u32,
            token_type: token_index,
            token_modifiers_bitset: modifier_bitset,
        };

        self.data.push(token);

        self.prev_line = range.start.line as u32;
        self.prev_char = range.start.character as u32;
    }

    pub fn build(self) -> SemanticTokens {
        SemanticTokens {
            result_id: Some(self.id),
            data: self.data,
        }
    }
}

pub fn diff_tokens(
    old: &[lsp_types::SemanticToken],
    new: &[lsp_types::SemanticToken],
) -> Vec<SemanticTokensEdit> {
    let offset = new
        .iter()
        .zip(old.iter())
        .take_while(|&(n, p)| n == p)
        .count();

    let (_, old) = old.split_at(offset);
    let (_, new) = new.split_at(offset);

    let offset_from_end = new
        .iter()
        .rev()
        .zip(old.iter().rev())
        .take_while(|&(n, p)| n == p)
        .count();

    let (old, _) = old.split_at(old.len() - offset_from_end);
    let (new, _) = new.split_at(new.len() - offset_from_end);

    if old.is_empty() && new.is_empty() {
        vec![]
    } else {
        // The lsp data field is actually a byte-diff but we
        // travel in tokens so `start` and `delete_count` are in multiples of the
        // serialized size of `SemanticToken`.
        vec![SemanticTokensEdit {
            start: 5 * offset as u32,
            delete_count: 5 * old.len() as u32,
            data: Some(new.into()),
        }]
    }
}

pub(crate) fn get_type_index(ty: SemanticTokenType) -> u32 {
    SUPPORTED_TYPES.iter().position(|it| *it == ty).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from(t: (u32, u32, u32, u32, u32)) -> lsp_types::SemanticToken {
        lsp_types::SemanticToken {
            delta_line: t.0,
            delta_start: t.1,
            length: t.2,
            token_type: t.3,
            token_modifiers_bitset: t.4,
        }
    }

    #[test]
    fn test_diff_insert_at_end() {
        let before = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];
        let after = [
            from((1, 2, 3, 4, 5)),
            from((6, 7, 8, 9, 10)),
            from((11, 12, 13, 14, 15)),
        ];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 10,
                delete_count: 0,
                data: Some(vec![from((11, 12, 13, 14, 15))])
            }
        );
    }

    #[test]
    fn test_diff_insert_at_beginning() {
        let before = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];
        let after = [
            from((11, 12, 13, 14, 15)),
            from((1, 2, 3, 4, 5)),
            from((6, 7, 8, 9, 10)),
        ];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 0,
                delete_count: 0,
                data: Some(vec![from((11, 12, 13, 14, 15))])
            }
        );
    }

    #[test]
    fn test_diff_insert_in_middle() {
        let before = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];
        let after = [
            from((1, 2, 3, 4, 5)),
            from((10, 20, 30, 40, 50)),
            from((60, 70, 80, 90, 100)),
            from((6, 7, 8, 9, 10)),
        ];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 5,
                delete_count: 0,
                data: Some(vec![
                    from((10, 20, 30, 40, 50)),
                    from((60, 70, 80, 90, 100))
                ])
            }
        );
    }

    #[test]
    fn test_diff_remove_from_end() {
        let before = [
            from((1, 2, 3, 4, 5)),
            from((6, 7, 8, 9, 10)),
            from((11, 12, 13, 14, 15)),
        ];
        let after = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 10,
                delete_count: 5,
                data: Some(vec![])
            }
        );
    }

    #[test]
    fn test_diff_remove_from_beginning() {
        let before = [
            from((11, 12, 13, 14, 15)),
            from((1, 2, 3, 4, 5)),
            from((6, 7, 8, 9, 10)),
        ];
        let after = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 0,
                delete_count: 5,
                data: Some(vec![])
            }
        );
    }

    #[test]
    fn test_diff_remove_from_middle() {
        let before = [
            from((1, 2, 3, 4, 5)),
            from((10, 20, 30, 40, 50)),
            from((60, 70, 80, 90, 100)),
            from((6, 7, 8, 9, 10)),
        ];
        let after = [from((1, 2, 3, 4, 5)), from((6, 7, 8, 9, 10))];

        let edits = diff_tokens(&before, &after);
        assert_eq!(
            edits[0],
            SemanticTokensEdit {
                start: 5,
                delete_count: 10,
                data: Some(vec![])
            }
        );
    }
}
