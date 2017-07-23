use std::collections::HashMap;

use substate::Status;

pub enum StoredValue {
    FloatingPoint { value: f32 },
    Integral { value: i32 },
    Boolean { value: bool },
    Textual { value: String },
}

pub struct StateInfo {
    status: Option<Status>,
    data_store: HashMap<&'static str, StoredValue>,
    should_refresh_ui: bool,
}

impl StateInfo {
    pub fn new() -> StateInfo {
        StateInfo {
            status: None,
            data_store: HashMap::<&'static str, StoredValue>::new(),
            should_refresh_ui: false,
        }
    }

    pub fn get_status(&mut self) -> Option<Status> {
        let status = self.status.clone();
        self.status = None;
        status
    }

    pub fn transition(&mut self, id: &'static str) {
        self.status = Some(Status::Transition { id: id });
    }

    pub fn quit(&mut self) {
        self.status = Some(Status::Quit);
    }

    pub fn set_value(&mut self, key: &'static str, value: StoredValue) {
        self.data_store.insert(key, value);
    }

    pub fn get_value(&mut self, key: &'static str) -> Result<&StoredValue, String> {
        if self.data_store.contains_key(key) {
            Ok(&self.data_store[key])
        } else {
            Err(format!("No value is stored with key {}!", key))
        }
    }

    pub fn refresh_ui(&mut self) {
        self.should_refresh_ui = true;
    }

    pub fn is_ui_dirty(&mut self) -> bool {
        if self.should_refresh_ui {
            self.should_refresh_ui = false;
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod test {
    use super::StateInfo;
    use substate::Status;

    #[test]
    fn state_transition_status_works_correctly() {
        let mut info_manager = StateInfo::new();

        //first make sure that the status defaults to None
        assert!(info_manager.get_status().is_none());

        //set the info manager to indicate that it's ready to transition
        info_manager.transition("test");

        //ensure that the new status of the info manager is Transition and that
        //the transition's state id is "test"
        if let Some(status) = info_manager.get_status() {
            assert!(status == Status::Transition { id: "test" });
        } else {
            panic!("Status of info manager was not what was expected!");
        }

        //make sure that the status of the info manager has been reset to None
        assert!(info_manager.get_status().is_none());
    }
}
