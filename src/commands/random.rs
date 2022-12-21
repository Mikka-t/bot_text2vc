use std::{collections::HashMap, convert::TryInto, env, sync::Arc, path::PathBuf};

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::{
    driver::Bitrate,
    input::{
        self,
        cached::{Compressed, Memory},
        Input,
    },
};

// ref https://github.com/serenity-rs/songbird/

struct SoundStore;

impl TypeMapKey for SoundStore {
    type Value = Arc<Mutex<HashMap<String, CachedSound>>>;
}

enum CachedSound {
    Compressed(Compressed),
    Uncompressed(Memory),
}

impl From<&CachedSound> for Input {
    fn from(obj: &CachedSound) -> Self {
        use CachedSound::*;
        match obj {
            Compressed(c) => c.new_handle()
                .into(),
            Uncompressed(u) => u.new_handle()
                .try_into()
                .expect("Failed to create decoder for Memory source."),
        }
    }
}

#[command]
#[description = "random"]
async fn random(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird の初期化エラーです……").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // wav読む
        
        let path_of_cargo_toml = env::current_dir()?; // wavのpathとるとこ
        let mut path = PathBuf::from(path_of_cargo_toml);
        path.push("nyan.wav"); // path of cargo.toml + filename
        println!("wav_dir: {}", path.display());

        let wav_src = Compressed::new(
                input::ffmpeg("nyan.wav").await.expect("ファイルが見つかりません……"),
                Bitrate::BitsPerSecond(128_000),
            ).expect("These parameters are well-defined.");
        let _ = wav_src.raw.spawn_loader();
        let source = wav_src;

        let _sound = handler.play_source(source.into());

        msg.channel_id
            .say(&ctx.http, format!(":wave: 退出しました"))
            .await?;
    } else {
        msg.channel_id
            .say(&ctx.http, format!("今はVCにいません！"))
            .await?;
    }

    Ok(())
}