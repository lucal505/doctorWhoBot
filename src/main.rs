mod commands;
mod handlers;
mod misc;
mod data_structs;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::commands::*;
use crate::handlers::*;
use crate::data_structs::*;
use crate::misc::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match dotenv() {
        Ok(path) => println!("[info]: .env file found: {:?}", path),
        Err(e) => println!("[err]: .env file not found [{}]", e),
    }

    let quotes: Vec<String> = load_from_file("quotes.json");
    let episodes: Vec<Episode> = load_from_file("episodes.json");

    let trivia_questions: Vec<TriviaQuestion> = load_from_file("trivia.json");

    let points_map: HashMap<serenity::UserId, u64> = load_from_file("points.json");
    let data = Data {
        quotes,
        episodes,
        trivia_questions,
        points: Arc::new(Mutex::new(points_map)),
        current_question: Arc::new(Mutex::new(None)),
    };

    // fara tokenul de discord botul nu poate porni
    let token = std::env::var("DISCORD_TOKEN")?;
    
    // ofer permisiunile de baza botului dar ii dau acces si la continutul mesajelor trimise
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![quote(), doctor(), episode(), points()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            println!("[info]: Bot logged in as {}", ready.user.name);
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let ctx_clone = ctx.clone();
                let data_clone = data.clone();

                tokio::spawn(async move {
                    trivia_loop(ctx_clone, data_clone).await;
                });

                Ok(data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}