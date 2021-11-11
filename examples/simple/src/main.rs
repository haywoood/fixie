use fixie::*;
use sycamore::{
    context::{ContextProvider, ContextProviderProps, use_context},
    prelude::*,
};

struct AppState {
    count: Signal<i32>,
}


enum Events {
    InitDB(AppState),
}

#[component(App<G>)]
fn app() -> View<G> {
    view! {
        p { "coca-colas" }
    }
}

fn main() {
    let app_state = AppState { count: 10 };
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| {
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
