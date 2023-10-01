use crate::*;
use husky_entity_taxonomy::*;
use husky_text::TextRange;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangedSemanticToken {
    pub semantic_token: SemanticToken,
    pub range: TextRange,
}

impl PartialOrd for RangedSemanticToken {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.range.start.partial_cmp(&other.range.start)
    }
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
    ImplicitParameter,
    EnumVariant,
    Method,
    Literal,
    HtmlTagKind,
    WordPattern,
    WordOpr,
    SelfType,
    SelfValue,
    HtmlFunctionIdent,
    HtmlPropertyIdent,
    SubmoduleIdent,
    Todo,
    Unreachable,
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
            SemanticToken::Entity(item_kind) => match item_kind {
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
            SemanticToken::ImplicitParameter => ext::SemanticTokenType::TYPE_PARAMETER,
            SemanticToken::Parameter => ext::SemanticTokenType::PARAMETER,
            SemanticToken::EnumVariant => ext::SemanticTokenType::ENUM_MEMBER,
            SemanticToken::Method => ext::SemanticTokenType::METHOD,
            SemanticToken::Literal => ext::SemanticTokenType::NUMBER,
            SemanticToken::HtmlTagKind => ext::SemanticTokenType::FUNCTION,
            SemanticToken::WordPattern => ext::SemanticTokenType::ENUM_MEMBER,
            SemanticToken::Attribute => ext::SemanticTokenType::DECORATOR,
            SemanticToken::WordOpr => ext::SemanticTokenType::KEYWORD,
            SemanticToken::SelfType => ext::SemanticTokenType::TYPE,
            SemanticToken::SelfValue => ext::SemanticTokenType::KEYWORD,
            SemanticToken::HtmlFunctionIdent => ext::SemanticTokenType::FUNCTION,
            SemanticToken::HtmlPropertyIdent => ext::SemanticTokenType::PROPERTY,
            SemanticToken::SubmoduleIdent => ext::SemanticTokenType::NAMESPACE,
            SemanticToken::Todo => ext::SemanticTokenType::MACRO,
            SemanticToken::Unreachable => ext::SemanticTokenType::MACRO,
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
                    | StmtKeyword::NonImplFor
                    | StmtKeyword::ForExt
                    | StmtKeyword::While
                    | StmtKeyword::Do
                    | StmtKeyword::Break
                    | StmtKeyword::Return
                    | StmtKeyword::Require => result |= CONTROL_FLOW,
                    StmtKeyword::Let | StmtKeyword::Assert => (),
                },
                Keyword::End(_) => result |= CONTROL_FLOW,
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

pub(crate) fn get_type_index(ty: ext::SemanticTokenType) -> u32 {
    SUPPORTED_TYPES.iter().position(|it| *it == ty).unwrap() as u32
}
