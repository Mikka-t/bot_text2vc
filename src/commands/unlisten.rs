use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::fs::{File, OpenOptions};
use std::io::{Read, BufReader};
use std::env;

#[command]
#[description = "listen"]
async fn listen(ctx: &Context, msg: &Message) -> CommandResult {
    let chn_id = &msg.channel_id;

    let file = File::open("./data/channel")?;
    let mut contents = String::new();
    let s1 = file.read_to_string(&mut contents).expect("read error");
    let s2 = s1.replace(chn_id, "");
    
    let mut wf = fs::File::create("./data/channel")?;
    wf.write_all(s2).unwrap();

    Ok(())
}
