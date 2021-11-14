use log::debug;
use std::cell::*;
use std::collections::HashMap;
use std::rc::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::events::handle;

thread_local! {
    static EVENT_QUEUE: Rc<EventQueue> = Rc::new(EventQueue::new());
}

type PostEventCallback = Box<dyn FnOnce()>;

#[derive(Clone, Copy, Debug)]
enum Trigger {
    AddEvent,
    RunQueue,
    FinishRun,
    Exception,
}

#[derive(Clone, Copy, Debug)]
enum FsmState {
    Idle,
    Scheduled,
    Running,
    Paused,
}

pub trait Dispatch {}

pub type Event = Option<Box<dyn Dispatch>>;

#[derive(Clone)]
pub struct EventQueue {
    fsm_state: Rc<RefCell<FsmState>>,
    queue: Rc<RefCell<Vec<Event>>>,
    post_event_callback_fns: Rc<RefCell<HashMap<String, PostEventCallback>>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            fsm_state: Rc::new(RefCell::new(FsmState::Idle)),
            queue: Rc::new(RefCell::new(vec![])),
            post_event_callback_fns: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn fsm_trigger(&self, trigger: Trigger, event: Event) {
        let (new_fsm_state, action_fn): (FsmState, Box<dyn FnOnce()>) = {
            let current_fsm_state = *self.fsm_state.borrow();
            debug!("");
            debug!("__________________fsm_trigger______________________");
            debug!("");
            debug!("FsmState: {:?}", current_fsm_state);
            debug!("Trigger:  {:?}", trigger);
            debug!("");
            debug!("__________________/fsm_trigger_____________________");
            debug!("");
            // You should read the following "case" as:
            // [current-FSM-state trigger] -> [new-FSM-state action-fn]
            //
            // So, for example, the next line should be interpreted as:
            // if you are in state "FsmState::Idle" and a trigger "Trigger::AddEvent"
            // happens, then move the FSM to state "FsmState::Scheduled" and execute
            // that two-part Closure.
            match (current_fsm_state, trigger) {
                (FsmState::Idle, Trigger::AddEvent) => (
                    FsmState::Scheduled,
                    Box::new(move || {
                        self.add_event(event);
                        self.run_next_tick()
                    }),
                ),
                // State: FsmState::Scheduled  (the queue is scheduled to run, soon)
                (FsmState::Scheduled, Trigger::AddEvent) => {
                    (FsmState::Scheduled, Box::new(move || self.add_event(event)))
                }
                (FsmState::Scheduled, Trigger::RunQueue) => {
                    (FsmState::Running, Box::new(move || self.run_queue()))
                }
                // State: :running (the queue is being processed one event after another)
                (FsmState::Running, Trigger::AddEvent) => {
                    (FsmState::Running, Box::new(move || self.add_event(event)))
                }
                (FsmState::Running, Trigger::Exception) => {
                    (FsmState::Idle, Box::new(move || self.exception(event)))
                }
                (FsmState::Running, Trigger::FinishRun) => {
                    if self.queue.borrow().len() == 0 {
                        (FsmState::Idle, Box::new(|| {}))
                    } else {
                        (FsmState::Scheduled, Box::new(move || self.run_next_tick()))
                    }
                }
                _ => (FsmState::Scheduled, Box::new(|| {})),
            }
        };

        self.fsm_state.replace(new_fsm_state);
        action_fn()
    }

    pub fn push(&self, event: Event) {
        self.fsm_trigger(Trigger::AddEvent, event)
    }

    fn add_event(&self, event: Event) {
        self.queue.borrow_mut().push(event)
    }

    fn next_tick(&self, f: &Closure<dyn FnMut()>) {
        web_sys::window()
            .expect("no global `window` exists")
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("wtf");
    }

    fn run_next_tick(&self) {
        let self_cloned = self.clone();
        let next_tick = Closure::wrap(Box::new(move || {
            debug!("I'm in the closure");
            self_cloned.fsm_trigger(Trigger::RunQueue, None)
        }) as Box<dyn FnMut()>);
        self.next_tick(&next_tick);
        // we need to forget lest we want our closure to get dropped.
        // This can be a source of memory leaks CARE
        next_tick.forget()
    }

    fn process_first_event_in_queue(&self) {
        let mut queue = self.queue.borrow_mut();
        let event_v = queue.first().unwrap();
        debug!("____________process_first_event_in_queue___________");
        handle(event_v);
        queue.remove(0);
    }

    fn run_queue(&self) {
        let mut count = self.queue.borrow().len();

        loop {
            if count == 0 {
                self.fsm_trigger(Trigger::FinishRun, None);
                break;
            }
            debug!("");
            debug!("____________The Queue___________");
            debug!("");
            debug!("");
            debug!("____________The Queue___________");
            debug!("");
            self.process_first_event_in_queue();
            count -= 1;
        }
    }

    fn purge(&self) {
        self.queue.replace(vec![]);
    }

    fn exception(&self, _event: Event) {
        self.purge();
        // throw?
    }
}

pub fn dispatch(event: Box<dyn Dispatch>) {
    EVENT_QUEUE.with(|event_queue| {
        event_queue.push(Some(event))
    })
}
