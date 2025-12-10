use std::ops::Add;

use miette::{Context, miette};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<u64> {
    let (_, tiles) = red_tiles(input)
        .map_err(|err| miette!("{err}"))
        .context("parse red tiles")?;
    let largest_area = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, tile)| tiles.iter().take(i).map(move |opposite| (tile, opposite)))
        .map(|(a, b)| a.x.abs_diff(b.x).add(1) * a.y.abs_diff(b.y).add(1))
        .max()
        .unwrap_or(0);
    Ok(largest_area)
}

#[derive(Clone, Copy, Debug)]
struct TileCoordinate {
    x: u64,
    y: u64,
}

fn red_tiles(input: &str) -> IResult<&str, Vec<TileCoordinate>> {
    separated_list1(line_ending, red_tile).parse(input)
}

fn red_tile(input: &str) -> IResult<&str, TileCoordinate> {
    separated_pair(complete::u64, tag(","), complete::u64)
        .map(|(x, y)| TileCoordinate { x, y })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3\
";
        let expected = 50;
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
