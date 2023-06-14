use super::*;
use husky_trace_protocol::OriginalImageData;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

#[derive(Prop)]
pub struct ImageProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    image_layers: Vec<&'a ImageLayerData>,
}

fn render(
    html_canvas: HtmlCanvasElement,
    composed_image_data: &OriginalImageData,
    dimension: PixelDimension,
) {
    html_canvas.set_width(dimension.width);
    html_canvas.set_height(dimension.height);
    let html_canvas_rendering_context = html_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    html_canvas_rendering_context
        .put_image_data(
            &composed_image_data.to_image_data_scaled(dimension),
            0.0,
            0.0,
        )
        .unwrap()
}

#[component]
pub fn Image<'a, G: Html>(visibility: Scope<'a>, props: ImageProps<'a>) -> View<G> {
    let canvas_ref = create_node_ref_signal(visibility);
    let dimension = props.dimension;
    let composed_image_data = OriginalImageData::new_composed(&props.image_layers);
    effect!(visibility, {
        dimension.track();
        move || {
            let dimension = dimension.cget();
            if let Some(canvas) = canvas_ref.try_get::<DomNode>() {
                let canvas = canvas.inner_element();
                let html_canvas: HtmlCanvasElement = canvas
                    .dyn_into::<HtmlCanvasElement>()
                    .map_err(|_| ())
                    .unwrap();
                render(html_canvas, &composed_image_data, dimension)
            }
        }
    });
    view! {
        visibility,
        canvas (
            ref=canvas_ref,
            style=dimension.get().to_style()
        )
    }
}
