use chrono::NaiveDate;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MenuItem {
  title: String,
  date: String,
  vegan_menu: String,
  soup_of_the_day: String,
}

fn format_week_menu(menu: Vec<MenuItem>) -> String {
  let mut output = String::from("");
  for dish in menu.iter() {
    output += format_specific_day(dish).as_str();
  }
  return output;
}

fn format_specific_day(dish: &MenuItem) -> String {
  let date = NaiveDate::parse_from_str(&dish.date, "%Y-%m-%d").expect("Error parsing lunch date");

  format!("## {}:\n\t🍴 {}\n\t🥬 {}\n\t🍲 {}\n",
      date.format("%A, %-d %b"),
      dish.title, dish.vegan_menu,
      dish.soup_of_the_day)
}

pub async fn run(options: &[CommandDataOption]) -> Result<String, String> {
  let menu = this_weeks_menu().await.expect("Some error happend");

  let output;

  if options.len() > 0 {
    output = String::from("");
  } else {
    output = format_week_menu(menu);
  }


  Ok(output)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("lunch").description("lunch.")
}

pub async fn this_weeks_menu() -> Result<Vec<MenuItem>, reqwest::Error> {
  let url = "https://prod-198.westeurope.logic.azure.com/workflows/cc7c4c7157b14d5ba688859712303172/triggers/manual/paths/invoke?api-version=2016-06-01&sp=%2Ftriggers%2Fmanual%2Frun&sv=1.0&sig=cRM1huMwILXk-jf6xybnCcTRpnSxjKY53jFwwUGLx14";
  let menu: Vec<MenuItem> = reqwest::get(url).await?.json::<Vec<MenuItem>>().await?;

  return Ok(menu);
}
