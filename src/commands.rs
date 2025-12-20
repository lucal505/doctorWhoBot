use crate::data_structs::*;
use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;

//send random quote
#[poise::command(slash_command, prefix_command)]
pub async fn quote(ctx: Context<'_>) -> Result<(), Error> {
    let qts = &ctx.data().quotes;
    let rnd_qt = qts.choose(&mut rand::thread_rng());

    if let Some(quote) = rnd_qt {
        ctx.say(format!("**Random Quote: **{}", quote)).await?;
    } else {
        ctx.say("No quotes available.").await?;
    }

    Ok(())
}

//send doctor img
#[poise::command(slash_command, prefix_command)]
pub async fn doctor(
    ctx: Context<'_>,
    #[description = "Number of the doctor (number from 1 to 15)"] num: u8,
) -> Result<(), Error> {
    let img_url = match num {
        1 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/1-doctor.jpg",
        2 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/2-doctor.jpg",
        3 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/3-doctor.jpg",
        4 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/4-doctor.jpg",
        5 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/5-doctor.jpg",
        6 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/6-doctor.jpg",
        7 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/7-doctor.jpg",
        8 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/8-doctor.jpg",
        9 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/9-doctor.jpg",
        10 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/10-doctor.jpg",
        11 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/11-doctor.jpg",
        12 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/12-doctor.jpg",
        13 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/13-doctor.jpg",
        14 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/14-doctor.jpg",
        15 => "https://thedoctorwhosite.co.uk/wp-images/doctorwho/characters/15-doctor.jpg",
        _ => "",
    };

    if num > 15 {
        ctx.say("Please enter a valid Doctor number between 1 and 15.")
            .await?;
        return Ok(());
    }

    ctx.send(
        poise::CreateReply::default()
            .content(format!("Here's a image with the {}th Doctor:", num))
            .embed(
                serenity::CreateEmbed::new()
                    .image(img_url)
                    .title(format!("Doctor #{}", num)),
            ),
    )
    .await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn episode(
    ctx: Context<'_>,
    #[description = "Title or a part of the title of the episode"] phrase: String,
) -> Result<(), Error> {
    let eps = &ctx.data().episodes;
    let phrase_lower = phrase.to_lowercase();

    let ep_matches: Vec<&Episode> = eps
        .iter()
        .filter(|ep| ep.title.to_lowercase().contains(&phrase_lower))
        .collect();

    if ep_matches.is_empty() {
        ctx.say(format!("No episodes found matching \"{}\" phrase.", phrase))
            .await?;
    } else {
        for ep in ep_matches {
            ctx.send(
                poise::CreateReply::default().embed(
                    serenity::CreateEmbed::new()
                        .title(&ep.title)
                        .field("Season", ep.season.to_string(), true)
                        .field("Episode", ep.episode.to_string(), true)
                        .field("Runtime", &ep.runtime, true),
                ),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn points(ctx: Context<'_>) -> Result<(), Error> {
    let leaderboard = {
        //blocam accesul altor thread-uri la jsonu-ul cu puncte
        let points_lock = ctx
            .data()
            .points
            .lock()
            .map_err(|e| format!("[err]: Could not lock points file [{}]", e))?;

        let mut sorted_points: Vec<(&serenity::UserId, &u64)> = points_lock.iter().collect();
        sorted_points.sort_by(|a, b| b.1.cmp(a.1));

        let mut text = String::from("**🏆 Doctor Who Trivia Leaderboard 🏆**\n\n");

        if sorted_points.is_empty() {
            text.push_str("---No one has any points yet---.");
        } else {
            for (i, (user, score)) in sorted_points.iter().enumerate().take(10) {
                text.push_str(&format!("{}. <@{}> - {} point(s)\n", i + 1, user, score));
            }
        }
        text
    };
    ctx.say(leaderboard).await?;
    Ok(())
}
