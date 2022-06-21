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
    let toggle_focus_kind_handler = debugger_context.toggle_focus_kind_handler();
    let attention_dialog = get_element_by_id("attention-dialog")
        .dyn_into::<HtmlDialogElement>()
        .unwrap();
    let closure = {
        Closure::wrap(Box::new({
            move |event: web_sys::UiEvent| {
                let event: KeyboardEvent = event.unchecked_into();
                match event.key().as_str() {
                    "Enter" => {
                        let attention_dialog_form_sample_id_input =
                            get_element_by_id("attention-dialog-form-sample-id-input")
                                .dyn_into::<HtmlInputElement>()
                                .unwrap();
                        let sample_id_value = attention_dialog_form_sample_id_input.value();
                        match sample_id_value.parse::<i32>() {
                            Ok(_) => {
                                let attention_dialog = get_element_by_id("attention-dialog")
                                    .dyn_into::<HtmlDialogElement>()
                                    .unwrap();
                                attention_dialog.close()
                            }
                            Err(_) => todo!(),
                        }
                    }
                    _ => log::info!("event.key().as_str() = {}", event.key().as_str()),
                }
            }
        }) as Box<dyn FnMut(_)>)
    };
    attention_dialog.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
    closure.forget();
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
            button (
                on:click={
                    move |_| {
                        get_element_by_id("attention-dialog")
                        .dyn_into::<HtmlDialogElement>()
                        .unwrap() .show_modal();
                    }
                }
            ) {
                "open attention dialog"
            }
        }
    }
}
