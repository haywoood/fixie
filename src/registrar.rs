use std::rc::Rc;

pub enum Kinds {
    Event,
    Fx,
    Cofx,
    Sub,
}

thread_local! {
    static HANDLER_REGISTRY: Rc<HandlerRegistry> = Rc::new(HandlerRegistry::default());
}

struct HandlerRegistry {}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self { }
    }
}

pub fn register_handler(kind: Kinds, id: ) {
    
}

#[cfg(test)]
mod test {
    enum Events {
        Init,
    }

    #[test]
    fn blah() {
        
        assert!(false);
    }
}
