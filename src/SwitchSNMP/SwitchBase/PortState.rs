#[derive(Debug)]
pub enum State {
  IDLE = 0,
  UP,
  DOWN,
  TESTING,
  UNKNOWN,
  DORMANT,
  NOT_PRESENT,
  LOWER_LAYER_DOWN
}

pub fn getState(value:i32) -> State {
  match value {
    0 => return State::IDLE,
    1 => return State::UP,
    2 => return State::DOWN,
    3 => return State::TESTING,
    4 => return State::UNKNOWN,
    5 => return State::DORMANT,
    6 => return State::NOT_PRESENT,
    7 => return State::LOWER_LAYER_DOWN,
    _ => return State::UNKNOWN,
  }
}