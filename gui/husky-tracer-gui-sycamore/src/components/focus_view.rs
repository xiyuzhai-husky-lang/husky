use super::*;

#[component]
pub fn FocusView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let focus_context = &tracer_context.focus_context;
    let generic = create_signal(scope, true);
    let focus = focus_context.focus.clone();
    let last_input_id = create_signal(scope, focus.get_untracked().opt_input_id());
    let toggle_focus_kind_handler = tracer_context.toggle_focus_kind_handler();
    view! {
        scope,
        div (class="FocusView disable-select") {
            div (
                class="FocusKind",
                on:click=toggle_focus_kind_handler
            ) {
                (match *focus.get() {
                    Focus::Specific { .. } => "SPECIFIC",
                    Focus::Generic { .. } => "GENERIC",
                })
            }
        }
    }
}
