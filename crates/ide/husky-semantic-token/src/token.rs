use crate::*;
use husky_entity_kind::*;
use husky_keyword_kind::KeywordKind;
use husky_semantic_token_kind::SemanticTokenKind;
use husky_text::TextRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticToken {
    pub kind: SemanticTokenKind,
    pub range: TextRange,
}

impl PartialOrd for SemanticToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range.start.partial_cmp(&other.range.start)
    }
}

impl SemanticToken {
    pub fn new(kind: SemanticTokenKind, range: TextRange) -> Self {
        Self { kind, range }
    }
}

impl SemanticToken {
    #[inline(always)]
    pub fn token_type(self) -> u32 {
        get_type_index(match self.kind {
            SemanticTokenKind::Keyword(_) => ext::SemanticTokenType::KEYWORD,
            SemanticTokenKind::Comment => ext::SemanticTokenType::COMMENT,
            SemanticTokenKind::Field => ext::SemanticTokenType::PROPERTY,
            SemanticTokenKind::Special => ext::SemanticTokenType::OPERATOR,
            SemanticTokenKind::Variable => ext::SemanticTokenType::VARIABLE,
            SemanticTokenKind::ThisValue => ext::SemanticTokenType::VARIABLE,
            SemanticTokenKind::FrameVariable => ext::SemanticTokenType::VARIABLE,
            SemanticTokenKind::Entity(item_kind) => match item_kind {
                EntityKind::Module => ext::SemanticTokenType::NAMESPACE,
                EntityKind::MajorItem {
                    module_item_kind,
                    connection: _,
                } => match module_item_kind {
                    MajorItemKind::Type(_) => ext::SemanticTokenType::TYPE,
                    MajorItemKind::Fugitive(form_kind) => match form_kind {
                        FugitiveKind::Val => ext::SemanticTokenType::VARIABLE,
                        FugitiveKind::Fn | FugitiveKind::Gn => ext::SemanticTokenType::FUNCTION,
                        FugitiveKind::AliasType => todo!(),
                    },
                    MajorItemKind::Trait => ext::SemanticTokenType::TYPE,
                },
                EntityKind::AssociatedItem {
                    associated_item_kind,
                } => match associated_item_kind {
                    AssociatedItemKind::TypeItem(ty_item_kind) => match ty_item_kind {
                        TypeItemKind::MethodFn => ext::SemanticTokenType::METHOD,
                        TypeItemKind::AssociatedFn => ext::SemanticTokenType::FUNCTION,
                        TypeItemKind::MemoizedField => ext::SemanticTokenType::PROPERTY,
                        TypeItemKind::AssociatedVal => ext::SemanticTokenType::VARIABLE,
                        TypeItemKind::AssociatedType => ext::SemanticTokenType::TYPE,
                    },
                    AssociatedItemKind::TraitItem(_) => todo!(),
                    AssociatedItemKind::TraitForTypeItem(trai_for_ty_kind) => {
                        match trai_for_ty_kind {
                            TraitItemKind::MethodFn => ext::SemanticTokenType::METHOD,
                            TraitItemKind::AssociatedType => ext::SemanticTokenType::TYPE,
                        }
                    }
                },
                EntityKind::TypeVariant => ext::SemanticTokenType::ENUM_MEMBER,
                EntityKind::Trait => ext::SemanticTokenType::CLASS,
                EntityKind::ImplBlock => unreachable!(),
                EntityKind::Attr => unreachable!(),
            },
            SemanticTokenKind::ImplicitParameter => ext::SemanticTokenType::TYPE_PARAMETER,
            SemanticTokenKind::Parameter => ext::SemanticTokenType::PARAMETER,
            SemanticTokenKind::EnumVariant => ext::SemanticTokenType::ENUM_MEMBER,
            SemanticTokenKind::Method => ext::SemanticTokenType::METHOD,
            SemanticTokenKind::Literal => ext::SemanticTokenType::NUMBER,
            SemanticTokenKind::HtmlTagKind => ext::SemanticTokenType::FUNCTION,
            SemanticTokenKind::WordPattern => ext::SemanticTokenType::ENUM_MEMBER,
            SemanticTokenKind::Attribute => ext::SemanticTokenType::DECORATOR,
            SemanticTokenKind::WordOpr => ext::SemanticTokenType::KEYWORD,
            SemanticTokenKind::SelfType => ext::SemanticTokenType::TYPE,
            SemanticTokenKind::SelfValue => ext::SemanticTokenType::KEYWORD,
            SemanticTokenKind::HtmlFunctionIdent => ext::SemanticTokenType::FUNCTION,
            SemanticTokenKind::HtmlPropertyIdent => ext::SemanticTokenType::PROPERTY,
            SemanticTokenKind::SubmoduleIdent => ext::SemanticTokenType::NAMESPACE,
            SemanticTokenKind::Todo => ext::SemanticTokenType::MACRO,
            SemanticTokenKind::Unreachable => ext::SemanticTokenType::MACRO,
            SemanticTokenKind::Ident => ext::SemanticTokenType::VARIABLE,
            SemanticTokenKind::Label => ext::SemanticTokenType::VARIABLE,
            SemanticTokenKind::Error => ext::SemanticTokenType::MACRO,
        })
    }

    #[inline(always)]
    pub fn token_modifiers_bitset(self) -> u32 {
        let mut result = ModifierSet(0);
        match self.kind {
            SemanticTokenKind::Keyword(keyword_kind) => match keyword_kind {
                KeywordKind::ControlFlow => result |= CONTROL_FLOW,
                KeywordKind::Other => (),
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

pub(crate) fn get_type_index(ty: ext::SemanticTokenType) -> u32 {
    SUPPORTED_TYPES.iter().position(|it| *it == ty).unwrap() as u32
}
