use std::collections::HashMap;
use std::cell::RefCell;

pub struct Keeper {
    subscribers: HashMap<String, RefCell<Vec<String>>>
}

impl Keeper {
    pub fn new() -> Self {
        Keeper { subscribers: HashMap::new() }
    }

    pub fn subscribe(&mut self, event_name: String, target_url: String) -> Result<(), &str> {
        if !self.subscribers.contains_key(&event_name) {
            self.subscribers.insert(event_name.to_string(), RefCell::new(Vec::new()));
        }

        if self.subscribers[&event_name].borrow().contains(&target_url) {
            return Result::Err("already exists");
        }

        self.subscribers[&event_name].borrow_mut().push(target_url);
        return Result::Ok(());
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

        assert_eq!(kpr.subscribers.len(), 0);

        assert!(kpr.subscribe(test_event_name.to_string(), test_url.to_string()).is_ok());
        assert_eq!(kpr.subscribers.len(), 1);
        assert_eq!(kpr.subscribers[test_event_name].borrow().len(), 1);
        assert_eq!(kpr.subscribers[test_event_name].borrow()[0], test_url);

        assert!(kpr.subscribe(test_event_name.to_string(), test_url.to_string()).is_err());
        assert_eq!(kpr.subscribers[test_event_name].borrow().len(), 1);
    }
}