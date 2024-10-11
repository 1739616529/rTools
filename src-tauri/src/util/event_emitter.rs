use std::collections::HashMap;



pub struct EventEmitter {
    event: HashMap<String, Vec<Box<dyn Fn()>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        EventEmitter {
            event: HashMap::new(),
        }
    }

    pub fn on<F: Fn() + Send + Sync + 'static>(&mut self, event: &str, callback: F) {
        let events = self.event.entry(event.to_string()).or_insert(vec![]);
        events.push(Box::new(callback));

        println!("{:?}", self.event.keys())
    }

    pub fn emit(&self, event: &str) {


        println!("{:?}", self.event.keys());
        let events = self.event.get(event);


        if let Some(events) = events {
            events.iter().for_each(|cb| {
                cb()
            });
        }

    }
}
