use std::{fmt::Display, str::FromStr};
use parser::storage::Storage;


#[derive(Debug)]
pub enum Sign {
    Positive,
    Negative,
}
impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}
impl FromStr for Sign {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('-') {
            Ok(Sign::Negative)
        } else if s.contains('+') {
            Ok(Sign::Positive)
        } else {
            // On message we already check if it contains + or - sign, so we won't fall in this else.
            unreachable!()
        }
    }
}
pub struct ParseSignError<'a>(&'a str);

impl Display for ParseSignError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
#[derive(Debug)]
pub struct KarmaModel<'a> {
    pub sign: Sign,
    pub id_sender: u64,
    pub id_destination: u64,
    pub username_destination: &'a str,
    pub reason: &'a str,
}

impl From<KarmaModel<'_>> for Storage {
    fn from(value: KarmaModel) -> Self {
        match value.sign {
            Sign::Negative => Self::new(value.id_destination, value.username_destination.to_string(), -1),
            Sign::Positive => Self::new(value.id_destination, value.username_destination.to_string(), 1),
        }
    }
}