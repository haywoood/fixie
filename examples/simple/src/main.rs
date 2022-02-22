extern crate log;

use std::rc::Rc;
use log::debug;
use fixie::{
    Db,
    router::{Dispatchable, Effect, Event},
};
use sycamore::{context::{ContextProvider, ContextProviderProps, use_context}, prelude::*, reactive::create_reducer};

#[derive(Clone, Debug)]
struct AppState {
    count: Signal<i32>,
}

enum AppEvents {
    Inc(i32),
    Init
}

enum OtherEvents {
    OtherInc(i32),
}

impl Dispatchable<AppEvents> for AppState {
    fn handler(self: Rc<Self>, event: AppEvents) -> Event {
        match event {
            AppEvents::Init => Rc::new(vec![Box::new(Db {})]),
            AppEvents::Inc(inc_amount) => {
                Rc::new(vec![Box::new(Db {})])
            },
        }
    }
}

#[component(App<G>)]
fn app() -> View<G> {
    let app_state = Rc::new(use_context::<AppState>());

    app_state.clone().dispatch(AppEvents::Inc(10));

    view! {
        p { "Simple count" }
        p { (app_state.count.get()) }
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
