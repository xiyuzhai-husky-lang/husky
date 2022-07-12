use super::*;
use sycamore::render;
use web_sys::{Event, HtmlDialogElement, HtmlFormElement, HtmlInputElement, KeyboardEvent};

#[component]
pub fn RestrictionView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let debugger_context = use_debugger_context(scope);
    let restriction_context = &debugger_context.restriction_context;
    let generic = create_signal(scope, true);
    let restriction = restriction_context.restriction;
    let last_sample_id = create_signal(scope, restriction.get_untracked().opt_sample_id());
    let toggle_restriction_kind_handler = debugger_context.toggle_restriction_kind_handler();
    let restriction_dialog = get_element_by_id::<HtmlDialogElement>("restriction-dialog");
    let set_restriction_from_dialog = debugger_context.set_restriction_from_dialog_handler();
    let restriction_kind = memo!(
        scope,
        move || match restriction.get().is_specific() {
            true => "SPECIFIC",
            false => "GENERIC",
        }
        .to_string(),
        restriction
    );
    add_event_listener!(
        restriction_dialog,
        "keydown",
        move |event: web_sys::UiEvent| {
            let event: KeyboardEvent = event.unchecked_into();
            match event.key().as_str() {
                "Enter" => set_restriction_from_dialog(),
                _ => (),
            }
        }
    );
    view! {
        scope,
        div (class="RestrictionView disable-select") {
            div (
                class="RestrictionKind",
                on:click=toggle_restriction_kind_handler
            ) {
                (restriction_kind.get())
            }
            ({
                match restriction.get().opt_sample_id() {
                    Some(sample_id)=> {
                        view! {
                            scope,
                            label (
                                id="sample-id-name",
                            ) {
                                "sample id = "
                            }
                            label (
                                id="sample-id-value",
                                on:click={
                                    move |_| {
                                        get_element_by_id::<HtmlDialogElement>("restriction-dialog").show_modal();
                                        get_element_by_id::<HtmlInputElement>("sample-id-input").set_value("") ;
                                    }
                                }
                            ) {
                                (sample_id.0)
                            }
                        }
                    },
                    None=> view!{scope,},
            }})
        }
    }
}
