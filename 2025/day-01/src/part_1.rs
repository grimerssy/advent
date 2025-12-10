use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete, sequence::pair,
};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<String> {
    let zeroes = rotations(input)
        .scan(50, |dial, rotation| {
            *dial = (*dial + rotation).rem_euclid(100);
            Some(*dial)
        })
        .filter(|dial| *dial == 0)
        .count();
    Ok(zeroes.to_string())
}

fn rotations(input: &str) -> impl Iterator<Item = i32> {
    input
        .lines()
        .filter_map(|line| rotation.parse(line).map(|(_, rotation)| rotation).ok())
}

fn rotation(input: &str) -> IResult<&str, i32> {
    pair(alt((tag("L"), tag("R"))), complete::i32)
        .map(|(direction, value)| match direction {
            "L" => -value,
            "R" => value,
            _ => unreachable!(),
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        let expected = "3";
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
