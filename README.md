# WhoBot

A Doctor Who themed Discord bot built in Rust using the [poise](https://github.com/serenity-rs/poise) framework. Features slash commands, an automatic trivia system with a leaderboard, episode search, and random quotes.

## Features

- 🎲 **Random quotes** — sends a random Doctor Who quote
- 🩺 **Doctor images** — displays an image of any of the 15 Doctors
- 📺 **Episode search** — search episodes by title or partial title
- ❓ **Automatic trivia** — posts a trivia question every 60 seconds in a designated channel; first to answer correctly earns a point
- 🏆 **Leaderboard** — shows the top 10 trivia scorers, persisted across restarts

## Commands

| Command | Description |
|---|---|
| `/quote` | Sends a random Doctor Who quote |
| `/doctor <number>` | Shows an image of the nth Doctor (1–15) |
| `/episode <title>` | Searches episodes by title or partial title |
| `/points` | Shows the trivia leaderboard |

## Getting Started

### 1. Prerequisites

- [Rust](https://rustup.rs/) (stable)
- A Discord bot token — create one at the [Discord Developer Portal](https://discord.com/developers/applications)

### 2. Clone
```bash
git clone https://github.com/lucal505/whobot.git
cd whobot
```

### 3. Configure environment variables

Create a `.env` file in the root of the project:
```
DISCORD_TOKEN=your_bot_token_here
TRIVIA_CHANNEL_ID=your_channel_id_here
```

- `DISCORD_TOKEN` — your bot's token from the Discord Developer Portal
- `TRIVIA_CHANNEL_ID` — the ID of the channel where trivia questions will be posted automatically

### 4. Prepare data files

The bot loads data from JSON files in the project root. Create the following files:

**quotes.json** — list of quotes:
```json
["Wibbly wobbly, timey wimey.", "Allons-y!", "Geronimo!"]
```

**episodes.json** — list of episodes:
```json
[
  { "title": "Blink", "season": 3, "episode": 10, "runtime": "45 min" },
  { "title": "The Doctor Dances", "season": 1, "episode": 10, "runtime": "45 min" }
]
```

**trivia.json** — trivia questions with accepted answers:
```json
[
  { "question": "What is the Doctor's home planet?", "answers": ["gallifrey"] },
  { "question": "What species is the Doctor?", "answers": ["time lord", "timelord"] }
]
```

**points.json** — automatically created and updated by the bot, but must exist initially:
```json
{}
```

### 5. Build and run
```bash
cargo run --release
```

## Project Structure

```
whobot/
├── src/
│   ├── main.rs           # bot setup, framework init, tokio entrypoint
│   ├── commands.rs       # slash commands (quote, doctor, episode, points)
│   ├── handlers.rs       # event handler and trivia loop
│   ├── data_structs.rs   # shared data types and bot state
│   └── misc.rs           # utility functions (JSON file loader)
├── quotes.json
├── episodes.json
├── trivia.json
├── points.json
├── .env
├── Cargo.toml
└── README.md
```

## Dependencies

| Crate | Purpose |
|---|---|
| `poise` | Discord bot framework (slash + prefix commands) |
| `serenity` | Discord API client |
| `tokio` | Async runtime |
| `serde` / `serde_json` | JSON serialization |
| `dotenv` | Load environment variables from `.env` |
| `rand` | Random selection for quotes and trivia |
