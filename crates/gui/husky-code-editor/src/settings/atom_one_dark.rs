use husky_token_protocol::TokenClass;

use ecolor::Color32;

pub const ATOM_ONE_DARK_COLOR_BLACK: Color32 = Color32::from_rgb(0x28, 0x2C, 0x34);

pub const ATOM_ONE_DARK_COLOR_RED: Color32 = Color32::from_rgb(0xE0, 0x6C, 0x75);

pub const ATOM_ONE_DARK_COLOR_GREEN: Color32 = Color32::from_rgb(0x98, 0xC3, 0x79);

pub const ATOM_ONE_DARK_COLOR_YELLOW: Color32 = Color32::from_rgb(0xE5, 0xC0, 0x7B);

pub const ATOM_ONE_DARK_COLOR_BLUE: Color32 = Color32::from_rgb(0x61, 0xAF, 0xEF);

pub const ATOM_ONE_DARK_COLOR_PURPLE: Color32 = Color32::from_rgb(0xC6, 0x78, 0xDD);

pub const ATOM_ONE_DARK_COLOR_CYAN: Color32 = Color32::from_rgb(0x56, 0xB6, 0xC2);

pub const ATOM_ONE_DARK_COLOR_WHITE: Color32 = Color32::from_rgb(0xAB, 0xB2, 0xBF);

pub const ATOM_ONE_DARK_COLOR_BRIGHT_BLACK: Color32 = Color32::from_rgb(0x54, 0x58, 0x62);

pub const ATOM_ONE_DARK_COLOR_BRIGHT_RED: Color32 = Color32::from_rgb(0xE0, 0x6C, 0x75);

pub const ATOM_ONE_DARK_COLOR_BRIGHT_GREEN: Color32 = Color32::from_rgb(0x98, 0xC3, 0x79);

pub const ATOM_ONE_DARK_COLOR_BRIGHT_YELLOW: Color32 = Color32::from_rgb(0xE5, 0xC0, 0x7B);

pub const ATOM_ONE_DARK_COLOR_BRIGHT_BLUE: Color32 = Color32::from_rgb(0x61, 0xAF, 0xEF);

pub const ATOM_ONE_DARK_COLOR_BRIGHT_PURPLE: Color32 = Color32::from_rgb(0xC6, 0x78, 0xDD);

pub const ATOM_ONE_DARK_COLOR_BRIGHT_CYAN: Color32 = Color32::from_rgb(0x56, 0xB6, 0xC2);

pub const ATOM_ONE_DARK_COLOR_BRIGHT_WHITE: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);

pub fn atom_one_theme_map(token_class: TokenClass) -> Color32 {
    match token_class {
        TokenClass::Attribute => ATOM_ONE_DARK_COLOR_BRIGHT_WHITE,
        TokenClass::Comment => ATOM_ONE_DARK_COLOR_BRIGHT_BLACK,
        TokenClass::ControlFlowKeyword => ATOM_ONE_DARK_COLOR_PURPLE,
        TokenClass::OtherKeyword => ATOM_ONE_DARK_COLOR_PURPLE,
        TokenClass::Field => ATOM_ONE_DARK_COLOR_WHITE,
        TokenClass::Punctuation => ATOM_ONE_DARK_COLOR_WHITE,
        TokenClass::Parameter => ATOM_ONE_DARK_COLOR_RED,
        TokenClass::Variable => ATOM_ONE_DARK_COLOR_RED,
        TokenClass::FrameVariable => ATOM_ONE_DARK_COLOR_WHITE,
        TokenClass::ModuleEntity => ATOM_ONE_DARK_COLOR_WHITE,
        TokenClass::TypeEntity => ATOM_ONE_DARK_COLOR_BRIGHT_YELLOW,
        TokenClass::FunctionEntity => ATOM_ONE_DARK_COLOR_BLUE,
        TokenClass::ValEntity => ATOM_ONE_DARK_COLOR_CYAN,
        TokenClass::TraitEntity => ATOM_ONE_DARK_COLOR_PURPLE,
        TokenClass::TypeVariantEntity => ATOM_ONE_DARK_COLOR_BRIGHT_YELLOW,
        TokenClass::MethodEntity => ATOM_ONE_DARK_COLOR_BLUE,
        TokenClass::MemoizedFieldEntity => ATOM_ONE_DARK_COLOR_WHITE,
        TokenClass::ImplicitParameter => ATOM_ONE_DARK_COLOR_WHITE,
        TokenClass::Method => ATOM_ONE_DARK_COLOR_BLUE,
        TokenClass::Literal => ATOM_ONE_DARK_COLOR_BRIGHT_YELLOW,
        TokenClass::HtmlTagKind => ATOM_ONE_DARK_COLOR_GREEN,
        TokenClass::WordOpr => ATOM_ONE_DARK_COLOR_BRIGHT_WHITE,
        TokenClass::SelfType => ATOM_ONE_DARK_COLOR_CYAN,
        TokenClass::SelfValue => ATOM_ONE_DARK_COLOR_RED,
        TokenClass::HtmlFunctionIdent => ATOM_ONE_DARK_COLOR_BLUE,
        TokenClass::HtmlPropertyIdent => ATOM_ONE_DARK_COLOR_BRIGHT_YELLOW,
        TokenClass::Todo => ATOM_ONE_DARK_COLOR_BRIGHT_RED,
        TokenClass::Unreachable => ATOM_ONE_DARK_COLOR_BRIGHT_RED,
        TokenClass::Ident => ATOM_ONE_DARK_COLOR_WHITE,
        TokenClass::Label => ATOM_ONE_DARK_COLOR_BRIGHT_GREEN,
        TokenClass::Error => ATOM_ONE_DARK_COLOR_BRIGHT_RED,
    }
}
