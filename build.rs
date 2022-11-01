use std::{path::Path, error::Error};
extern crate rustsourcebundler;

use rustsourcebundler::Bundler;

fn main() -> Result<(), Box<dyn Error>> {
  let mut bundler: Bundler = Bundler::new(
    Path::new("src/bin/wood_league.rs"),
    Path::new("target/merge/merged.rs"));

  bundler.crate_name("code-royale");
  bundler.run();

  Ok(())
}