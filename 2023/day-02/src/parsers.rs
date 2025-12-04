use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u32},
    combinator::iterator,
    sequence::separated_pair,
};

use crate::Set;

pub fn parse_set(s: &str) -> IResult<&str, Set> {
    let parse_color = alt((tag("red"), tag("green"), tag("blue")));
    let parse_prime = separated_pair(u32, space1, parse_color);
    let mut it = iterator(s, parse_prime);
    let (r, g, b) = it.fold((0, 0, 0), |(r, g, b), (value, color)| match color {
        "red" => (r + value as usize, g, b),
        "green" => (r, g + value as usize, b),
        "blue" => (r, g, b + value as usize),
        _ => unreachable!(),
    });
    let (rem, _) = it.finish()?;
    Ok((rem, Set { r, g, b }))
}
