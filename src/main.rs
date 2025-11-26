use serenity::{async_trait};
use serenity::model::channel::Message;
use serenity::model::gateway::{Ready};
use serenity::prelude::*;

mod commands;
mod utils;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.dnd();
        println!("{} is connected!", ready.user.name);
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        dotenvy::dotenv().ok();
        match msg.content.as_str() {
            "!ping" => {
                commands::ping::run(ctx, msg).await;
            },
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // Login with a bot token from the environment
    let token = dotenvy::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}