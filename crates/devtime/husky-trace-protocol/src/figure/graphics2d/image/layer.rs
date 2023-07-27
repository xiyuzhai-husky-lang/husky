use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum ImageLayerData {
    Colored { pixels: Vec<Vec<(u8, u8, u8)>> },
    Binary28 { rows: Box<[u32; 28]> },
}

impl Signalable for ImageLayerData {}

impl ImageLayerData {
    pub fn binary_image28(padded_rows: &[u32; 30]) -> Self {
        Self::Binary28 {
            rows: Box::new(padded_rows[1..29].try_into().unwrap()),
        }
    }

    pub fn dimension(&self) -> PixelDimension {
        match self {
            ImageLayerData::Colored { pixels } => PixelDimension {
                width: pixels[0].len() as u32,
                height: pixels.len() as u32,
            },
            ImageLayerData::Binary28 { .. } => PixelDimension {
                width: 28,
                height: 28,
            },
        }
    }
}
