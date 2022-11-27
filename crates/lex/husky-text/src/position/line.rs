use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Line position in a document (zero-based)
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct TextLine(pub u32);

impl TextLine {
    pub fn to_next_line(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl HuskyDisplay for TextLine {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}row {: <4}{}",
                husky_print_utils::YELLOW,
                self.0 + 1,
                husky_print_utils::RESET
            )
            .unwrap();
        } else {
            write!(result, "row {: <4}", self.0 + 1,).unwrap();
        }
    }
}

impl std::fmt::Debug for TextLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0 + 1).fmt(f)
    }
}

impl const From<u32> for TextLine {
    fn from(base0: u32) -> Self {
        TextLine(base0)
    }
}

impl From<usize> for TextLine {
    fn from(base0: usize) -> Self {
        TextLine(<usize as TryInto<u32>>::try_into(base0).unwrap())
    }
}

impl const From<i32> for TextLine {
    fn from(base0: i32) -> Self {
        assert!(base0 >= 0);
        TextLine(base0 as u32)
    }
}
