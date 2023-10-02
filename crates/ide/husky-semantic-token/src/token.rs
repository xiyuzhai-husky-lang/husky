use crate::*;
use husky_entity_kind::*;
use husky_entity_protocol::EntityKindSketch;
use husky_text::TextRange;
use husky_token_protocol::{KeywordKindProtocol, TokenKindProtocol};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticToken {
    pub token_protocol: TokenKindProtocol,
    pub range: TextRange,
}

impl PartialOrd for SemanticToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range.start.partial_cmp(&other.range.start)
    }
}

impl SemanticToken {
    pub fn new(token_protocol: TokenKindProtocol, range: TextRange) -> Self {
        Self {
            token_protocol,
            range,
        }
    }
}

impl SemanticToken {
    #[inline(always)]
    pub fn token_type(self) -> u32 {
        get_type_index(match self.token_protocol {
            TokenKindProtocol::Keyword(_) => ext::SemanticTokenType::KEYWORD,
            TokenKindProtocol::Comment => ext::SemanticTokenType::COMMENT,
            TokenKindProtocol::Field => ext::SemanticTokenType::PROPERTY,
            TokenKindProtocol::Special => ext::SemanticTokenType::OPERATOR,
            TokenKindProtocol::Variable => ext::SemanticTokenType::VARIABLE,
            TokenKindProtocol::ThisValue => ext::SemanticTokenType::VARIABLE,
            TokenKindProtocol::FrameVariable => ext::SemanticTokenType::VARIABLE,
            TokenKindProtocol::Entity(entity_protocol) => match entity_protocol {
                EntityKindSketch::Module => ext::SemanticTokenType::NAMESPACE,
                EntityKindSketch::Type => ext::SemanticTokenType::TYPE,
                EntityKindSketch::Val => ext::SemanticTokenType::VARIABLE,
                EntityKindSketch::FunctionFn | EntityKindSketch::FunctionGn => {
                    ext::SemanticTokenType::FUNCTION
                }
                EntityKindSketch::AliasType => todo!(),
                EntityKindSketch::Trait => ext::SemanticTokenType::CLASS,
                EntityKindSketch::MethodFn | EntityKindSketch::MethodGn => {
                    ext::SemanticTokenType::METHOD
                }
                EntityKindSketch::AssociatedFunctionFn | EntityKindSketch::AssociatedFunctionGn => {
                    ext::SemanticTokenType::FUNCTION
                }
                EntityKindSketch::MemoizedField => ext::SemanticTokenType::PROPERTY,
                EntityKindSketch::AssociatedVal => ext::SemanticTokenType::VARIABLE,
                EntityKindSketch::AssociatedType => ext::SemanticTokenType::TYPE,
                EntityKindSketch::TypeVariant => ext::SemanticTokenType::ENUM_MEMBER,
                EntityKindSketch::ImplBlock => unreachable!(),
                EntityKindSketch::Attr => unreachable!(),
            },
            TokenKindProtocol::ImplicitParameter => ext::SemanticTokenType::TYPE_PARAMETER,
            TokenKindProtocol::Parameter => ext::SemanticTokenType::PARAMETER,
            TokenKindProtocol::EnumVariant => ext::SemanticTokenType::ENUM_MEMBER,
            TokenKindProtocol::Method => ext::SemanticTokenType::METHOD,
            TokenKindProtocol::Literal => ext::SemanticTokenType::NUMBER,
            TokenKindProtocol::HtmlTagKind => ext::SemanticTokenType::FUNCTION,
            TokenKindProtocol::WordPattern => ext::SemanticTokenType::ENUM_MEMBER,
            TokenKindProtocol::Attribute => ext::SemanticTokenType::DECORATOR,
            TokenKindProtocol::WordOpr => ext::SemanticTokenType::KEYWORD,
            TokenKindProtocol::SelfType => ext::SemanticTokenType::TYPE,
            TokenKindProtocol::SelfValue => ext::SemanticTokenType::KEYWORD,
            TokenKindProtocol::HtmlFunctionIdent => ext::SemanticTokenType::FUNCTION,
            TokenKindProtocol::HtmlPropertyIdent => ext::SemanticTokenType::PROPERTY,
            TokenKindProtocol::SubmoduleIdent => ext::SemanticTokenType::NAMESPACE,
            TokenKindProtocol::Todo => ext::SemanticTokenType::MACRO,
            TokenKindProtocol::Unreachable => ext::SemanticTokenType::MACRO,
            TokenKindProtocol::Ident => ext::SemanticTokenType::VARIABLE,
            TokenKindProtocol::Label => ext::SemanticTokenType::VARIABLE,
            TokenKindProtocol::Error => ext::SemanticTokenType::MACRO,
        })
    }

    #[inline(always)]
    pub fn token_modifiers_bitset(self) -> u32 {
        let mut result = ModifierSet(0);
        match self.token_protocol {
            TokenKindProtocol::Keyword(keyword_kind) => match keyword_kind {
                KeywordKindProtocol::ControlFlow => result |= CONTROL_FLOW,
                KeywordKindProtocol::Other => (),
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
