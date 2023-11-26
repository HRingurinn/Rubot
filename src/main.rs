mod commands;

use poise::serenity_prelude as serenity;
use serde_json::error;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[tokio::main]
async fn main() {
  // Configure the client with your Discord bot token in the environment.
  dotenvy::dotenv().expect("Failed to load .env file");
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      commands: vec![
        commands::malid::lunch(),
        commands::rooms::schedule(),
        commands::rooms::room(),
      ],
      ..Default::default()
    })
    .token(token)
    .intents(serenity::GatewayIntents::empty())
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        println!("Logged in as {}", _ready.user.name);
        println!("Commands: {:?}", framework.options().commands);
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
        Ok(())
      })
    });

  match framework.run().await {
    Ok(v) => v,
    Err(e) => panic!("{}", e),
  }
}
