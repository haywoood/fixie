use std::rc::*;

pub mod router;
pub mod events;

pub struct Db;

impl router::Effect for Db {}
