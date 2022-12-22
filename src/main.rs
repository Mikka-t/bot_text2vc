
// ref https://github.com/serenity-rs/serenity/tree/current/examples
// ref https://github.com/serenity-rs/serenity


use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::env;
use std::path::PathBuf;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::framework::standard::macros::{group};
use serenity::framework::standard::{StandardFramework};
use serenity::model::{gateway::Ready};

use songbird::SerenityInit;

mod commands;
use commands::{nyan::*, ping::*, come::*, leave::*, random::*};

mod queue;
use queue::*;

#[group]
#[commands(ping, nyan, come, leave, random, queue)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Botが起動したとき
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn get_token(filename: &str) -> std::io::Result<String> {
    let path_of_cargo_toml = env::current_dir()?;
    let mut path = PathBuf::from(path_of_cargo_toml);
    // path.pop();
    // path.pop(); // commandsディレクトリから2つ上のディレクトリへ
    // 環境で変わる？気が向いたら検証する
    path.push(filename); // path of cargo.toml + filename
    println!("dir: {:?}", path.display());

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Lines<BufReader<File>> = reader.lines();

    for line in lines {
        let line = line?;

        if line.starts_with("SECRET_TOKEN:") {
            // SECRET_TOKEN:mytokenという行だった場合、mytokenを取り出す
            let token = line[13..].to_string();
            return Ok(token);
        }
    }

    // TOKEN: が見つからなかった場合はエラーを返す
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "TOKEN not found",
    ))
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("voicebot ")) // プレフィックス
        .group(&GENERAL_GROUP);

    // Login
    let token = get_token("Secret.txt")
        .expect("ERROR: TOKEN NOT FOUND"); // ファイル名をわたす
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    // メインループ
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

}
