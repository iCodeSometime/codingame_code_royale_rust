use std::fmt::Display;

use crate::unit::UnitType;
use crate::position::{Position};

pub struct TurnCommand {
  pub move_command: MoveCommand,
  pub train_location_ids: Vec<i32>,
}

pub enum MoveCommand {
  Wait,
  Move(Position),
  Build{ site_id: i32, unit_type: UnitType },
}

impl Display for TurnCommand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let move_text = self.move_command.to_string();
    let train_id_text: String = self.train_location_ids.iter().map(|l| l.to_string() + " ").collect();
    let train_text = format!("TRAIN {}", &train_id_text);
    let train_text = train_text.trim();
    write!(f, "{}\n{}\n", move_text, train_text)
  }
}

impl Display for MoveCommand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      MoveCommand::Wait => write!(f, "WAIT"),
      MoveCommand::Move(position) => write!(f, "MOVE {} {}", position.x, position.y),
      MoveCommand::Build { site_id, unit_type } => write!(f, "BUILD {} BARRACKS-{}", site_id, unit_type.to_string()),
    }
  }
}
