use miette::{Context, miette};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<String> {
    let (_, grid) = item_grid
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse item grid")?;
    let removed = core::iter::repeat(())
        .scan(grid, |grid, ()| {
            let removed = accessible_rolls(grid)
                .collect::<Vec<_>>()
                .into_iter()
                .map(|(i, j)| *grid.get_mut(i).unwrap().get_mut(j).unwrap() = Item::None)
                .count();
            Some(removed)
        })
        .take_while(|removed| *removed != 0)
        .sum::<usize>();
    Ok(removed.to_string())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Item {
    Paper,
    None,
}

fn accessible_rolls(grid: &[Vec<Item>]) -> impl Iterator<Item = (usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, item)| (i, j, item)))
        .filter(|(_, _, item)| **item == Item::Paper)
        .filter(|(i, j, _)| {
            (i.checked_sub(1).unwrap_or(*i)..=i + 1)
                .flat_map(|i| (j.checked_sub(1).unwrap_or(*j)..=j + 1).map(move |j| (i, j)))
                .filter(|(adj_i, adj_j)| adj_i != i || adj_j != j)
                .filter_map(|(i, j)| grid.get(i).and_then(|row| row.get(j)))
                .filter(|adjacent| **adjacent == Item::Paper)
                .count()
                < 4
        })
        .map(|(i, j, _)| (i, j))
}

fn item_grid(input: &str) -> IResult<&str, Vec<Vec<Item>>> {
    separated_list1(line_ending, item_row).parse(input)
}

fn item_row(input: &str) -> IResult<&str, Vec<Item>> {
    many1(item).parse(input)
}

fn item(input: &str) -> IResult<&str, Item> {
    alt((tag("."), tag("@")))
        .map(|item| match item {
            "@" => Item::Paper,
            "." => Item::None,
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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let expected = "43";
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
