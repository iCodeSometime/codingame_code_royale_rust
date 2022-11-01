use std::fmt::Display;
use std::convert::TryFrom;

use crate::position::{ Position, HasPosition };

pub const QUEEN_RADIUS: i32 = 30;
pub const KNIGHT_RADIUS: i32 = 20;
pub const ARCHER_RADIUS: i32 = 25;

#[derive(Debug)]
pub struct Unit {
  pub position: Position,
  pub health: i32,
  pub owner: UnitOwner,
  pub unit_type: UnitType,
}

impl HasPosition for Unit {
  fn get_position(&self) -> &Position {
    &self.position
  }
}

#[derive(Debug)]
pub enum UnitType {
  Queen = -1,
  Knight = 0,
  Archer = 1,
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let name = match self {
        UnitType::Queen => "QUEEN",
        UnitType::Knight => "KNIGHT",
        UnitType::Archer => "ARCHER",
      };
      write!(f, "{}", name)
    }
}

#[derive(Debug)]
pub enum UnitOwner {
  Friendly = 0,
  Enemy = 1,
}

impl TryFrom<i32> for UnitOwner {
  type Error = ();
  fn try_from(v: i32) -> Result<Self, Self::Error> {
    match v {
      x if x == UnitOwner::Friendly as i32 => Ok(UnitOwner::Friendly),
      x if x == UnitOwner::Enemy as i32 => Ok(UnitOwner::Enemy),
      _ => Err(()),
    }
  }
}

impl TryFrom<i32> for UnitType {
  type Error = ();
  fn try_from(v: i32) -> Result<Self, Self::Error> {
    match v {
      x if x == UnitType::Queen as i32 => Ok(UnitType::Queen),
      x if x == UnitType::Knight as i32 => Ok(UnitType::Knight),
      x if x == UnitType::Archer as i32 => Ok(UnitType::Archer),
      _ => Err(()),
    }
  }
}
