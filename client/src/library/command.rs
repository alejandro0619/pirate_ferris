use teloxide::{prelude::*, utils::command::BotCommands};

#[allow(dead_code)]
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Commands {
    #[command(description = "Help")]
    Help,
    #[command(description = "Top 10 users")]
    KarmaTop,
    #[command(description = "Top 10 hated users")]
    KarmaTopHate,
    #[command(description = "Check a user's karma")]
    CheckUser,
}

impl Commands {
    pub async fn answer(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
        println!("bien");
        match cmd {
            Commands::Help => {
                bot.send_message(msg.chat.id, Commands::descriptions().to_string())
                    .await?
            }
            Commands::KarmaTop => {
                bot.send_message(msg.chat.id, "This triggers top 10 users")
                    .await?
            }
            Commands::KarmaTopHate => {
                bot.send_message(msg.chat.id, "This triggers top 10 hated users")
                    .await?
            }
            Commands::CheckUser => {
                bot.send_message(msg.chat.id, "Here you must send a username")
                    .await?
            }
        };

        Ok(())
    }
}
