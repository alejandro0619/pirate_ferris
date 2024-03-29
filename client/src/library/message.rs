use super::parser::{Dest, InputParser, Sender};
use crate::library::state::Model;
use chrono::{FixedOffset, Utc};
use parser::storage::{Storage, StorageHandler};
use teloxide::prelude::*;

/// Handles the messages that doesn't match the Commands
pub struct Handler {}

impl Handler {
    pub async fn process_msg(bot: Bot, msg: Message) -> ResponseResult<()> {
        //Early return if its the current chat is not a group:
        if !msg.chat.is_group() {
            bot.send_message(
                msg.chat.id,
                "Arrgh! I don't like private chats, would you mind adding me to a group?",
            )
            .await?;
        }

        // Check if the message is a text
        if let Some(txt) = msg.text() {
            // Check if it contains +1 or -1
            if txt.contains("+1") || txt.contains("-1") {
                // We parse the input here:
                let (sender, dest, sign, reason) = InputParser::parse_input(txt, &msg);

                // We check the destination:
                match dest {
                    // We get the destination ID and nickname
                    Dest::IDandUser((d, user)) => {
                        // We check the sender
                        if let Sender::Id(s) = sender {
                            if s == d.0 {
                                bot.send_message(
                                    msg.chat.id,
                                    "Arrgh! You can't add or sub karma from yourself.",
                                )
                                .await?;
                            } else {
                                // Here we create the timestamp
                                let last_edited = Utc::now();

                                let karma =
                                    Model::new(sign, last_edited.into(), s, d.0, &user, reason);
                                let karma = Storage::from(karma);
                                if StorageHandler::update(&karma).unwrap() {
                                    println!("Sin problemas aqui");
                                    let total_karma = StorageHandler::find(karma.get_id()).unwrap();
                                    bot.send_message(
                                        msg.chat.id,
                                        format!(
                                            "{user} now has {} karma\nReason: {}",
                                            total_karma, reason
                                        ),
                                    )
                                    .await?;
                                } else {
                                    bot.send_message(
                                        msg.chat.id,
                                        "You have to wait at least 1 minute to give or take karma to the same user"
                                    ).await?;
                                }
                            }
                        }
                    }
                    Dest::None => {
                        bot.send_message(
                            msg.chat.id,
                            "Arrgh! In order to give or take karma you have to reply to a message",
                        )
                        .await?;
                    }
                }
            }
        }
        Ok(())
    }
}
