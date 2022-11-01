use std::collections::HashMap;

use crate::tower::Tower;
use crate::unit::Unit;

pub const BOARD_HEIGHT: i32 = 1000;
pub const BOARD_WIDTH: i32 = 1920;

#[derive(Debug)]
pub struct Board {
  pub height: i32,
  pub width: i32,
  pub towers: HashMap<i32, Tower>,
  pub gold: i32,
  pub touched_site_id: Option<i32>,
  pub friendly_queen_index: Option<i32>,
  pub enemy_queen_index: Option<i32>,
  pub friendly_units: Vec<Unit>,
  pub enemy_units: Vec<Unit>,
}
