use super::*;
use sycamore::render;
use web_sys::{Event, HtmlDialogElement, HtmlFormElement, HtmlInputElement, KeyboardEvent};

#[component]
pub fn AttentionView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let debugger_context = use_context::<DebuggerContext>(scope);
    let attention_context = &debugger_context.attention_context;
    let generic = create_signal(scope, true);
    let attention = attention_context.attention.clone();
    let last_sample_idx = create_signal(scope, attention.get_untracked().opt_sample_idx());
    let toggle_attention_kind_handler = debugger_context.toggle_attention_kind_handler();
    let attention_dialog = get_element_by_id::<HtmlDialogElement>("attention-dialog");
    let set_attention_from_dialog = debugger_context.set_attention_from_dialog_handler();
    let attention_kind = memo!(
        scope,
        move || match *attention.get() {
            Attention::Specific { .. } => "SPECIFIC",
            Attention::Generic { .. } => "GENERIC",
        }
        .to_string(),
        attention
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
        div (class="AttentionView disable-select") {
            div (
                class="AttentionKind",
                on:click=toggle_attention_kind_handler
            ) {
                (attention_kind.get())
            }
            (match *attention.get() {
                Attention::Specific { sample_idx } => {
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
                                    get_element_by_id::<HtmlInputElement>("sample-id-input").set_value("") ;
                                }
                            }
                        ) {
                            (sample_idx.0)
                        }
                    }
                },
                Attention::Generic { .. } =>view!{scope,},
            })
        }
    }
}
