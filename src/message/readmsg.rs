use std::{convert::TryInto, sync::Arc};
use std::{fs::File, io::Write, fs};
use reqwest::*;

use serenity::model::prelude::*;
use serenity::prelude::*;

use songbird::{
    driver::Bitrate,
    input::{
        self,
        cached::{Compressed},
    },
};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct User2Voice {
    User: u64,
    Voice: u32,
}

use super::messagefix::*;


pub async fn readmsg(ctx: &Context, msg: &Message){
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let data = &msg.content;

    println!("readmsg: {}", data);
    
    let manager = songbird::get(ctx).await
        .expect("Songbird の初期化エラーです……").clone();
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let client = reqwest::Client::new();

        let data_fix = messagefix(data).await.unwrap();

        println!("fixedmsg: {}", data_fix);

        // 文字列を分割
        let texts: Vec<&str> = data_fix.split([',', '.', '、', '。', '\n', '?', '!', '？', '！']).collect();

        // ユーザーの声を取得
        let v_file = fs::read_to_string("./data/voice.json").expect("JSON READ FAILED");
        let deserialized: Vec<User2Voice> = serde_json::from_str(&v_file).unwrap();
        let mut chara: u32 = 1;
        for dic in &deserialized {
            if dic.User == msg.author.id.0 {
                chara = dic.Voice;
            } 
        }
        println!("{}",chara);

        for text_str in texts{
            let text = text_str.to_string();
            if text == "" {
                continue;
            }

            let res = client.post("http://localhost:50021/audio_query")
                .query(&[("text", text.as_str()), ("speaker", &chara.to_string())])
                .send()
                .await;
            match res{
                Err(e) => println!("ERR: {}",e),
                Ok(v) => {
                    //println!("http: {:?}",v);
                    let js = v.text().await.unwrap();
                    // println!("http: {:?}",js);

                    /*
                    let mut file = File::create("temp.json").expect("file not found.");
                    writeln!(file, "{}",js).expect("file write failed");
                    */
                    
                    let wav = client.post("http://localhost:50021/synthesis")
                        .query(&[("speaker", "1")])
                        .header("Content-type", "application/json")
                        .body(js)
                        .send()
                        .await;
                    match wav{
                        Err(we) => {
                            println!("ERR: {}",we);
                            // wav読む
                            let wav_src = Compressed::new(
                                input::ffmpeg(format!("./voice/yomiage.wav")).await.expect("ファイルが見つかりません……"),
                                Bitrate::BitsPerSecond(128_000),
                            ).expect("These parameters are well-defined.");
                            let _ = wav_src.raw.spawn_loader();
                            let source = wav_src;

                            // let _sound = handler.play_source(source.into());
                            handler.enqueue_source(source.into());
                        },
                        Ok(wv) => {
                            // println!("http: {:?}",wv);
                            println!("読み上げ: {}",text);
                            let voice = wv.bytes().await.unwrap();

                            // wav書き込み？
                            let mut file = File::create("./voice/temp.wav").expect("file not found.");
                            file.write_all(&voice).expect("file write failed");

                            // wav読む
                            let wav_src = Compressed::new(
                                input::ffmpeg(format!("./voice/temp.wav")).await.expect("ファイルが見つかりません……"),
                                Bitrate::BitsPerSecond(128_000),
                            ).expect("These parameters are well-defined.");
                            let _ = wav_src.raw.spawn_loader();
                            let source = wav_src;

                            // let _sound = handler.play_source(source.into());
                            handler.enqueue_source(source.into());
                        }
                    }
                    
                    
                },
            }
        }
    }

}