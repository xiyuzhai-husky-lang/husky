use super::*;
use sycamore::render;
use web_sys::{Event, HtmlDialogElement, HtmlFormElement, HtmlInputElement, KeyboardEvent};

#[component]
pub fn AttentionView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let debugger_context = use_context::<DebuggerContext>(scope);
    let attention_context = &debugger_context.attention_context;
    let generic = create_signal(scope, true);
    let focus = attention_context.focus.clone();
    let last_input_id = create_signal(scope, focus.get_untracked().opt_sample_id());
    let toggle_focus_kind_handler = debugger_context.toggle_attention_kind_handler();
    let attention_dialog = get_element_by_id::<HtmlDialogElement>("attention-dialog");
    let set_attention_from_dialog = debugger_context.set_attention_from_dialog_handler();
    let attention_kind = memo!(
        scope,
        move || match *focus.get() {
            Focus::Specific { .. } => "SPECIFIC",
            Focus::Generic { .. } => "GENERIC",
        }
        .to_string(),
        focus
    );
    add_event_listener!(
        attention_dialog,
        "keydown",
        move |event: web_sys::UiEvent| {
            let event: KeyboardEvent = event.unchecked_into();
            match event.key().as_str() {
                "Enter" => set_attention_from_dialog(),
                _ => (),
            }
        }
    );
    view! {
        scope,
        div (class="FocusView disable-select") {
            div (
                class="FocusKind",
                on:click=toggle_focus_kind_handler
            ) {
                (attention_kind.get())
            }
            (match *focus.get() {
                Focus::Specific { input_id } => {
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
                                    get_element_by_id::<HtmlDialogElement>("attention-dialog").show_modal();
                                }
                            }
                        ) {
                            (input_id)
                        }
                    }
                },
                Focus::Generic { .. } =>view!{scope,},
            })
        }
    }
}
