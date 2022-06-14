use super::*;

#[derive(Prop)]
pub struct ImageProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    image_layers: Rc<Vec<ImageLayerData>>,
}

#[component]
pub fn Image<'a, G: Html>(scope: Scope<'a>, props: ImageProps<'a>) -> View<G> {
    let canvas_ref = create_node_ref(scope);
    let dimension = props.dimension;
    create_effect(scope, {
        dimension.track();
        || {
            dimension.track();
            log::info!("todo: draw")
        }
    });
    view! {
        scope,
        canvas (ref=canvas_ref)
    }
}
