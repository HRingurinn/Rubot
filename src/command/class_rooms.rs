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


pub async fn class_room(room: String) -> Result<(), reqwest::Error> {
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
