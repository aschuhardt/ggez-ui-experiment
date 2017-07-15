pub mod mapper;
pub mod states;

use ggez::event::EventHandler;

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Status {
    Transition { id: &'static str },
    Quit,
}

pub trait SubState: EventHandler {
    fn get_status(&mut self) -> Option<Status>;
}
