use std::{convert::TryInto, sync::Arc};
use reqwest::*;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;



async fn readmsg(ctx: &Context, msg: &Message) -> CommandResult{
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let data = msg.content;

    let client = reqwest::Client::new();

    let text = "こんにちは";
    let mut res = client.post("localhost:50021/audio_query")
        .query(&[("speaker", "1")])
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(format!("text={}", text))
        .send()
        .await?;
        .unwrap();
        .await?;
        println!("http: {}",res)

    Ok(())
}