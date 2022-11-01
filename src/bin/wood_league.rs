use rand::Rng;

extern crate code_royale;

use code_royale::input_reader::*;
use code_royale::commands::*;
use code_royale::board::*;
use code_royale::tower::*;
use code_royale::unit::*;
use code_royale::position::*;

fn main() {
  let mut game_board = read_initial_input();

  loop {
    update_board(&mut game_board);
    
    let best_move = get_best_move(&game_board);
    print!("{}", best_move);
  }
}

fn get_best_move(board: &Board) -> TurnCommand {
  let move_command = get_best_move_command(board);
  let train_location_ids = get_best_training_moves(board);

  TurnCommand {
    move_command,
    train_location_ids,
  }
}

fn get_best_move_command(board: &Board) -> MoveCommand {
  let best_command = if let Some(touched_site_id) = board.touched_site_id {
    let touched_site = board.towers.get(&touched_site_id);
    if matches!(touched_site, Some(Tower::NoStructure(_))) {
      Some(MoveCommand::Build { 
        site_id: touched_site_id,
        unit_type: if rand::thread_rng().gen_range(0..100) > 50 {
          UnitType::Knight
        } else {
          UnitType::Archer
        },
      })
    } else {
      None
    }
  } else {
    None
  };

  if let Some(command) = best_command {
    command
  } else {
      let friendly_queen = &board.friendly_units[board.friendly_queen_index.unwrap() as usize];
      let unbuilt_towers: Vec<&Tower> = board.towers.values().filter(|tower| matches!(tower, Tower::NoStructure(_))).collect();
      let closest = get_closest(&friendly_queen.position, &unbuilt_towers);

      if let Some(tower) = closest {
        MoveCommand::Move(*tower.get_position())
      } else {
        MoveCommand::Wait
      }
  }
}

fn get_best_training_moves(board: &Board) -> Vec<i32> {
  let mut result: Vec<i32> = vec![];
  for item in &board.towers {
    let tower = item.1;
    if let Tower::Barracks(barracks) = tower {
      if matches!(barracks.base.current_owner, TowerOwner::Friendly) && barracks.training_turns_left == 0 {
        result.push(*item.0);
      }
    };
  }

  result.into_iter().take(board.gold as usize / 80).collect()
}

fn get_closest<'a, T>(position: &Position, towers: &'a Vec<&T>) -> Option<&'a T> where T: HasPosition {
  let mut closest: Option<&T> = None;
  let mut closest_distance: f32 = BOARD_WIDTH as f32 + BOARD_HEIGHT as f32;

  for item in towers {
    let tower = *item;
    let tower_position = tower.get_position();
    let distance = f32::sqrt(
      ((position.x - tower_position.x).pow(2) +
      (position.y - tower_position.y).pow(2)) as f32);
    if distance < closest_distance {
      closest_distance = distance;
      closest = Some(tower);
    }
  }
  return closest;
}