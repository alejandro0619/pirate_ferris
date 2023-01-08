use nom::{
    bytes::complete::tag_no_case,
    character::complete::{newline, not_line_ending, u64},
    combinator::opt,
    multi::many1,
    sequence::tuple,
    IResult,
};
use std::fs::OpenOptions;
use std::io::{self, Write};

/// This handler manages methods such as parse, sort_popular and sort_hated
/// Such that is not needed to implement it by hand directly in the client
/// fn parse() returns an instance of Storage with id, nick and score
pub struct StorageHandler;
#[allow(dead_code)]
#[derive(Debug)]
pub struct Storage {
    id: u64,
    nick: String,
    score: i64,
}
impl Storage {
    pub fn new(id: u64, nick: String, score: i64) -> Self {
        Self { id, nick, score }
    }
    pub fn get_id(&self) -> u64 {
        self.id
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
    pub fn parse_score(input: &str) -> IResult<&str, i64> {
        let (input, _) = tag_no_case("score = ")(input)?;
        let (input, score) = not_line_ending(input)?;
        let score = score.parse::<i64>().unwrap(); // This should never panic
        Ok((input, score))
    }
    /// It's the "combinator" where we use ``parse_id``, ``parse_nick`` and ``parse_score``.
    /// Returns an instance of Storage.
    fn parse_user(input: &str) -> IResult<&str, Storage> {
        let (input, id) = Self::parse_id(input)?;
        let (input, nick) = Self::parse_nick(input)?;
        let (input, score) = Self::parse_score(input)?;

        Ok((input, Storage::new(id, nick.to_string(), score)))
    }
    ///Exposes a handy function that returns a vector of all items.
    pub fn read(source: &str) -> IResult<&str, Vec<Storage>> {
        let (input, result) = many1(Self::parse_user)(source)?;
        //println!("{result:#?}");
        Ok((input, result))
    }
    fn write(i: &Storage) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("storage.txt")?;

        let input = format!("ID = {}\nNICK = {}\nSCORE = {}", i.id, i.nick, i.score);

        if let Err(e) = writeln!(file, "{input}") {
            eprintln!("Couldn't write to file: {}", e);
        }

        Ok(())
    }
}

impl StorageHandler {
    /// The Update method takes a Storage, if it exists chunks the Array of Users:
    /// [before the user is found] user [after the user is found]
    /// Then we edit the user and append it to [before the user is found] + user
    /// Then we append it to the rest of the array [after the user is found]
    /// Then we have our new array.
    /// If the user is not found then we create the new user and append it to the final of the file
    /// NOTE: This needs to be rewritten for efficiency, here we allocate a lot of vectors and Strings
    /// Probably one good way to approach this is to modify in place while reading the file
    /// So we don't have to save it in a buffer and a lot of the allocated vectores would be unnecesary.
    pub fn update(i: &Storage) -> std::io::Result<()> {
        let input = std::fs::read_to_string("storage.txt")?;

        let lines = input.lines(); // Converts into an iterator of lines
                                   // This chunks the iterator by 3 (id, nick and score) and collects into a vector of arrays [&str, 3]
        let vec_lines: Vec<[&str; 3]> = lines.clone().array_chunks().collect();
        // We check if the given ID is already in the document: If Some() it is and we update the score (karma)
        // If None we need to append the new user
        let is_found = lines
            .clone()
            .array_chunks::<3>()
            .enumerate()
            .find(|(_, [id, _, _])| id.contains(&format!("{}", i.id)));

        match is_found {
            Some((index, [id, nick, score])) => {
                // Here we parse the score from SCORE = number ->> number
                let (_, score) = StorageHandler::parse_score(score).unwrap();
                // We create a updated user array that contains [id, nick, score] and we turn score back into a string
                let user = [id, nick, &format!("SCORE = {}", score + i.score)];
                // We concat the array before the user was found with the item of the user
                let with_user = [&vec_lines[..index], &[user]].concat();
                // We now concat the array with the user with the rest of the array after the user was found
                // We stringify it to parse it later
                let result = [&with_user[..], &vec_lines[(index + 1)..]].concat();

                let source = result
                    .iter()
                    .map(|[id, nick, score]| format!("{id}\n{nick}\n{score}\n"))
                    .collect::<String>();
                // We are parsing it again
                let (_, parsed) = StorageHandler::read(&source).unwrap();
                std::fs::remove_file("storage.txt")?; // we remove the file
                parsed.iter().for_each(|u| {
                    // we create the file and append the content to it
                    StorageHandler::write(u).unwrap();
                });
            }
            None => {
                // If there's no user with the given id, we append the new user with it's information
                StorageHandler::write(i)?;
            }
        }
        Ok(())
    }

    pub fn find(i: u64) -> Option<i64> {
        let input = std::fs::read_to_string("storage.txt").unwrap();

        let lines = input.lines(); // Converts into an iterator of lines
        let is_found = lines
            .clone()
            .array_chunks::<3>()
            .enumerate()
            .find(|(_, [id, _, _])| id.contains(&format!("{}", i)));

        match is_found {
            Some((_index, [_id, _nick, score])) => {
                let (_, score) = Self::parse_score(score).unwrap();
                Some(score)
            }
            None => None,
        }
    }

    pub fn sort() -> io::Result<String> {
        let input = std::fs::read_to_string("storage.txt")?;
        let (_, mut users) = StorageHandler::read(&input).unwrap();
        let mut listed_top = String::new();
        // Sort the array of Users
        users.sort_by(|a, b| b.score.cmp(&a.score));

        users.iter().enumerate().for_each(|(i, u)| {
            // Only shows top ten:
            if i < 10 {
                listed_top.push_str(&format!(
                    "{}.- Name: {}\nScore: {}\n\n",
                    i + 1,
                    u.nick,
                    u.score
                ))
            }
        });
        Ok(listed_top)
    }

    pub fn rev_sort() -> io::Result<String> {
        let input = std::fs::read_to_string("storage.txt")?;
        let (_, mut users) = StorageHandler::read(&input).unwrap();
        let mut listed_hated = String::new();
        users.sort_by(|a, b| a.score.cmp(&b.score));

        users.iter().enumerate().for_each(|(i, u)| {
            // Only shows top ten:
            if i < 10 {
                listed_hated.push_str(&format!(
                    "{}.- Name: {}\nScore: {}\n\n",
                    i + 1,
                    u.nick,
                    u.score
                ))
            }
        });
        Ok(listed_hated)
    }
}
