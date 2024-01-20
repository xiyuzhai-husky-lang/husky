use super::*;
use egui::{Color32, ColorImage};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct ImageVisual(VisualId);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageVisualData {
    Binary {
        bits_per_row: u8,
        width: u32,
        height: u32,
        /// the len must be a multiple of `bits_per_row`
        bitmap: Vec<u8>,
    },
}

impl ImageVisual {
    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a ImageVisualData {
        let VisualData::Image(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }

    pub fn color_image(self, visual_synchrotron: &VisualSynchrotron) -> ColorImage {
        self.data(visual_synchrotron).into()
    }
}

impl Into<ColorImage> for &ImageVisualData {
    fn into(self) -> ColorImage {
        match *self {
            ImageVisualData::Binary {
                width,
                height,
                bits_per_row,
                ref bitmap,
            } => {
                let pixels = (0..height)
                    .into_iter()
                    .map(move |i| {
                        (0..width).into_iter().map(move |j| {
                            let j0 = j / 8;
                            let j1 = j % 8;
                            debug_assert_eq!(j, j0 * 8 + j1);
                            debug_assert!(j1 >= 0);
                            debug_assert!(j1 < 8);
                            let byte: u8 = bitmap[(i * (bits_per_row as u32) + j0) as usize];
                            let bit: bool = (byte & (1 << (7 - j1))) != 0;
                            match bit {
                                true => Color32::WHITE,
                                false => Color32::BLACK,
                            }
                        })
                    })
                    .flatten()
                    .collect();
                ColorImage {
                    size: [width as usize, height as usize],
                    pixels,
                }
            }
        }
    }
}

#[test]
fn image_visual_data_into_color_image_works() {
    use expect_test::{expect, expect_file};

    let ColorImage { size, pixels } = (&ImageVisualData::Binary {
        bits_per_row: 2,
        width: 15,
        height: 4,
        bitmap: vec![1, 0, 11, 0, 31, 0, 51, 0],
    })
        .into();
    let size_expected = expect![[r#"
        [
            15,
            4,
        ]
    "#]];
    let pixels_expected = expect_file!["../../expect-files/image_visual_data_into_color_image.md"];
    size_expected.assert_debug_eq(&size);
    pixels_expected.assert_debug_eq(&pixels)
}
