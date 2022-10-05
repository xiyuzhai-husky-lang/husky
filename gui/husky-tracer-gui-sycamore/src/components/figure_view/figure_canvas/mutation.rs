use super::*;

#[derive(Prop)]
pub struct MutationCanvasProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    control_data: &'a Signal<FigureControlData>,
    mutation: &'static MutationFigureData,
}

#[component]
pub fn MutationCanvas<'a, G: Html>(scope: Scope<'a>, props: MutationCanvasProps<'a>) -> View<G> {
    view! {
        scope,
        FigureCanvasSwitch {
            canvas_value: &props.mutation.after,
            control_data: props.control_data,
            dimension: props.dimension
        }
    }
}
