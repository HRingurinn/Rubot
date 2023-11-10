use chrono::{NaiveDate, Local, Datelike};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DailyMenu {
  date: String,
  #[serde(rename= "Title")]
  main_course: String,
  #[serde(rename= "VeganMenu")]
  vegan_course: String,
  soup_of_the_day: String,
}

fn format_week_menu(week_menu: Vec<DailyMenu>) -> String {
  let mut output = String::from("");
  for menu in week_menu.iter() {
    output += format_specific_day(menu).as_str();
  }
  return output;
}

fn format_specific_day(menu: &DailyMenu) -> String {
  let date = NaiveDate::parse_from_str(&menu.date, "%Y-%m-%d").expect("Error parsing lunch date");

  format!("## {}:\n\t🍴 {}\n\t🥬 {}\n\t🍲 {}\n",
      date.format("%A, %-d %b"),
      menu.main_course,
      menu.vegan_course,
      menu.soup_of_the_day)
}

fn get_day_of_week() -> usize {
  let today = Local::now();
  today.weekday().num_days_from_monday() as usize
}

pub async fn run(options: &[CommandDataOption]) -> Result<String, String> {
  let week_menu = this_weeks_menu().await.expect("Some error happend");

  let output = match options.get(0)  {
        Some(arg) => {
          let value = arg.value.as_ref().expect("Error in argument").as_str().expect("Argument not of type str");
          match value {
            "week" => format_week_menu(week_menu),
            _ => "Unknown argument".to_string(),
          }
        },
        None => {
          let weekday = get_day_of_week();
          if weekday > week_menu.len() {
            "No lunch today".to_string()
          } else {
            format_specific_day(&week_menu[weekday])
          }
        }
    };

  Ok(output)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command.name("lunch").description("lunch.").create_option(|option| {
    option
    .name("Argument")
    .description("Currently implemented arguments: week")
    .kind(CommandOptionType::String)
    .required(false)
  })
}

pub async fn this_weeks_menu() -> Result<Vec<DailyMenu>, reqwest::Error> {
  let url = "https://prod-198.westeurope.logic.azure.com/workflows/cc7c4c7157b14d5ba688859712303172/triggers/manual/paths/invoke?api-version=2016-06-01&sp=%2Ftriggers%2Fmanual%2Frun&sv=1.0&sig=cRM1huMwILXk-jf6xybnCcTRpnSxjKY53jFwwUGLx14";
  let menu: Vec<DailyMenu> = reqwest::get(url).await?.json::<Vec<DailyMenu>>().await?;

  return Ok(menu);
}
