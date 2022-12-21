use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "leave"]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird の初期化エラーです……").clone();
    let has_handler = manager.get(guild_id).is_some(); // VCにいるか？

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            msg.channel_id
                .say(&ctx.http, format!("{:?}", e))
                .await?;
        }

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