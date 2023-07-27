use husky_signal::Signalable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PixelDimension {
    pub width: u32,
    pub height: u32,
}

impl Signalable for PixelDimension {}

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

impl std::ops::Add<(u32, u32)> for PixelDimension {
    type Output = Self;

    fn add(self, (width, height): (u32, u32)) -> Self::Output {
        Self {
            width: self.width + width,
            height: self.height + height,
        }
    }
}

impl std::ops::Sub<(u32, u32)> for PixelDimension {
    type Output = Self;

    fn sub(self, (width, height): (u32, u32)) -> Self::Output {
        Self {
            width: self.width - width,
            height: self.height - height,
        }
    }
}

impl std::ops::Mul<(u32, u32)> for PixelDimension {
    type Output = Self;

    fn mul(self, (width, height): (u32, u32)) -> Self::Output {
        Self {
            width: self.width * width,
            height: self.height * height,
        }
    }
}

impl std::ops::Div<(u32, u32)> for PixelDimension {
    type Output = Self;

    fn div(self, (width, height): (u32, u32)) -> Self::Output {
        Self {
            width: self.width / width,
            height: self.height / height,
        }
    }
}
