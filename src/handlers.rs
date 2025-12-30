use crate::data_structs::*;
use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;
use std::fs;
use tokio::time::{self, Duration};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::Message { new_message } = event {
        //ignor mesajele date de boti
        if new_message.author.bot {
            return Ok(());
        }

        let user_answer = new_message.content.trim().to_lowercase();
        let mut is_winner = false;

        {
            //blocam accesul altor thread-uri la intrebarea curenta
            let mut curr_question_lock = match data.current_question.lock() {
                Ok(lock) => lock,
                Err(_) => return Ok(()),
            };

            if let Some(question) = &*curr_question_lock && question.answers.contains(&user_answer) {
                is_winner = true;
                *curr_question_lock = None;
            };
            
        }

        if is_winner {
            let user_id = new_message.author.id;

            //update puncte
            {
                //blochez accesul altor thread-uri la puncte
                if let Ok(mut points_lock) = data.points.lock() {
                    *points_lock.entry(user_id).or_insert(0) += 1;

                    if let Ok(json_text) = serde_json::to_string(&*points_lock) {
                        if let Err(e) = fs::write("points.json", json_text) {
                            eprintln!("[err]: Could not update points file [{}]", e);
                        } else {
                            println!("[info]: Points updated for user {}", user_id);
                        }
                    }
                }
            }

            if let Err(e) = new_message
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Correct, <@{}>! +1 point!**",
                        user_id
                    ),
                )
                .await
            {
                eprintln!("[err]: Could not send message [{}]", e);
            }
        }
    }
    Ok(())
}

pub async fn trivia_loop(ctx: serenity::Context, data: Data) {
    let mut interval = time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        let should_post;
        {
            //verific daca nu e deja o intrebare activa
            match data.current_question.lock() {
                Ok(q_lock) => {
                    should_post = q_lock.is_none();
                }
                Err(e) => {
                    println!("[warn]: Could not lock current question [{}]", e);
                    continue;
                }
            }
        }

        if should_post {
            let new_question = {
                let mut rng = rand::thread_rng();
                data.trivia_questions.choose(&mut rng).cloned()
            };

            if let Some(q) = new_question {
                {
                    match data.current_question.lock() {
                        Ok(mut q_lock) => *q_lock = Some(q.clone()),
                        Err(e) => {
                            println!("[warn]: Could not instantiate new question [{}]", e);
                            continue;
                        }
                    }
                }

                println!("**New trivia question**: {}", q.question);

                if let Ok(channel_string) = std::env::var("TRIVIA_CHANNEL_ID") {
                    if let Ok(channel_id) = channel_string.parse::<u64>() {
                        if let Err(e) = serenity::ChannelId::new(channel_id)
                            .say(
                                &ctx.http,
                                format!(
                                    "❓ **Trivia Time!** ❓\n{}\n*(First to answer correctly gets the point!)*",
                                    q.question
                                ),
                            )
                            .await
                        {
                            eprintln!("[warn]: Could not send trivia question [{}]", e);
                        }
                    } else {
                        eprintln!("[warn]: TRIVIA_CHANNEL_ID invalid");
                    }
                } else {
                    eprintln!("[warn]: TRIVIA_CHANNEL_ID not set");
                }
            }
        }
    }
}
