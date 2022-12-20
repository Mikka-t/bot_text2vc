use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

// https://zenn.dev/t4t5u0/articles/cd731e0293cf224cb4dc

#[command]
#[description = "nyan"]
async fn nyan(ctx: &Context, msg: &Message) -> CommandResult {
    // msg.channel_id.say で，channel_id の channel にメッセージを投稿
    msg.channel_id
        .say(&ctx.http, format!("{} にゃーん", msg.author.mention()))
        .await?;
    // CommandResultはResultを継承している
    // `Result?` は正常な値の場合，Resultの中身を返し，エラーの場合は即座にreturnする演算子
    Ok(())
}
