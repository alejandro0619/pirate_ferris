use crate::library::state::KarmaModel;

use super::parser::{InputParser, Sender, Dest};
use teloxide::prelude::*;

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
                    Dest::Id(d) => {
                        // We check the sender
                        if let Sender::Id(s) = sender {
                            dbg!(KarmaModel {
                                id_destination: d,
                                id_sender: s,
                                reason: reason,
                                sign: sign,
                                username_destination: String::from("test")
                            });
                        }
                    },
                    Dest::None => {
                        bot.send_message(msg.chat.id,
                             "In order to give or take karma you have to reply to a message").await?;
                    }
                }
            }
        }
        Ok(())
    }
}
