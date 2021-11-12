use fixie::router::Dispatch;
use sycamore::{context::{ContextProvider, ContextProviderProps, use_context}, prelude::*, reactive::create_reducer};

struct AppState {
    count: Signal<i32>,
}


enum Events {
    InitDB(AppState),
}

impl Dispatch for Events {
    fn dispatch(&self) -> () {
        match self {
            Events::InitDB(AppState) => ()
        }
    }
}

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
