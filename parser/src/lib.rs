use nom::{
    bytes::complete::tag_no_case,
    character::complete::{not_line_ending, newline},
    IResult, multi::many1, sequence::tuple, combinator::opt,
};

fn parse_id(input: &str) -> IResult<&str, &str> {
    let (input, _) = tuple((opt(newline), opt(newline)))(input)?;
    let (input, _) = tag_no_case("id = ")(input)?;
    let (input, id) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    
    Ok((input, id))
}

fn parse_nick(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag_no_case("nick = ")(input)?;
    let (input, nick) = not_line_ending(input)?;
    let (input, _) = newline(input)?;

    Ok((input, nick))
}

fn parse_score(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag_no_case("score = ")(input)?;
    let (input, score) = not_line_ending(input)?;
    Ok((input, score))
}

fn parse_user(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, id) = parse_id(input)?;
    let (input, nick) = parse_nick(input)?;
    let (input, score) = parse_score(input)?;

    Ok((input, (id, nick, score)))
}
pub fn parse(input: &str) -> IResult<&str, ()>{
    let (input, result ) = many1(parse_user)(input)?;
    println!("parsed: {result:#?}");

    Ok((input, ()))
}

