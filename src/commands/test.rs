use notion_client::endpoints::{
    databases::query::request::{QueryDatabaseRequest},
    Client,
};

use serenity::all::CreateMessage;
use serenity::model::channel::Message;
use serenity::client::Context;

pub async fn run(ctx: Context, msg: Message) {
    dotenvy::dotenv().ok();
    let client = Client::new(
        dotenvy::var("NOTION_TOKEN").unwrap_or_default(),
        None
    );

    let Ok(client) = client else {
        panic!("Notion error!!");
    };

    // Send request
    let res = client
        .databases
        .query_a_database(
            "daf91b5c9c9047de80bddedb740e4176",
            QueryDatabaseRequest { ..Default::default() } 
        )
        .await;

    print!("{:#?}", res);

    let builder = CreateMessage::new().content("hola");
    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
        println!("Error sending message: {why:?}");
    };
}