use super::*;

#[derive(Prop)]
pub struct MutationCanvasProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    control_data: &'a Signal<FigureControlData>,
    mutation: &'static MutationFigureData,
}

#[component]
pub fn MutationCanvas<'a, G: Html>(
    visibility: Scope<'a>,
    props: MutationCanvasProps<'a>,
) -> View<G> {
    todo!()
    // view! {
    //     visibility,
    //     FigureCanvasSwitch {
    //         canvas_value: &props.mutation.after,
    //         control_data: props.control_data,
    //         dimension: props.dimension
    //     }
    // }
}
