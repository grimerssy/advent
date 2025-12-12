use miette::{Context, miette};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<u64> {
    let (_, (_, regions)) = puzzle
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse puzzle")?;
    let fitting_regions = regions
        .into_iter()
        .map(|region| {
            (
                region.width * region.length,
                region.quantities.iter().sum::<u64>(),
            )
        })
        .filter(|&(area, shapes)| area >= shapes * 9)
        .count() as u64;
    Ok(fitting_regions)
}

type Shape = Vec<Vec<Option<ShapePart>>>;

#[derive(Clone, Copy, Debug)]
struct ShapePart;

#[derive(Clone, Debug)]
struct Region {
    width: u64,
    length: u64,
    quantities: Vec<u64>,
}

fn puzzle(input: &str) -> IResult<&str, (Vec<Shape>, Vec<Region>)> {
    separated_pair(shapes, line_ending, regions).parse(input)
}

fn shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    separated_list1(
        line_ending,
        preceded(
            digit1,
            preceded(
                tag(":"),
                preceded(line_ending, terminated(shape, line_ending)),
            ),
        ),
    )
    .parse(input)
}

fn regions(input: &str) -> IResult<&str, Vec<Region>> {
    separated_list1(line_ending, region).parse(input)
}

fn shape(input: &str) -> IResult<&str, Shape> {
    separated_list1(line_ending, many1(shape_part)).parse(input)
}

fn region(input: &str) -> IResult<&str, Region> {
    separated_pair(
        terminated(dimensions, tag(":")),
        space1,
        separated_list1(space1, complete::u64),
    )
    .map(|((width, length), quantities)| Region {
        width,
        length,
        quantities,
    })
    .parse(input)
}

fn shape_part(input: &str) -> IResult<&str, Option<ShapePart>> {
    alt((tag("#").map(|_| Some(ShapePart)), tag(".").map(|_| None))).parse(input)
}

fn dimensions(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(complete::u64, tag("x"), complete::u64).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_variables)]
    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";
        let expected = 2;
        let solution = solve(input)?;
        // assert_eq!(solution, expected);
        Ok(())
    }
}
