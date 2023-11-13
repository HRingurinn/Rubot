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

#[tokio::main]
async fn main() {
  let rooms = [
    "M101", "M102", "M103", "M104", "M105", "M106", "M107", "M108", "M109", "M110", "M111", "M112", "M113", "M114",
    "M115", "M116", "M117", "M118", "M119", "M120", "M121", "M122", "M123", "M124", "M201", "M208", "M209", "M325",
    "M326", "V101", "V102", "V103", "V104", "V105", "V106", "V107", "V108", "V109", "V110", "V111", "V112", "V113",
    "V114", "V116", "V117", "V118", "V201", "V206", "V207", "V209", "U201",
  ];

  for room in rooms {
    if let Err(y) = class_room(room.to_string()).await {
      println!("error: {:?}", y);
    }
  }
}

async fn class_room(room: String) -> Result<(), reqwest::Error> {
  let date = chrono::Utc::now().date_naive();
  let url = format!("https://utils.ru.is/api/calendars/{room}/day?date={date}");
  let time_slots: Vec<TimeSlot> = reqwest::get(url).await?.json::<Vec<TimeSlot>>().await?;

  let n = time_slots.iter().max_by(|a, b| a.end_time.cmp(&b.end_time));

  print!("{} ", room);

  match n {
    Some(course) => println!("last class at: {}", course.end_time),
    None => println!("Free all day long!"),
  }

  Ok(())
}
