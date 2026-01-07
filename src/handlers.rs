use crate::data_structs::*;
use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;
use std::fs;
use tokio::time::{self, Duration};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::Ready { data_about_bot } = event {
        println!("[info]: Authentified as: {}", data_about_bot.user.name);
        println!(
            "[info]: Active commands: {}.",
            framework.options().commands.len()
        );
    }

    if let serenity::FullEvent::Message { new_message } = event {
        //ignor mesajele date de boti
        if new_message.author.bot {
            return Ok(());
        }

        let user_answer = new_message.content.trim().to_lowercase();
        let mut is_winner = false;

        //fac un scope separat ca sa dispara lock-ul dupa ce am terminat cu el
        {
            //blocam accesul altor thread-uri la intrebarea curenta
            let mut curr_question_lock = data
                .current_question
                .lock()
                .map_err(|e| format!("Failed to lock current_question: {}", e))?;

            if let Some(question) = &*curr_question_lock
                && question.answers.contains(&user_answer)
            {
                is_winner = true;
                *curr_question_lock = None;
            };
        }

        if is_winner {
            let user_id = new_message.author.id;

            //update puncte
            {
                //blochez accesul altor thread-uri la puncte
                let mut points_lock = data
                    .points
                    .lock()
                    .map_err(|e| format!("Failed to lock points: {}", e))?;

                *points_lock.entry(user_id).or_insert(0) += 1;

                //pregatesc textul pentru json si il scriu in fisier
                let json_text = serde_json::to_string(&*points_lock)?;
                fs::write("points.json", json_text)?;
                println!("[info]: Points updated for user {}", user_id);
            }

            new_message
                .channel_id
                .say(&ctx.http, format!("Correct, <@{}>! +1 point!**", user_id))
                .await?;
        }
    }
    Ok(())
}

pub async fn trivia_loop(ctx: serenity::Context, data: Data) -> Result<(), Error> {
    let channel_id_str =
        std::env::var("TRIVIA_CHANNEL_ID").map_err(|_| "TRIVIA_CHANNEL_ID env var not found")?;

    let channel_id = channel_id_str
        .parse::<u64>()
        .map_err(|_| "TRIVIA_CHANNEL_ID is not valid")?;

    let target_channel = serenity::ChannelId::new(channel_id);

    let mut interval = time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        let should_post;
        {
            //verific daca nu e deja o intrebare activa
            let q_lock = data
                .current_question
                .lock()
                .map_err(|e| format!("Failed to lock question (read): {}", e))?;

            should_post = q_lock.is_none();
        }

        if should_post {
            let new_question = {
                let mut rng = rand::thread_rng();
                data.trivia_questions.choose(&mut rng).cloned()
            };

            if let Some(q) = new_question {
                {
                    let mut q_lock = data
                        .current_question
                        .lock()
                        .map_err(|e| format!("Failed to lock question (write): {}", e))?;

                    *q_lock = Some(q.clone());
                }

                println!("**New trivia question**: {}", q.question);

                target_channel.say(
                    &ctx.http,
                    format!(
                        "❓ **Trivia Time!** ❓\n{}\n*(First to answer correctly gets the point!)*",
                        q.question
                    ),
                ).await?;
            }
        }
    }
}
