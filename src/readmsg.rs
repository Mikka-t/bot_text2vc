use std::{convert::TryInto, sync::Arc};
use reqwest::*;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;



pub async fn readmsg(ctx: &Context, msg: &Message){
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let data = &msg.content;

    println!("readmsg: {}", data);

    let client = reqwest::Client::new();

    let text = data; // あとで後処理追加する
    let res = client.post("http://localhost:50021/audio_query")
        .query(&[("text", text.as_str()), ("speaker", "1")])
        .send()
        .await;
    match res{
        Err(e) => println!("ERR: {}",e),
        Ok(v) => {
            println!("http: {:?}",v);
            //let js = v.json();
            let js = v.text().await;
            println!("http: {:?}",js);
            /*
            let wav = client.post("http://localhost:50021/audio_query")
                .json(&js)
                .send()
                .await;
            match res{
                Err(e) => println!("ERR: {}",e),
                Ok(v) => {
                    println!("http: {:?}",v);
                }
            }
            */
            
        },
    }

}