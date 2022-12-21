use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "come"]
async fn come(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id); // VCのID

    let connect_to = match channel_id { // channel_idがNoneか見る
        Some(channel) => channel,
        None => {
            msg.channel_id
                .say(&ctx.http, format!("VCに入ってください！"))
                .await?;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird の初期化エラーです……").clone();

    let _handler = manager.join(guild_id, connect_to).await;
    msg.channel_id
                .say(&ctx.http, format!(":+1: 参加しました"))
                .await?;

    Ok(())
}