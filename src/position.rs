pub trait HasPosition {
  fn get_position(&self) -> &Position;
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
  pub x: i32,
  pub y: i32,
  pub radius: i32,
}