use super::*;
use web_sys::{Event, HtmlDialogElement, KeyboardEvent};

#[component]
pub fn AttentionView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let debugger_context = use_context::<DebuggerContext>(scope);
    let attention_context = &debugger_context.attention_context;
    let generic = create_signal(scope, true);
    let focus = attention_context.focus.clone();
    let last_input_id = create_signal(scope, focus.get_untracked().opt_sample_id());
    let toggle_focus_kind_handler = debugger_context.toggle_focus_kind_handler();
    let attention_dialog = Rc::new(
        get_element_by_id("attention-dialog")
            .dyn_into::<HtmlDialogElement>()
            .unwrap(),
    );
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
                on:click=move |_| {
                    attention_dialog.show_modal();
                    let closure = {
                        let attention_dialog = attention_dialog.clone();
                        Closure::wrap(
                            Box::new(move |_event: web_sys::UiEvent| {
                                log::info!("keydown");
                                attention_dialog.close()
                            }) as Box<dyn FnMut(_)>,
                        )
                    };
                    attention_dialog.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
                    closure.forget();
                }
            ) {
                "open attention dialog"
            }
        }
    }
}
