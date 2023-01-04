#[allow(unused_imports)]
use super::state::{KarmaModel, Sign};
use teloxide::{prelude::*, types::MessageKind};
type ParseResponse<'a> = (Sender, Dest, Sign, &'a str);

/// This will return metainfo about the receiver
/// IDandUser will return a tuple with the ID(u64) and the nickname (String)
/// None means there's no Receive (The user didn't reply to a message)
#[derive(Debug)]
pub enum Dest {
    IDandUser((u64, String)),
    None,
}
/// This will return metainfo abou the sender
/// Id will return the User's ID (u64)
/// None means there's no Sender (probably means the sender deleted his account and we can't find the ID associated)
#[derive(Debug)]
pub enum Sender {
    Id(u64),
    None,
}
/// Receives a reference to a Message and returns the given Type
trait Parse {
    fn parse_user(msg: &Message) -> Self;
}
/// Just an interface to call the parser from it, nothing special though
#[derive(Debug)]
pub struct InputParser;


impl Parse for Sender {
    fn parse_user(msg: &Message) -> Self {
        match msg.kind.clone() {
            MessageKind::Common(cm) => Sender::Id(cm.from.unwrap().id.0),
            _ => Sender::None, // It's probable that the match never fall in this branch because it will always be a common message
        }
    }
}
impl Parse for Dest {
    fn parse_user(msg: &Message) -> Self {
        if let Some(message) = msg.reply_to_message() {
            match message.kind.clone() {
                MessageKind::Common(cm) => {
                    Dest::IDandUser((cm.clone().from.unwrap().id.0, cm.from.unwrap().first_name))
                }
                _ => unreachable!(), // Same thing here
            }
        } else {
            // This means the message (+1 or -1) where send but without replying to a message.
            // This will trigger the bot to send a message saying that is a must to reply when using it.
            Dest::None 
            
        }
    }
}

impl InputParser {
    pub fn parse_input<'a>(txt: &'a str, msg: &Message) -> ParseResponse<'a> {
        match txt.split_once(char::is_numeric) {
            Some((sign, reason)) => {
                let sign = sign.parse::<Sign>().unwrap();
                // Get's the sender and the destination
                (
                    Sender::parse_user(msg),
                    Dest::parse_user(msg),
                    sign,
                    reason,
                )
            }
            None => unreachable!(), // Dunno how its possible to reach this point without a sender, the only reason i find is that the sender deletes the account lol
        }
    }
}
