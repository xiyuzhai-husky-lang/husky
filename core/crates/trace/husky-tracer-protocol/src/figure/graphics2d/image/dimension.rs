#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelDimension {
    pub width: u32,
    pub height: u32,
}

impl std::fmt::Display for PixelDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "width: {}px; height: {}px",
            self.width, self.height
        ))
    }
}

impl PixelDimension {
    pub fn to_style(&self) -> String {
        format!("width: {}px; height: {}px", self.width, self.height)
    }
}
