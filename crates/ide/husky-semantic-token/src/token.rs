use crate::*;
use husky_entity_kind::*;
use husky_entity_protocol::EntityClass;
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
                EntityClass::Module => ext::SemanticTokenType::NAMESPACE,
                EntityClass::Type => ext::SemanticTokenType::TYPE,
                EntityClass::Val => ext::SemanticTokenType::VARIABLE,
                EntityClass::FunctionFn | EntityClass::FunctionGn => {
                    ext::SemanticTokenType::FUNCTION
                }
                EntityClass::AliasType => todo!(),
                EntityClass::Trait => ext::SemanticTokenType::CLASS,
                EntityClass::MethodFn | EntityClass::MethodGn => ext::SemanticTokenType::METHOD,
                EntityClass::AssociatedFunctionFn | EntityClass::AssociatedFunctionGn => {
                    ext::SemanticTokenType::FUNCTION
                }
                EntityClass::MemoizedField => ext::SemanticTokenType::PROPERTY,
                EntityClass::AssociatedVal => ext::SemanticTokenType::VARIABLE,
                EntityClass::AssociatedType => ext::SemanticTokenType::TYPE,
                EntityClass::TypeVariant => ext::SemanticTokenType::ENUM_MEMBER,
                EntityClass::ImplBlock => unreachable!(),
                EntityClass::Attr => unreachable!(),
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
