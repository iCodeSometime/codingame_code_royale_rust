use std::convert::TryFrom;

use crate::position::{ Position, HasPosition };
use crate::unit::UnitType;

pub trait HasId {
  fn get_id(&self) -> i32;
}

#[derive(Debug)]
pub enum Tower {
  NoStructure(BaseTower),
  Barracks(BarracksTower),
}

impl HasId for Tower {
  fn get_id(&self) -> i32 {
    match self {
      Self::NoStructure(inner) => inner.id,
      Self::Barracks(inner) => inner.base.id,
    }
  }
}

impl HasPosition for Tower {
  fn get_position(&self) -> &Position {
    match self {
      Tower::NoStructure(inner) => &inner.position,
      Tower::Barracks(inner) => &inner.base.position,
    }
  }
}

impl HasPosition for &Tower {
  fn get_position(&self) -> &Position {
    match *self {
      Tower::NoStructure(inner) => &inner.position,
      Tower::Barracks(inner) => &inner.base.position,
    }
  }
}

#[derive(Debug)]
pub enum TowerOwner {
  None = -1,
  Friendly = 0,
  Enemy = 1,
}

impl TryFrom<i32> for TowerOwner {
  type Error = ();
  fn try_from(v: i32) -> Result<Self, Self::Error> {
    match v {
      x if x == TowerOwner::None as i32 => Ok(TowerOwner::None),
      x if x == TowerOwner::Friendly as i32 => Ok(TowerOwner::Friendly),
      x if x == TowerOwner::Enemy as i32 => Ok(TowerOwner::Enemy),
      _ => Err(()),
    }
  }
}

pub enum TowerType {
  NoStructure = -1,
  Barracks = 2,
}

impl TryFrom<i32> for TowerType {
  type Error = ();
  fn try_from(v: i32) -> Result<Self, Self::Error> {
      match v {
          x if x == TowerType::NoStructure as i32 => Ok(TowerType::NoStructure),
          x if x == TowerType::Barracks as i32 => Ok(TowerType::Barracks),
          _ => Err(()),
      }
  }
}

#[derive(Debug)]
pub struct BaseTower {
  pub id: i32,
  pub position: Position,
  pub current_owner: TowerOwner,
}

#[derive(Debug)]
pub struct BarracksTower {
  pub base: BaseTower,
  pub training_turns_left: i32,
  pub type_training: Option<UnitType>,
}
