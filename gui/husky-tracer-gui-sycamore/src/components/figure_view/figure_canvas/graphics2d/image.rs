use super::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[derive(Prop)]
pub struct ImageProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    image_layers: Rc<Vec<ImageLayerData>>,
}

impl<'a> ImageProps<'a> {
    fn compose_image_layers(&self, image_layers: &[ImageLayerData]) -> ImageData {
        husky_tracer_protocol::OriginalImageData::new_composed(image_layers)
            .to_image_data_scaled(self.dimension.get_cloned())
    }
}

#[component]
pub fn Image<'a, G: Html>(scope: Scope<'a>, props: ImageProps<'a>) -> View<G> {
    let canvas_ref = create_node_ref(scope);
    let dimension = props.dimension;
    if props.image_layers.len() > 0 {
        let canvas_drawing_dimension = props.image_layers[0].dimension();
        let composed_image_data = memo!(scope, props.compose_image_layers(&props.image_layers));
        create_effect(scope, {
            dimension.track();
            move || {
                let dimension = dimension.get();
                if let Some(canvas) = canvas_ref.try_get::<DomNode>() {
                    let canvas = canvas.inner_element();
                    let canvas: web_sys::HtmlCanvasElement = canvas
                        .dyn_into::<web_sys::HtmlCanvasElement>()
                        .map_err(|_| ())
                        .unwrap();
                    canvas.set_width(dimension.width);
                    canvas.set_height(dimension.height);
                    let context = canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<web_sys::CanvasRenderingContext2d>()
                        .unwrap();
                    let composed_image_data = composed_image_data.get();
                    context
                        .put_image_data(&composed_image_data, 0.0, 0.0)
                        .unwrap();
                }
            }
        });
        view! {
            scope,
            canvas (
                ref=canvas_ref,
                style=dimension.get().to_style()
            )
        }
    } else {
        view! {scope,}
    }
}
