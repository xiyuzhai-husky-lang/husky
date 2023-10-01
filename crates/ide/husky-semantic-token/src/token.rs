use crate::*;
use husky_entity_kind::*;
use husky_entity_protocol::EntityProtocol;
use husky_text::TextRange;
use husky_token_protocol::{KeywordProtocol, TokenProtocol};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticToken {
    pub token_protocol: TokenProtocol,
    pub range: TextRange,
}

impl PartialOrd for SemanticToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range.start.partial_cmp(&other.range.start)
    }
}

impl SemanticToken {
    pub fn new(token_protocol: TokenProtocol, range: TextRange) -> Self {
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
            TokenProtocol::Keyword(_) => ext::SemanticTokenType::KEYWORD,
            TokenProtocol::Comment => ext::SemanticTokenType::COMMENT,
            TokenProtocol::Field => ext::SemanticTokenType::PROPERTY,
            TokenProtocol::Special => ext::SemanticTokenType::OPERATOR,
            TokenProtocol::Variable => ext::SemanticTokenType::VARIABLE,
            TokenProtocol::ThisValue => ext::SemanticTokenType::VARIABLE,
            TokenProtocol::FrameVariable => ext::SemanticTokenType::VARIABLE,
            TokenProtocol::Entity(entity_protocol) => match entity_protocol {
                EntityProtocol::Module => ext::SemanticTokenType::NAMESPACE,
                EntityProtocol::Type => ext::SemanticTokenType::TYPE,
                EntityProtocol::Val => ext::SemanticTokenType::VARIABLE,
                EntityProtocol::FunctionFn | EntityProtocol::FunctionGn => {
                    ext::SemanticTokenType::FUNCTION
                }
                EntityProtocol::AliasType => todo!(),
                EntityProtocol::Trait => ext::SemanticTokenType::CLASS,
                EntityProtocol::MethodFn | EntityProtocol::MethodGn => {
                    ext::SemanticTokenType::METHOD
                }
                EntityProtocol::AssociatedFunctionFn | EntityProtocol::AssociatedFunctionGn => {
                    ext::SemanticTokenType::FUNCTION
                }
                EntityProtocol::MemoizedField => ext::SemanticTokenType::PROPERTY,
                EntityProtocol::AssociatedVal => ext::SemanticTokenType::VARIABLE,
                EntityProtocol::AssociatedType => ext::SemanticTokenType::TYPE,
                EntityProtocol::TypeVariant => ext::SemanticTokenType::ENUM_MEMBER,
                EntityProtocol::ImplBlock => unreachable!(),
                EntityProtocol::Attr => unreachable!(),
            },
            TokenProtocol::ImplicitParameter => ext::SemanticTokenType::TYPE_PARAMETER,
            TokenProtocol::Parameter => ext::SemanticTokenType::PARAMETER,
            TokenProtocol::EnumVariant => ext::SemanticTokenType::ENUM_MEMBER,
            TokenProtocol::Method => ext::SemanticTokenType::METHOD,
            TokenProtocol::Literal => ext::SemanticTokenType::NUMBER,
            TokenProtocol::HtmlTagKind => ext::SemanticTokenType::FUNCTION,
            TokenProtocol::WordPattern => ext::SemanticTokenType::ENUM_MEMBER,
            TokenProtocol::Attribute => ext::SemanticTokenType::DECORATOR,
            TokenProtocol::WordOpr => ext::SemanticTokenType::KEYWORD,
            TokenProtocol::SelfType => ext::SemanticTokenType::TYPE,
            TokenProtocol::SelfValue => ext::SemanticTokenType::KEYWORD,
            TokenProtocol::HtmlFunctionIdent => ext::SemanticTokenType::FUNCTION,
            TokenProtocol::HtmlPropertyIdent => ext::SemanticTokenType::PROPERTY,
            TokenProtocol::SubmoduleIdent => ext::SemanticTokenType::NAMESPACE,
            TokenProtocol::Todo => ext::SemanticTokenType::MACRO,
            TokenProtocol::Unreachable => ext::SemanticTokenType::MACRO,
            TokenProtocol::Ident => ext::SemanticTokenType::VARIABLE,
            TokenProtocol::Label => ext::SemanticTokenType::VARIABLE,
            TokenProtocol::Error => ext::SemanticTokenType::MACRO,
        })
    }

    #[inline(always)]
    pub fn token_modifiers_bitset(self) -> u32 {
        let mut result = ModifierSet(0);
        match self.token_protocol {
            TokenProtocol::Keyword(keyword_kind) => match keyword_kind {
                KeywordProtocol::ControlFlow => result |= CONTROL_FLOW,
                KeywordProtocol::Other => (),
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
