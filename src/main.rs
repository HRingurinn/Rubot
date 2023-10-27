mod command;
use std::env;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().expect("Failed to load .env file");

  // TODO: Read rest of env tokens
  let _token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let rooms = [
    "M101", "M102", "M103", "M104", "M105", "M106", "M107", "M108", "M109", "M110", "M111", "M112", "M113", "M114",
    "M115", "M116", "M117", "M118", "M119", "M120", "M121", "M122", "M123", "M124", "M201", "M208", "M209", "M325",
    "M326", "V101", "V102", "V103", "V104", "V105", "V106", "V107", "V108", "V109", "V110", "V111", "V112", "V113",
    "V114", "V116", "V117", "V118", "V201", "V206", "V207", "V209", "U201",
  ];

  for room in rooms {
    if let Err(_y) = command::class_rooms::class_room(room.to_string()).await {
      println!("{}", room);
    }
  }
}
