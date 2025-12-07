use core::iter::once;

use miette::{Context, miette};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<u64> {
    let (_, (beam_row, splitter_field)) = manifold
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse manifold")?;
    let splitter_hits = splitter_field
        .into_iter()
        .scan(beam_row, |beams, splitters| {
            let interactions = beams.iter().zip(splitters.iter());
            let passed_beams = interactions
                .clone()
                .map(|(beam, splitter)| beam.filter(|_| splitter.is_none()));
            let split_beams = interactions
                .clone()
                .map(|(beam, splitter)| beam.filter(|_| splitter.is_some()));
            let split_left = split_beams
                .clone()
                .skip(1)
                .chain(once(None))
                .take(beams.len());
            let split_right = once(None).chain(split_beams.clone()).take(beams.len());
            let splitter_hits = split_beams.flatten().count() as u64;
            *beams = passed_beams
                .zip(split_left.zip(split_right))
                .map(|(pass, (l, r))| pass.or(l).or(r))
                .collect();
            Some(splitter_hits)
        })
        .sum::<u64>();
    Ok(splitter_hits)
}

#[derive(Clone, Copy, Debug)]
struct Beam;

#[derive(Clone, Copy, Debug)]
struct Splitter;

type BeamRow = Vec<Option<Beam>>;
type SplitterRow = Vec<Option<Splitter>>;
type SplitterField = Vec<SplitterRow>;

fn manifold(input: &str) -> IResult<&str, (BeamRow, SplitterField)> {
    separated_pair(source_row, line_ending, splitter_field).parse(input)
}

fn source_row(input: &str) -> IResult<&str, BeamRow> {
    many1(alt((empty_cell.map(|_| None), beam.map(Some)))).parse(input)
}

fn splitter_field(input: &str) -> IResult<&str, SplitterField> {
    separated_list1(line_ending, splitter_row).parse(input)
}

fn splitter_row(input: &str) -> IResult<&str, SplitterRow> {
    many1(alt((empty_cell.map(|_| None), splitter.map(Some)))).parse(input)
}

fn empty_cell(input: &str) -> IResult<&str, &str> {
    tag(".").parse(input)
}

fn beam(input: &str) -> IResult<&str, Beam> {
    tag("S").map(|_| Beam).parse(input)
}

fn splitter(input: &str) -> IResult<&str, Splitter> {
    tag("^").map(|_| Splitter).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............\
";
        let expected = 21;
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
