use super::*;

#[derive(Prop)]
pub struct MutationCanvasProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    pinned_canvas_values: &'a ReadSignal<Vec<&'static FigureCanvasData>>,
    control_data: &'a Signal<FigureControlData>,
    mutation: &'static MutationFigureData,
}

#[component]
pub fn MutationCanvas<'a, G: Html>(scope: Scope<'a>, props: MutationCanvasProps<'a>) -> View<G> {
    view! {
        scope,
        FigureContentSwitch {
            pinned_canvas_values: props.pinned_canvas_values,
            canvas_value: &props.mutation.after,
            control_data: props.control_data,
            dimension: props.dimension
        }
    }
}
