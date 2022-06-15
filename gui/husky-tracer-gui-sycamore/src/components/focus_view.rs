use super::*;

#[component]
pub fn FocusView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let generic = create_signal(scope, true);
    view! {
        scope,
        div (class="FocusView disable-select") {
            div (
                class="FocusKind",
                on:click=|_|generic.set(!generic.get_cloned())
            ) {
                (if generic.get_cloned() {
                    "generic"
                } else {
                    "specific"
                })
            }
        }
    }
}
