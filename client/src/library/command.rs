use parser::storage::StorageHandler;
use teloxide::{prelude::*, utils::command::BotCommands};
#[allow(dead_code)]
#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Commands {
    #[command(description = "Sintax: +1 [reason why] or -1 [reason why]")]
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
        match cmd {
            Commands::Help => {
                bot.send_message(msg.chat.id, Commands::descriptions().to_string())
                    .await?
            }
            Commands::KarmaTop => {
                bot.send_message(msg.chat.id, format!("{}", StorageHandler::sort()?))
                    .await?
            }
            Commands::KarmaTopHate => {
                bot.send_message(msg.chat.id, format!("{:?}", StorageHandler::rev_sort()?))
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
