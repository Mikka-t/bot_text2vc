use std::fs::{self, File};
use std::io::prelude::*;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Word2Word {
    Before: String,
    After: String,
}

pub async fn messageregist(data:&String, ctx: &Context, msg: &Message) {
    println!("{}",data);
    let reference: Vec<&str> = data.split([' ']).collect();
    if reference.len() != 2{
        msg.channel_id
            .say(&ctx.http, format!("usage: voicebot regist [before] [after]"))
            .await.expect("err: msg regist");
        return;
    }
    let bef = reference[0];
    let aft = reference[1];
    let w2w = Word2Word{Before:String::from(bef), After:String::from(aft)};

    let in_file = fs::read_to_string("./data/dict.json").expect("JSON READ FAILED");
    let mut param_list: Vec<Word2Word> = serde_json::from_str(&in_file).expect("json to str fail");
    param_list.push(w2w);

    let serialized: String = serde_json::to_string(&param_list).unwrap();

    // ファイル出力
    let mut out_file = File::create("./data/dict.json").expect("ERR: writing");
    out_file.write_all(serialized.as_bytes());

}