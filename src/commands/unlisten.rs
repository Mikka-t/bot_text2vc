use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::fs::{self, File, OpenOptions};
use std::io::{Read, BufReader, Write};
use std::env;

#[command]
#[description = "unlisten"]
async fn unlisten(ctx: &Context, msg: &Message) -> CommandResult {
    let chn_id = &msg.channel_id;

    let mut file = File::open("./data/channel")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;//.expect("read error");
    println!("{}",contents);
    contents = contents.replace(&format!("{}\n",chn_id), &String::from(""));
    println!("{}",contents);
    println!("{}",chn_id);
    
    let mut wf = File::create("./data/channel")?;
    wf.write_all(contents.as_bytes()).unwrap();

    msg.channel_id
        .say(&ctx.http, format!("Ok..."))
        .await?;

    Ok(())
}
