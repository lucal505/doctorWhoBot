use poise::serenity_prelude as serenity;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Clone)]
pub struct Episode {
    pub title: String,
    pub season: u32,
    pub episode: u32,
    pub runtime: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TriviaQuestion {
    pub question: String,
    pub answers: Vec<String>, //lista de raspunsuri acceptate
}

#[derive(Clone)]
pub struct Data {
    pub quotes: Vec<String>,
    pub episodes: Vec<Episode>,
    pub trivia_questions: Vec<TriviaQuestion>,

    //mutex permite accesul concurent la date
    //arc permite partajarea intre firele de exec
    pub points: Arc<Mutex<HashMap<serenity::UserId, u64>>>,
    pub current_question: Arc<Mutex<Option<TriviaQuestion>>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
