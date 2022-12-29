use crate::*;
use husky_entity_taxonomy::{EntityKind, FormKind, ModuleItemKind};
use husky_text::TextRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangedSemanticToken {
    pub semantic_token: SemanticToken,
    pub range: TextRange,
}

impl RangedSemanticToken {
    pub fn new(token: SemanticToken, range: TextRange) -> Self {
        Self {
            semantic_token: token,
            range,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticToken {
    Attribute,
    Comment,
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
    WordOpr,
}

impl SemanticToken {
    pub fn token_type(self) -> u32 {
        get_type_index(match self {
            SemanticToken::Keyword(_) => ext::SemanticTokenType::KEYWORD,
            SemanticToken::Comment => ext::SemanticTokenType::COMMENT,
            SemanticToken::Field => ext::SemanticTokenType::PROPERTY,
            SemanticToken::Special => ext::SemanticTokenType::OPERATOR,
            SemanticToken::Variable => ext::SemanticTokenType::VARIABLE,
            SemanticToken::ThisValue => ext::SemanticTokenType::VARIABLE,
            SemanticToken::FrameVariable => ext::SemanticTokenType::VARIABLE,
            SemanticToken::Entity(entity_kind) => match entity_kind {
                EntityKind::Module => todo!(),
                EntityKind::ModuleItem {
                    module_item_kind,
                    connection,
                } => match module_item_kind {
                    ModuleItemKind::Type(_) => ext::SemanticTokenType::TYPE,
                    ModuleItemKind::Form(form_kind) => match form_kind {
                        FormKind::Feature => ext::SemanticTokenType::VARIABLE,
                        FormKind::Function => ext::SemanticTokenType::FUNCTION,
                        FormKind::Value => todo!(),
                        FormKind::TypeAlias => todo!(),
                    },
                    ModuleItemKind::Trait => ext::SemanticTokenType::TYPE,
                },
                EntityKind::AssociatedItem {
                    associated_item_kind,
                } => match associated_item_kind {
                    ModuleItemKind::Type(_) => todo!(),
                    ModuleItemKind::Form(_) => todo!(),
                    ModuleItemKind::Trait => todo!(),
                },
                EntityKind::Variant => todo!(),
                // EntityKind::Module => ext::SemanticTokenType::NAMESPACE,
                // EntityKind::Type(_) => ext::SemanticTokenType::TYPE,
                // EntityKind::Trait => ext::SemanticTokenType::TYPE,
                // EntityKind::Member(member_kind) => match member_kind {
                //     MemberKind::Method { .. } => ext::SemanticTokenType::METHOD,
                //     MemberKind::Call => todo!(),
                //     MemberKind::TraitAssociatedType => todo!(),
                //     MemberKind::TraitAssociatedConstSize => todo!(),
                //     MemberKind::Field => todo!(),
                //     MemberKind::TraitAssociatedAny => panic!(),
                // },
                // EntityKind::Function { .. } => ext::SemanticTokenType::FUNCTION,
                // EntityKind::Feature => ext::SemanticTokenType::VARIABLE,
                // EntityKind::EnumVariant => ext::SemanticTokenType::VARIABLE,
                // EntityKind::Main => panic!(),
                // EntityKind::Crate(_) => todo!(),
            },
            SemanticToken::GenericPlaceholder => ext::SemanticTokenType::TYPE_PARAMETER,
            SemanticToken::Parameter => ext::SemanticTokenType::PARAMETER,
            SemanticToken::EnumVariant => ext::SemanticTokenType::ENUM_MEMBER,
            SemanticToken::Method => ext::SemanticTokenType::METHOD,
            SemanticToken::Literal => ext::SemanticTokenType::NUMBER,
            SemanticToken::XmlTagKind => ext::SemanticTokenType::FUNCTION,
            SemanticToken::WordPattern => ext::SemanticTokenType::ENUM_MEMBER,
            SemanticToken::Attribute => ext::SemanticTokenType::DECORATOR,
            SemanticToken::WordOpr => ext::SemanticTokenType::OPERATOR,
        })
    }

    pub fn token_modifiers_bitset(self) -> u32 {
        let mut result = ModifierSet(0);
        match self {
            SemanticToken::Keyword(keyword) => match keyword {
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

#[derive(Default)]
pub(crate) struct ModifierSet(pub(crate) u32);

impl std::ops::BitOrAssign<ext::SemanticTokenModifier> for ModifierSet {
    fn bitor_assign(&mut self, rhs: ext::SemanticTokenModifier) {
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
    pub fn push(&mut self, range: ext::Range, token_index: u32, modifier_bitset: u32) {
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

    pub fn build(self) -> ext::SemanticTokens {
        ext::SemanticTokens {
            result_id: Some(self.id),
            data: self.data,
        }
    }
}

pub fn diff_tokens(
    old: &[lsp_types::SemanticToken],
    new: &[lsp_types::SemanticToken],
) -> Vec<ext::SemanticTokensEdit> {
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
        vec![ext::SemanticTokensEdit {
            start: 5 * offset as u32,
            delete_count: 5 * old.len() as u32,
            data: Some(new.into()),
        }]
    }
}

pub(crate) fn get_type_index(ty: ext::SemanticTokenType) -> u32 {
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
            ext::SemanticTokensEdit {
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
            ext::SemanticTokensEdit {
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
            ext::SemanticTokensEdit {
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
            ext::SemanticTokensEdit {
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
            ext::SemanticTokensEdit {
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
            ext::SemanticTokensEdit {
                start: 5,
                delete_count: 10,
                data: Some(vec![])
            }
        );
    }
}
