use std::{convert::TryInto, sync::Arc};
use reqwest::*;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;



pub async fn readmsg(ctx: &Context, msg: &Message){
    println!("pog");
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let data = &msg.content;

    println!("readmsg: {}", data);

    let client = reqwest::Client::new();

    let text = "こんにちは";
    let res = client.post("localhost:50021/audio_query")
        .query(&[("speaker", "1")])
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .body(format!("text={}", text))
        .send()
        .await;
    match res{
        Err(e) => println!("ERR: {}",e),
        Ok(v) => println!("http: {}",v.status()),
    }

}