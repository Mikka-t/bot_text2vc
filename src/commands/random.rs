use std::{collections::HashMap, convert::TryInto, sync::Arc};

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

use rand::Rng;

// ref https://github.com/serenity-rs/songbird/
// https://github.com/serenity-rs/songbird/blob/39a6f69f2324b89d17d7200905a9737d057c0d7e/examples/serenity/voice_storage/src/main.rs#L257

struct SoundStore;

impl TypeMapKey for SoundStore {
    type Value = Arc<Mutex<HashMap<String, CachedSound>>>;
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

        let wav_names = vec!["nyan"];
        let rand_num = rand::thread_rng().gen_range(0, wav_names.len());

        // wav読む
        let wav_src = Compressed::new(
                input::ffmpeg(format!("{}{}{}","./voice/", wav_names[rand_num], ".wav")).await.expect("ファイルが見つかりません……"),
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