use std::fs::{self, File};
use std::io::prelude::*;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[allow(non_snake_case)]
struct User2Voice {
    User: u64,
    Voice: u32,
}

pub async fn voiceregist(data:&String, ctx: &Context, msg: &Message) {
    println!("{}",data);
    let usrid = &msg.author.id;
    let mut chara = 1;
    let mut chara_str = "ずんだもん";

    // なんでこんなに変なコードなんですか？直すのめんどくさかったので……
    if data == "ずんだもん"{
        chara = 1;
        chara_str = "ずんだもん";
    }
    else if data == "ずんだもん2"{
        chara = 3;
        chara_str = "ずんだもん2";
    }
    else if data == "めたん"{
        chara = 0;
        chara_str = "めたん";
    }
    else if data == "めたん2"{
        chara = 2;
        chara_str = "めたん2";
    }
    else if data == "つむぎ"{
        chara = 8;
        chara_str = "つむぎ";
    }
    else if data == "ひまり"{
        chara = 14;
        chara_str = "ひまり";
    }
    else if data == "はう"{
        chara = 10;
        chara_str = "はう";
    }
    else if data == "そら"{
        chara = 15;
        chara_str = "そら";
    }
    else{
        msg.channel_id
            .say(&ctx.http, format!("voicebot voice {{ずんだもん, ずんだもん2, めたん, めたん2, つむぎ, ひまり, そら, はう}}"))
            .await.expect("err: voice regist");
    }

    let u2v = User2Voice{User:usrid.0, Voice:chara};

    let in_file = fs::read_to_string("./data/voice.json").expect("JSON READ FAILED");
    let mut param_list: Vec<User2Voice> = serde_json::from_str(&in_file).expect("json to str fail");
    let mut flag = false;
    for idx in 0..param_list.len(){
        if param_list[idx].User == usrid.0 {
            param_list[idx] = u2v;
            flag = true;
            break;
        }
    }
    if !flag{
        param_list.push(u2v);
    }

    let serialized: String = serde_json::to_string(&param_list).unwrap();

    // ファイル出力
    let mut out_file = File::create("./data/voice.json").expect("ERR: writing");
    out_file.write_all(serialized.as_bytes()).expect("err registering");

    msg.channel_id
        .say(&ctx.http, format!("changed: {}",chara_str))
        .await.expect("err sending");

}