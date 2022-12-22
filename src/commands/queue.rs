use std::{convert::TryInto, sync::Arc};

use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::{
    driver::Bitrate,
    input::{
        self,
        cached::{Compressed, Memory},
        Input,
        restartable::Restartable,
    },
};

// 公式のほぼコピペで、連続で喋ることがなくなる。すごい。

#[command]
#[description = "queue"]
async fn queue(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let wav = match args.single::<String>() {
        Ok(wav) => wav,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, format!("queue: ファイル名エラー！"))
                .await?;

            return Ok(());
        },
    };

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird のクライアントエラーです……")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        // Here, we use lazy restartable sources to make sure that we don't pay
        // for decoding, playback on tracks which aren't actually live yet.
        let source = match Restartable::ffmpeg(format!("{}{}{}","./voice/", wav, ".wav"), true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id
                    .say(&ctx.http, format!("ファイル読み込みエラー"))
                    .await?;

                return Ok(());
            },
        };

        handler.enqueue_source(source.into());

        msg.channel_id
            .say(&ctx.http, format!("キューに追加"))
            .await?;
        
    } else {
        msg.channel_id
            .say(&ctx.http, format!("今はVCにいません！"))
            .await?;
    }

    Ok(())
}









