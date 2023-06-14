mod image;
mod shape2d;

use super::*;
use image::*;
use shape2d::*;

#[derive(Prop)]
pub struct Graphics2dCanvasProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    image_layers: Vec<&'static ImageLayerData>,
    shapes: Vec<&'static Shape2dData>,
    xrange: (f32, f32),
    yrange: (f32, f32),
}

impl<'a> Graphics2dCanvasProps<'a> {
    fn svg_view_box(&self) -> String {
        let xmin = self.xrange.0;
        let width = self.xrange.1 - self.xrange.0;
        let ymin = 0;
        let height = self.yrange.1 - self.yrange.0;
        format!("{xmin} {ymin} {width} {height}")
    }

    fn svg_transform(&self) -> String {
        format!("matrix(1 0 0 -1 0 {})", self.yrange.1)
    }
}
#[component]
pub fn Graphics2dCanvas<'a, G: Html>(
    visibility: Scope<'a>,
    props: Graphics2dCanvasProps<'a>,
) -> View<G> {
    let transform = props.svg_transform();
    let view_box = props.svg_view_box();
    view! {
        visibility,
        div (
            class="Graphics2dWrapper",
            style=props.dimension.get().to_style(),
        ) {
            div (
                class="Graphics2d",
                style=props.dimension.get().to_style(),
            ) {
                Image {
                    dimension: props.dimension,
                    image_layers: props.image_layers,
                }
                svg (
                    style=props.dimension.get().to_style(),
                    preserveAspectRatio="none",
                    viewBox=view_box
                ) {
                    g (
                        transform=transform
                    ) {
                        (View::new_fragment(props.shapes.iter().map(|data| {
                            view! {
                                visibility,
                                Shape2d {
                                    data: data.clone()
                                }
                            }
                        }).collect()))
                    }
                }
            }
        }
    }
}
