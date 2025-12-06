use std::{cmp::Reverse, ops::RangeInclusive};

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
    let (_, mut fresh_ranges) = fresh_ingredients
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse ingredients")?;
    fresh_ranges.sort_unstable_by_key(|range| Reverse(range.clone().count()));
    let fresh = fresh_ranges
        .iter()
        .enumerate()
        .flat_map(|(i, range)| {
            fresh_ranges
                .iter()
                .take(i)
                .fold(range.clone(), |mut range, prev| {
                    if !range.is_empty() {
                        let start = match range.clone().next().unwrap() {
                            start if prev.contains(&start) => prev.clone().next_back().unwrap() + 1,
                            start => start,
                        };
                        let end = match range.clone().next_back().unwrap() {
                            end if prev.contains(&end) => prev.clone().next().unwrap() - 1,
                            end => end,
                        };
                        range = start..=end;
                    }
                    range
                })
        })
        .count();
    Ok(fresh.to_string())
}

fn fresh_ingredients(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    many0(terminated(ingredient_range, line_ending)).parse(input)
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
";
        let expected = "14";
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
