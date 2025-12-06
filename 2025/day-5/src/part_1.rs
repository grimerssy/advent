use std::ops::RangeInclusive;

use miette::{Context, miette};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::many0,
    sequence::{separated_pair, terminated},
};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<String> {
    let (input, fresh_ranges) = fresh_ingredients
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse ingredients")?;
    let available = available_ingredients(input);
    let fresh = available
        .filter(|ingredient| fresh_ranges.iter().any(|range| range.contains(ingredient)))
        .count();
    Ok(fresh.to_string())
}

fn fresh_ingredients(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    many0(terminated(ingredient_range, line_ending)).parse(input)
}

fn available_ingredients(input: &str) -> impl Iterator<Item = u64> {
    input.lines().filter_map(|line| {
        ingredient
            .parse(line)
            .map(|(_, ingredient)| ingredient)
            .ok()
    })
}

fn ingredient_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(ingredient, tag("-"), ingredient)
        .map(|(start, end)| start..=end)
        .parse(input)
}

fn ingredient(input: &str) -> IResult<&str, u64> {
    complete::u64.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let expected = "3";
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
