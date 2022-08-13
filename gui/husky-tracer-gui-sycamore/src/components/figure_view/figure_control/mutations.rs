use super::*;

#[derive(Prop)]
pub struct MutationsControlProps<'a> {
    mutations: &'a [MutationFigureData],
    figure_control_data: &'a Signal<FigureControlData>,
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn MutationsControl<'a, G: Html>(
    scope: Scope<'a>,
    props: MutationsControlProps<'a>,
) -> View<G> {
    view! {
        scope,
        div (class="MutationsControl") {
            (View::new_fragment(props.mutations.iter().map(|mutation|{
                view! {
                    scope,
                    MutationControl {
                        mutation,
                        figure_control_data: props.figure_control_data
                    }
                }
            }).collect()))
        }
    }
}

#[derive(Prop)]
pub struct MutationControlProps<'a> {
    mutation: &'a MutationFigureData,
    figure_control_data: &'a Signal<FigureControlData>,
}

#[component]
pub fn MutationControl<'a, G: Html>(scope: Scope<'a>, props: MutationControlProps<'a>) -> View<G> {
    view! {
        scope,
        div (class="MutationsControl") {
            div (class="MutationControlInner") {
                div (class="Name") {
                    (props.mutation.name)
                }
            }
        }
    }
}
