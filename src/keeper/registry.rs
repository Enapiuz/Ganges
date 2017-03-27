use std::collections::HashMap;
use std::cell::RefCell;

pub struct Keeper {
    subscribers: HashMap<String, RefCell<Vec<String>>>
}

impl Keeper {
    pub fn new() -> Self {
        Keeper { subscribers: HashMap::new() }
    }

    pub fn subscribe(&mut self, event_name: String, target_url: String) {
        if !self.subscribers.contains_key(&event_name) {
            self.subscribers.insert(event_name.to_string(), RefCell::new(Vec::new()));
        }

        self.subscribers[&event_name].borrow_mut().push(target_url);
    }
}


#[cfg(test)]
mod tests {
    use keeper::registry::Keeper;

    #[test]
    fn test_keeper_add() {
        let test_event_name = "test_event";
        let test_url = "test_url";
        let mut kpr = Keeper::new();
        assert!(kpr.subscribers.len() == 0);
        kpr.subscribe(test_event_name.to_string(), test_url.to_string());
        assert!(kpr.subscribers.len() == 1);
        assert!(kpr.subscribers[test_event_name].borrow().len() == 1);
        assert!(kpr.subscribers[test_event_name].borrow()[0] == test_url);
    }
}