// #![recursion_limit = "512"]

// mod components;
// mod services;

// use components::*;
// use std::cell::RefCell;
// use std::rc::Rc;
// use wasm_bindgen::prelude::*;
// use yew::functional::*;
// use yew::prelude::*;
// use yew_router::prelude::*;

// // When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// // allocator.
// //
// // If you don't want to use `wee_alloc`, you can safely delete this.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[function_component(Main)]
// fn main() -> Html {
//     let ctx = use_state(|| {
//         Rc::new(UserInner {
//             username: RefCell::new("initial".into()),
//         })
//     });

//     html! {
//         <ContextProvider<User> context={(*ctx).clone()}>
//             <BrowserRouter>
//                 <div class="flex w-screen h-screen">
//                     <Panel />
//                 </div>
//             </BrowserRouter>
//         </ContextProvider<User>>
//     }
// }

// #[wasm_bindgen]
// pub fn run_app() -> Result<(), JsValue> {
//     wasm_logger::init(wasm_logger::Config::default());
//     yew::start_app::<Main>();
//     Ok(())
// }

// pub type User = Rc<UserInner>;

// #[derive(Debug, PartialEq)]
// pub struct UserInner {
//     pub username: RefCell<String>,
// }

use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
