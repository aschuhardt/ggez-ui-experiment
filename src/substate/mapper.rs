use substate::SubState;
use substate::states;

pub fn from_id(id: &'static str) -> Result<Box<SubState>, String> {
    match id {
        "menu" => Ok(Box::new(states::MenuState::new())),
        "mapgen" => Ok(Box::new(states::MapGenState::new())),
        "about" => Ok(Box::new(states::AboutState::new())),
        _ => Err(format!("No sub-state was found with ID {}!", id)),
    }
}

#[cfg(test)]
mod test {
    use substate::mapper;

    #[test]
    fn mapper_returns_correct_value() {
        if let Ok(_) = mapper::from_id("menu") {
            assert!(true);
        } else {
            panic!("Didn't receive expected value from mapper!");
        }
    }

    #[test]
    fn mapper_returns_err_on_invalid_id() {
        assert!(mapper::from_id("asdf").is_err());
    }
}
