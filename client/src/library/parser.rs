#[allow(unused_imports)]
use super::state::{KarmaModel, Sign};
use teloxide::{
    prelude::*,
    types::MessageKind,
};

#[derive(Debug)]
pub enum Dest {
    Id(u64),
    None
}
#[derive(Debug)]
pub enum Sender {
    Id(u64),
    None
}
trait Parse {
    fn parse_user(msg: &Message) -> Self;
}
#[derive(Debug)]
pub struct InputParser;

impl Parse for Sender {
    fn parse_user(msg: &Message) -> Self {
        match msg.kind.clone() {
            MessageKind::Common(common_msg) => Sender::Id(common_msg.from.unwrap().id.0),
            _ => Sender::None, // It's probable that the match never fall in this branch because it will always be a common message
        }
    }
}
impl Parse for Dest {
  fn parse_user(msg: &Message) -> Self {
      if let Some(message) = msg.reply_to_message() {
        match message.kind.clone() {
          MessageKind::Common(common_msg) => Dest::Id(common_msg.from.unwrap().id.0),
            _ => Dest::None,
        }
      } else {
        Dest::None
      }
  }
}
impl InputParser {
    pub fn parse_input<'a>(txt: &'a str, msg: &Message) -> (Sender, Dest, Sign, &'a str) {
        match txt.split_once(char::is_numeric) {
            Some((sign, reason)) => {
                let sign = sign.parse::<Sign>().unwrap();
                // Get's the sender and the destination
                (Sender::parse_user(&msg), Dest::parse_user(&msg), sign, reason)
            },
            None => unreachable!() // Dunno how its possible to reach this point without a sender, the only reason i find is that the sender deletes the account lol
        }
    }
}
