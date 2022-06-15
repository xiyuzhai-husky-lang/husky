use super::*;

#[component]
pub fn FocusView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let focus_context = &tracer_context.focus_context;
    let generic = create_signal(scope, true);
    let focus = focus_context.focus.clone();
    let last_input_id = create_signal(scope, focus.get_untracked().opt_input_id());
    view! {
        scope,
        div (class="FocusView disable-select") {
            div (
                class="FocusKind",
                on:click=|_|generic.set(!generic.get_cloned())
            ) {
                (if generic.get_cloned() {
                    "GENERIC"
                } else {
                    "SPECIFIC"
                })
            }
        }
    }
}
