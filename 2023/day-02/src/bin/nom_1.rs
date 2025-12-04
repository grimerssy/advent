use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u32},
    combinator::iterator,
    sequence::separated_pair,
    IResult,
};

fn main() {
    // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let input = "1 red6 blue2 green";
    let result = Set::parse(input);
    println!("{result:?}");
}

#[derive(Clone, Debug, Default)]
struct Set {
    r: usize,
    g: usize,
    b: usize,
}

impl Set {
    fn parse(s: &str) -> IResult<&str, Self> {
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
        Ok((rem, Self { r, g, b }))
    }
}
