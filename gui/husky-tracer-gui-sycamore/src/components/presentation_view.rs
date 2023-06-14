use super::*;
use sycamore::render;
use web_sys::{Event, HtmlDialogElement, HtmlFormElement, HtmlInputElement, KeyboardEvent};

#[component]
pub fn PresentationView<'a, G: Html>(visibility: Scope<'a>) -> View<G> {
    let ctx = use_dev_context(visibility);
    let generic = create_signal(visibility, true);
    let presentation_signal = ctx.presentation_signal();
    let last_sample_id = create_signal(
        visibility,
        presentation_signal.get_untracked().opt_sample_id(),
    );
    let toggle_presentation_kind_handler = ctx.toggle_presentation_kind_handler();
    let presentation_kind = memo!(
        visibility,
        move || match presentation_signal.get().is_specific() {
            true => "SPECIFIC",
            false => "GENERIC",
        }
        .to_string(),
        presentation_signal
    );
    view! {
        visibility,
        div (class="PresentationView disable-select") {
            div (
                class="PresentationKind",
                on:click=toggle_presentation_kind_handler
            ) {
                (presentation_kind.get())
            }
            label (
                id="sample-id-name",
            ) {
                "base point = "
            }
            label (
                id="sample-id-value",
                on:click=ctx.set_presentation_from_dialog_handler()
            ) {
                (presentation_signal.get().sample_id().0)
            }
        }
    }
}
