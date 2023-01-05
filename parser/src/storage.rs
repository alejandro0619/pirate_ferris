use nom::{
    bytes::complete::tag_no_case,
    character::complete::{newline, not_line_ending, u64},
    combinator::opt,
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::fs::OpenOptions;
use std::io::*;

/// This handler manages methods such as parse, sort_popular and sort_hated
/// Such that is not needed to implement it by hand directly in the client
/// fn parse() returns an instance of Storage with id, nick and score
pub struct StorageHandler;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Storage {
    id: u64,
    nick: String,
    score: u64,
}
impl Storage {
    pub fn new(id: u64, nick: &str, score: u64) -> Self {
        Self {
            id,
            nick: nick.to_string(),
            score,
        }
    }
}

impl StorageHandler {
    /// Parses the ID to u64
    fn parse_id(input: &str) -> IResult<&str, u64> {
        let (input, _) = tuple((opt(newline), opt(newline)))(input)?;
        let (input, _) = tag_no_case("id = ")(input)?;
        let (input, id) = not_line_ending(input)?;
        let (_, id) = u64(id)?; // We parse the str to number
        let (input, _) = newline(input)?;

        Ok((input, id))
    }
    /// Parses the user's nickname
    fn parse_nick(input: &str) -> IResult<&str, &str> {
        let (input, _) = tag_no_case("nick = ")(input)?;
        let (input, nick) = not_line_ending(input)?;
        let (input, _) = newline(input)?;

        Ok((input, nick))
    }
    /// Parses the user's score
    fn parse_score(input: &str) -> IResult<&str, u64> {
        let (input, _) = tag_no_case("score = ")(input)?;
        let (input, score) = not_line_ending(input)?;
        let (_, score) = u64(score)?; // We parse the str to number
        Ok((input, score))
    }
    /// It's the "combinator" where we use ``parse_id``, ``parse_nick`` and ``parse_score``.
    /// Returns an instance of Storage.
    fn parse_user(input: &str) -> IResult<&str, Storage> {
        let (input, id) = Self::parse_id(input)?;
        let (input, nick) = Self::parse_nick(input)?;
        let (input, score) = Self::parse_score(input)?;

        Ok((input, Storage::new(id, nick, score)))
    }
    ///Exposes a handy function that returns a vector of all items.
    pub fn read(source: &str) -> IResult<&str, Vec<Storage>> {
        let (input, result) = many1(Self::parse_user)(source)?;
        println!("{result:#?}");
        Ok((input, result))
    }
    pub fn write(i: &Storage) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("example.txt")?;
            
        let input = format!("ID = {}\nNICK = {}\nSCORE = {} ", i.id, i.nick, i.score);

        if let Err(e) = writeln!(file, "{input}") {
            eprintln!("Couldn't write to file: {}", e);
        }

        Ok(())
    }
}
