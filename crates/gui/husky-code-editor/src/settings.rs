use ecolor::Color32;
use enum_index::array::EnumArray;
use husky_token_protocol::TokenClass;

#[derive(PartialEq, Eq)]
pub struct CodeEditorSettings {
    token_foreground_colors: EnumArray<TokenClass, Color32>,
}

impl Default for CodeEditorSettings {
    fn default() -> Self {
        Self {
            token_foreground_colors: EnumArray::new(|token_class| match token_class {
                TokenClass::Attribute => Color32::WHITE,
                TokenClass::Comment => Color32::WHITE,
                TokenClass::ControlFlowKeyword => Color32::LIGHT_RED,
                TokenClass::OtherKeyword => Color32::LIGHT_RED,
                TokenClass::Field => Color32::WHITE,
                TokenClass::Special => Color32::WHITE,
                TokenClass::Parameter => Color32::RED,
                TokenClass::Variable => Color32::RED,
                TokenClass::FrameVariable => Color32::WHITE,
                TokenClass::ModuleEntity => Color32::WHITE,
                TokenClass::TypeEntity => Color32::LIGHT_YELLOW,
                TokenClass::FunctionEntity => Color32::BLUE,
                TokenClass::ValEntity => Color32::WHITE,
                TokenClass::TraitEntity => Color32::WHITE,
                TokenClass::TypeVariantEntity => Color32::WHITE,
                TokenClass::MethodEntity => Color32::WHITE,
                TokenClass::MemoizedFieldEntity => Color32::WHITE,
                TokenClass::ImplicitParameter => Color32::WHITE,
                TokenClass::Method => Color32::WHITE,
                TokenClass::Literal => Color32::WHITE,
                TokenClass::HtmlTagKind => Color32::WHITE,
                TokenClass::WordPattern => Color32::WHITE,
                TokenClass::WordOpr => Color32::WHITE,
                TokenClass::SelfType => Color32::WHITE,
                TokenClass::SelfValue => Color32::WHITE,
                TokenClass::HtmlFunctionIdent => Color32::WHITE,
                TokenClass::HtmlPropertyIdent => Color32::WHITE,
                TokenClass::Todo => Color32::WHITE,
                TokenClass::Unreachable => Color32::WHITE,
                TokenClass::Ident => Color32::WHITE,
                TokenClass::Label => Color32::WHITE,
                TokenClass::Error => Color32::WHITE,
            }),
        }
    }
}

pub trait HasCodeEditorSettings {
    fn code_editor_settings(&self) -> &CodeEditorSettings;
}
