use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Character offset on a line in a document (zero-based). The meaning of this
/// offset is determined by the negotiated `PositionEncodingKind`.
///
/// If the character value is greater than the line length it defaults back
/// to the line length.
#[derive(
    Debug, PartialEq, Default, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
)]
pub struct TextColumn(pub u32);

impl HuskyDisplay for TextColumn {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}col {: <4}{}",
                husky_print_utils::YELLOW,
                self.0 + 1,
                husky_print_utils::RESET
            )
            .unwrap();
        } else {
            write!(result, "col {: <4}", self.0 + 1,).unwrap();
        }
    }
}

impl From<u32> for TextColumn {
    fn from(raw: u32) -> Self {
        TextColumn(raw)
    }
}

impl From<usize> for TextColumn {
    fn from(raw: usize) -> Self {
        TextColumn(<usize as TryInto<u32>>::try_into(raw).expect("success"))
    }
}

impl From<i32> for TextColumn {
    fn from(raw: i32) -> Self {
        assert!(raw >= 0);
        TextColumn(raw as u32)
    }
}

#[test]
fn test_conversion() {
    let a: i32 = -1;
    let _b: u32 = a as u32;
}
