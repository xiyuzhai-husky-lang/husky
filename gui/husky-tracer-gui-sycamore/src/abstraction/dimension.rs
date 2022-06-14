pub struct PixelDimension {
    pub width: i32,
    pub height: i32,
}

impl PixelDimension {
    pub fn to_style(&self) -> String {
        format!("width: {}px; height: {}px", self.width, self.height)
    }
}
