use fixie::router::{Dispatch, dispatch};
use sycamore::{context::{ContextProvider, ContextProviderProps, use_context}, prelude::*, reactive::create_reducer};

struct AppState {
    count: Signal<i32>,
}


enum Events {
    InitDB,
}

impl Dispatch for Events {}

#[component(App<G>)]
fn app() -> View<G> {
    view! {
        p { "coca-cola~" }
    }
}

fn main() {
    let app_state = AppState { count: Signal::new(10) };
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    dispatch(Box::new(Events::InitDB));

    sycamore::render(move || {
        view! {
            ContextProvider(ContextProviderProps {
                value: app_state,
                children: || view! {
                    App()
                }
            })
        }
    });
}
