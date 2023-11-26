use crate::{Context, Error};
use chrono::{DateTime, Local};

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

#[poise::command(slash_command, subcommands("room"), subcommand_required)]
pub async fn schedule(_: Context<'_>) -> Result<(), Error> {
  Ok(())
}

#[poise::command(slash_command)]
pub async fn room(
  ctx: Context<'_>,
  #[min_length = 4]
  #[max_length = 4]
  room: String,
) -> Result<(), Error> {
  let date = chrono::Utc::now().date_naive();

  let url = format!("https://utils.ru.is/api/calendars/{room}/day?date={date}");
  let time_slots: Vec<TimeSlot> = reqwest::get(url).await?.json::<Vec<TimeSlot>>().await?;
  let mut msg: String = format!("## {room} \n");

  for booking in time_slots.iter() {
    msg += &format!(
      "\t- {} {}-{}\n",
      booking.course_code,
      booking.start_time,
      booking.end_time,
    )
    .to_string();
  }

  ctx.send(|b| b.content(msg).ephemeral(true)).await?;

  return Ok(());
}
