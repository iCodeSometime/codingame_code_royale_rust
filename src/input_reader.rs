use std::collections::HashMap;
use std::convert::TryInto;
use std::io;

use crate::board::*;
use crate::tower::*;
use crate::position::*;
use crate::unit::*;

macro_rules! parse_input {
  ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub fn read_initial_input() -> Board {
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let num_sites = parse_input!(input_line, i32);
  let mut towers = HashMap::<i32, Tower>::new();

  for _ in 0..num_sites {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let site_id = parse_input!(inputs[0], i32);
    let x = parse_input!(inputs[1], i32);
    let y = parse_input!(inputs[2], i32);
    let radius = parse_input!(inputs[3], i32);
    towers.insert(site_id, Tower::NoStructure(BaseTower {
      id: site_id,
      position: Position {
        x, y, radius,
      },
      current_owner: TowerOwner::None,
    }));
  }
  
  let board = Board {
    height: BOARD_HEIGHT,
    width: BOARD_WIDTH,
    towers,
    gold: 0,
    touched_site_id: None,
    friendly_queen_index: None,
    enemy_queen_index: None,
    friendly_units: vec![],
    enemy_units: vec![],
  };
  
  board
}

pub fn update_board(board: &mut Board) {
  let tower_count = board.towers.len();
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let inputs = input_line.split(" ").collect::<Vec<_>>();
  let gold = parse_input!(inputs[0], i32);
  let touched_site_id = parse_input!(inputs[1], i32);
  board.gold = gold;
  board.touched_site_id = Some(touched_site_id);

  for _ in 0..tower_count {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let site_id = parse_input!(inputs[0], i32);
    let _ignore_1 = parse_input!(inputs[1], i32);
    let _ignore_2 = parse_input!(inputs[2], i32); // used in future leagues
    let structure_type = parse_input!(inputs[3], i32); // -1 = No structure, 2 = Barracks
    let owner = parse_input!(inputs[4], i32); // -1 = No structure, 0 = Friendly, 1 = Enemy
    let param_1 = parse_input!(inputs[5], i32);
    let param_2 = parse_input!(inputs[6], i32);

    let cur_tower = board.towers.remove(&site_id).unwrap();

    let new_tower = match cur_tower {
      Tower::NoStructure(mut inner) => {
        inner.current_owner = owner.try_into().unwrap();
        
        if matches!(structure_type.try_into().unwrap(), TowerType::Barracks) {
          Tower::Barracks(BarracksTower {
            base: inner,
            training_turns_left: param_1,
            type_training: match param_2 { // hack
              -1 => None,
              _ => Some(param_2.try_into().unwrap()),
            }
          })
        } else {
          Tower::NoStructure(inner)
        }
      },
      Tower::Barracks(mut inner) => {
        inner.base.current_owner = owner.try_into().unwrap();
        inner.training_turns_left = param_1;
        inner.type_training = match param_2 {
          -1 => None,
          _ => Some(param_2.try_into().unwrap()),
        };

        if matches!(structure_type.try_into().unwrap(), TowerType::NoStructure) {
          Tower::NoStructure(inner.base)
        } else {
          Tower::Barracks(inner)
        }
      }
    };

    board.towers.insert(site_id, new_tower);
  }

  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).unwrap();
  let num_units = parse_input!(input_line, i32);
  for _ in 0..num_units as usize {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let x = parse_input!(inputs[0], i32);
    let y = parse_input!(inputs[1], i32);
    let owner_id = parse_input!(inputs[2], i32);
    let owner: UnitOwner = owner_id.try_into().unwrap();
    let unit_type_id = parse_input!(inputs[3], i32); // -1 = QUEEN, 0 = KNIGHT, 1 = ARCHER
    let unit_type: UnitType = unit_type_id.try_into().unwrap();
    let health = parse_input!(inputs[4], i32);

    let unit = Unit {
      position: Position {
        x, y, radius: match unit_type {
          UnitType::Archer => ARCHER_RADIUS,
          UnitType::Knight => KNIGHT_RADIUS,
          UnitType::Queen => QUEEN_RADIUS,
        }
      },
      health, unit_type,
      owner: owner.try_into().unwrap(),
    };


    if matches!(unit.unit_type, UnitType::Queen) {
      // Uses Vec length to get current index - must be before push
      if matches!(unit.owner, UnitOwner::Friendly) {
        board.friendly_queen_index = Some(board.friendly_units.len().try_into().unwrap());
      } else if matches!(unit.owner, UnitOwner::Enemy) {
        board.enemy_queen_index = Some(board.enemy_units.len().try_into().unwrap());
      }
    }

    if matches!(unit.owner, UnitOwner::Friendly) {
      board.friendly_units.push(unit);
    } else if matches!(unit.owner, UnitOwner::Enemy) {
      board.enemy_units.push(unit);
    }
  }
}