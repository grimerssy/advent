use std::ops::RangeInclusive;

use nom::{IResult, Parser, bytes::complete::tag, character::complete, sequence::separated_pair};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<String> {
    let sum = ids(input)
        .filter(|id| is_repeating(&id.to_string()))
        .sum::<u64>();
    Ok(sum.to_string())
}

fn is_repeating(s: &str) -> bool {
    (1..=s.len() / 2)
        .filter(|sublen| s.len().is_multiple_of(*sublen))
        .any(|sublen| all_equal(s.as_bytes().windows(sublen).step_by(sublen)))
}

fn all_equal<T>(mut it: impl Iterator<Item = T>) -> bool
where
    T: PartialEq,
{
    it.next()
        .is_some_and(|first| it.all(|other| other == first))
}

fn ids(input: &str) -> impl Iterator<Item = u64> {
    id_ranges(input).flatten()
}

fn id_ranges(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input
        .split(',')
        .filter_map(|range| id_range(range).map(|(_, range)| range).ok())
}

fn id_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(id, tag("-"), id)
        .map(|(start, end)| start..=end)
        .parse(input)
}

fn id(input: &str) -> IResult<&str, u64> {
    complete::u64.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";
        let expected = "4174379265";
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
