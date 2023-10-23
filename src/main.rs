extern crate dotenv;
extern crate serde_json;
extern crate tokio;
extern crate chrono;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct MenuItem {
  title: String,
  date: String,
  vegan_menu: String,
  soup_of_the_day: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct TimeSlot {
  course_code: String, // TODO: find a way to mute rustAnalyser
  course_name: String,
  start_time: String,
  end_time: String,
  department_name: String,
}

mod command;
use std::env;

#[tokio::main]
async fn main() {
  dotenv::dotenv().expect("Failed to load .env file");
  let _token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  if let Err(y) = malid().await {
    println!("error: {:?}", y);
  }
  if let Err(y) = class_room().await {
    println!("error: {:?}", y);
  }
}

async fn malid() -> Result<(), reqwest::Error> {
  let url = "https://prod-198.westeurope.logic.azure.com/workflows/cc7c4c7157b14d5ba688859712303172/triggers/manual/paths/invoke?api-version=2016-06-01&sp=%2Ftriggers%2Fmanual%2Frun&sv=1.0&sig=cRM1huMwILXk-jf6xybnCcTRpnSxjKY53jFwwUGLx14";
  let menu: Vec<MenuItem> = reqwest::get(url).await?.json::<Vec<MenuItem>>().await?;

  for item in menu {
    println!("{}:", item.date);
    println!("Main: {}", item.title);
    println!("Vegan: {}", item.vegan_menu);
    println!("Soup of the Day: {}", item.soup_of_the_day);
    println!("----------");
  }
  return Ok(());
}

async fn class_room() -> Result<(), reqwest::Error> {
  let date = chrono::Utc::now().date_naive();
  // TODO: get a list of all classrooms with a timetable
  let class_room = "M101";

  let url = format!("https://utils.ru.is/api/calendars/{class_room}/day?date={date}");
  let time_slots: Vec<TimeSlot> = reqwest::get(url).await?.json::<Vec<TimeSlot>>().await?;

  let n = time_slots.iter().max_by(|a, b| a.end_time.cmp(&b.end_time));

  println!("M101:");

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
