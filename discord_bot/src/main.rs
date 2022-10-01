use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::{Channel, Message};
use serenity::model::id::RoleId;

use serenity::prelude::GatewayIntents;
use std::env;

#[group]
#[commands(verify)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn verify(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = std::env::var("GUILD_ID").unwrap();
    let role_id = std::env::var("ROLE_ID").unwrap();
    let verification_api_addr = std::env::var("VERIFICATION_API_ADDRESS").unwrap();
    let discord_bot_secret = std::env::var("DISCORD_BOT_SECRET").unwrap();

    match msg.channel_id.to_channel(&ctx).await.unwrap() {
        Channel::Private(_) => (),
        _ => return Ok(()),
    }
    let guild = &ctx.http.get_guild(guild_id.parse().unwrap()).await.unwrap();
    msg.channel_id
        .say(&ctx, "Please give discord verification token")
        .await
        .unwrap();
    let discord_tok = msg.author.await_reply(&ctx).await.unwrap();
    let s_member = guild.member(&ctx, &msg.author.id).await;
    if s_member.is_err() {
        let _ = msg
            .channel_id
            .say(&ctx, "Please join the Discord server before verification")
            .await;
        return Ok(());
    }
    let mut s_member = s_member.unwrap();
    let nickname = s_member.clone().nick;
    if nickname.is_none() {
        let _ = msg
            .channel_id
            .say(
                &ctx,
                "Please set a nickname equal to your account username in nerdtree website",
            )
            .await;
        return Ok(());
    }
    // struct for request
    #[derive(serde::Serialize, serde::Deserialize)]
    struct APIVerfRequest {
        pub username: String,
        pub discord_token: String,
        pub discord_bot_token: String,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Response {
        pub error: Option<String>,
        pub success: bool,
    }

    // send request and verify
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/discord/verify", verification_api_addr))
        .json(&APIVerfRequest {
            username: nickname.unwrap(),
            discord_token: discord_tok.content.clone(),
            discord_bot_token: discord_bot_secret,
        })
        .send()
        .await;
    if res.is_err() {
        let _ = msg.channel_id.say(&ctx, format!("Unfortunately, I've encountered some errors on my back :'(. Please try again later. Errors Message: ```{}```", res.err().unwrap().to_string())).await;
        return Ok(());
    }
    let res = res.unwrap().json::<Response>().await;
    if res.is_err() {
        let _ = msg.channel_id.say(&ctx, format!("Unfortunately, I've encountered some errors on my back :'(. Please try again later. Errors Message: ```{}```", res.err().unwrap().to_string())).await;
        return Ok(());
    }

    let res = res.unwrap();
    if res.success {
        let is_success = s_member
            .add_role(&ctx, &RoleId(role_id.parse().unwrap()))
            .await;
        return if is_success.is_err() {
            let _ = msg.channel_id.say(&ctx, "Your verification was successful, but I failed to add the 'Nerd' role to your account :'( Contact root nodes for help").await;
            let _ = msg
                .channel_id
                .say(
                    &ctx,
                    format!(
                        "Error details: ```{}```",
                        is_success.err().unwrap().to_string()
                    ),
                )
                .await;
            Ok(())
        } else {
            let _ = msg
                .channel_id
                .say(
                    &ctx,
                    ":ballot_box_with_check: You are successfully verified!",
                )
                .await;
            Ok(())
        };
    } else {
        let _ = msg.channel_id.say(&ctx, "Oops! Probably you did something wrong or the server just couldn't handle the stress :'(").await;
        let _ = msg
            .channel_id
            .say(&ctx, format!("Error: ```{}```", res.error.unwrap()))
            .await;
        Ok(())
    }
}
