use std::collections::HashMap;

use iron::prelude::*;
use iron::{Handler};
use iron::status;

pub struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}



#[cfg(test)]
mod tests {
    use Router;
    use iron::prelude::*;
    use iron::status;

    #[test]
    fn test_nothing() {
        let mut router = Router::new();
        router.add_route("123".to_string(), |_: &mut Request| {
            Ok(Response::with((status::Ok, "Hello world !")))
        });
        assert!(router.routes.len() == 1);
    }
}
