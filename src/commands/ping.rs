use serenity::all::CreateEmbedFooter;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::client::Context;
use serenity::model::channel::Message;
use chrono::prelude::Utc;
// color generator
use crate::utils::colors::get_random_color;
use crate::utils::ping::get_discordapi_ping;
// command
pub async fn run(ctx: Context, msg: Message) {
    // building embed
    let embed = CreateEmbed::new()
        .description(get_discordapi_ping())
        .color(get_random_color())
        .footer(CreateEmbedFooter::new("Pong!"))
        .timestamp(Utc::now());
    // building message
    let builder = CreateMessage::new().embed(embed);  
    // sending message
    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
        println!("Error sending message: {why:?}");
    }
}