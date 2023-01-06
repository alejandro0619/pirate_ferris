use parser::storage::Storage;
use std::{fmt::Display, str::FromStr};

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
pub struct Model<'a> {
    sign: Sign,
    _id_sender: u64,
    id_destination: u64,
    username_destination: &'a str,
    _reason: &'a str,
}

impl From<Model<'_>> for Storage {
    fn from(value: Model) -> Self {
        match value.sign {
            Sign::Negative => Self::new(
                value.id_destination,
                value.username_destination.to_string(),
                -1,
            ),
            Sign::Positive => Self::new(
                value.id_destination,
                value.username_destination.to_string(),
                1,
            ),
        }
    }
}
impl<'a> Model<'a> {
    pub fn new(
        sign: Sign,
        sender: u64,
        dest: u64,
        username: &'a str,
        reason: &'a str,
    ) -> Model<'a> {
        Model {
            sign,
            _id_sender: sender,
            id_destination: dest,
            username_destination: username,
            _reason: reason,
        }
    }
}
