mod atom_one_dark;

use ecolor::Color32;
use enum_index::full_map::{EnumFullMap, EnumFullMapRef};
use husky_token_protocol::TokenClass;

#[derive(PartialEq, Eq)]
pub struct CodeEditorSettings {
    token_foreground_colors: EnumFullMap<TokenClass, Color32>,
}

impl CodeEditorSettings {
    pub fn token_foreground_colors(&self) -> EnumFullMapRef<TokenClass, Color32> {
        self.token_foreground_colors.as_ref()
    }
}

impl Default for CodeEditorSettings {
    fn default() -> Self {
        Self {
            token_foreground_colors: EnumFullMap::new(atom_one_dark::atom_one_theme_map),
        }
    }
}

pub trait HasCodeEditorSettings {
    fn code_editor_settings(&self) -> &CodeEditorSettings;
}
