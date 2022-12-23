use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::fs::{File, OpenOptions};
use std::io::{Read, BufReader, Write};

#[command]
#[description = "listen"]
async fn listen(ctx: &Context, msg: &Message) -> CommandResult {
    let data = &msg.channel_id.0;

    let mut file = OpenOptions::new()
        .append(true)
        .open("./data/channel")?;
    file.write(format!("{}\n",data).as_bytes())?;

    msg.channel_id
        .say(&ctx.http, format!("Ok!"))
        .await?;
    
    Ok(())
}
