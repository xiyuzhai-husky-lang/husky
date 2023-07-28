use sycamore::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Element, HtmlDialogElement, HtmlInputElement};

macro_rules! add_event_listener {
    ($element: expr, $event: expr, $closure: expr) => {{
        let closure = Closure::wrap(Box::new($closure) as Box<dyn FnMut(_)>);
        $element.add_event_listener_with_callback($event, closure.as_ref().unchecked_ref());
        closure.forget();
    }};
}
pub(crate) use add_event_listener;

pub(crate) fn get_gui() -> Element {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    document
        .get_elements_by_class_name("HuskyTracerGui")
        .item(0)
        .unwrap()
}

pub(crate) fn get_element_by_id<T: wasm_bindgen::JsCast>(id: &'static str) -> T {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    document
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<T>()
        .unwrap()
}

pub(crate) fn presentation_dialog() -> HtmlDialogElement {
    get_element_by_id::<HtmlDialogElement>("presentation-dialog")
}

pub(crate) fn new_partition_dialog() -> HtmlDialogElement {
    get_element_by_id::<HtmlDialogElement>("new-partition-dialog")
}

pub(crate) fn partition_input() -> HtmlInputElement {
    get_element_by_id::<HtmlInputElement>("partition-input")
}

pub(crate) fn partition_ncol_input() -> HtmlInputElement {
    get_element_by_id::<HtmlInputElement>("partition-ncol-input")
}

pub(crate) fn sample_id_input() -> HtmlInputElement {
    get_element_by_id::<HtmlInputElement>("sample-id-input")
}

macro_rules! alert {
    ($($args: expr), *) => {
        web_sys::window()
                .unwrap()
                .alert_with_message(&format!($($args),*))
                .unwrap()
    }
}
pub(crate) use alert;

macro_rules! insert_new {
    ($dict: expr, $key: expr, $value: expr) => {
        if $dict.contains_key(&$key) {
            panic!("key {:?} already exists in {:?}", $key, $dict.keys())
        }
        $dict.insert($key, $value).is_none();
    };
}
pub(crate) use insert_new;

use crate::context::DeveloperGuiContext;

pub(crate) fn create_leash<'a, T>(visibility: Scope<'a>, value: T) -> &'static T {
    unsafe { as_leash(create_ref(visibility, value)) }
}

pub(crate) fn create_static_signal<'a, T>(visibility: Scope<'a>, value: T) -> &'static Signal<T>
where
    T: Signalable,
{
    unsafe { as_leash(create_signal(visibility, value)) }
}
pub(crate) fn create_static_memo<'a, T>(
    visibility: Scope<'a>,
    f: impl FnMut() -> T + 'a,
) -> &'static ReadSignal<T>
where
    T: Signalable + 'a,
{
    unsafe {
        as_leash(create_memo(
            visibility,
            f,
            "create_static_memotodo".to_string(),
        ))
    }
}

pub(crate) unsafe fn as_leash<'a, T>(value: &T) -> &'static T {
    let ptr: *const T = value;
    &*ptr
}

pub(crate) fn use_dev_context<'a>(visibility: Scope<'a>) -> &'static DeveloperGuiContext {
    unsafe { as_leash(use_context::<DeveloperGuiContext>(visibility)) }
}
