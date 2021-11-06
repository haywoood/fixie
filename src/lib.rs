use std::rc::*;

pub mod router;
pub mod events;

use router::{EventQueue, Dispatch};

thread_local! {
    static EVENT_QUEUE: Rc<EventQueue<Dispatch>> = Rc::new(EventQueue::<Dispatch>::new())
}

pub fn dispatch(event: Dispatch) {
    EVENT_QUEUE.with(|event_queue| {
        event_queue.push(Some(event))
    })
}
