use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct TimeSlot {
  course_code: String, // TODO: find a way to mute rustAnalyser
  course_name: String,
  start_time: String,
  end_time: String,
  department_name: String,
}

pub async fn run(_options: &[CommandDataOption]) -> Result<String, String> {
  Ok("Hey, I'm alive!".to_string())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("schedule").description("Class room schedule.")
}

pub async fn schedule(room: String) -> Result<(), reqwest::Error> {
  let _rooms = [
    "M101", "M102", "M103", "M104", "M105", "M106", "M107", "M108", "M109", "M110", "M111", "M112", "M113", "M114",
    "M115", "M116", "M117", "M118", "M119", "M120", "M121", "M122", "M123", "M124", "M201", "M208", "M209", "M325",
    "M326", "V101", "V102", "V103", "V104", "V105", "V106", "V107", "V108", "V109", "V110", "V111", "V112", "V113",
    "V114", "V116", "V117", "V118", "V201", "V206", "V207", "V209", "U201",
  ];
  let date = chrono::Utc::now().date_naive();

  let url = format!("https://utils.ru.is/api/calendars/{room}/day?date={date}");
  let time_slots: Vec<TimeSlot> = reqwest::get(url).await?.json::<Vec<TimeSlot>>().await?;

  let n = time_slots.iter().max_by(|a, b| a.end_time.cmp(&b.end_time));

  println!("{room}:");

  for booking in time_slots.iter() {
    println!("{}", booking.course_code);
    println!("{}", booking.start_time);
    println!("{}", booking.end_time);
  }

  match n {
    Some(course) => println!("last class at: {}", course.end_time),
    None => println!("somthin wron"),
  }

  return Ok(());
}
