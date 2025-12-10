use std::ops::{Add, Mul};

use miette::{Context, miette};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space0},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<String> {
    let (_, (numbers, operations)) = worksheet
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse worksheet")?;
    let answers = numbers
        .into_iter()
        .zip(operations)
        .map(|(operands, operation)| operands.into_iter().reduce(operation).unwrap_or(0));
    Ok(answers.sum::<u64>().to_string())
}

type Operation = fn(u64, u64) -> u64;

fn worksheet(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operation>)> {
    separated_pair(number_lists, line_ending, operation_row).parse(input)
}

fn number_lists(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(line_ending, number_row)
        .map(|rows| {
            let mut rows = rows.into_iter();
            let lists = rows
                .next()
                .map(|row| row.into_iter().map(|n| vec![n]).collect::<Vec<_>>())
                .unwrap_or_default();
            rows.fold(lists, |mut lists, row| {
                lists.iter_mut().zip(row).for_each(|(list, n)| list.push(n));
                lists
            })
        })
        .parse(input)
}

fn number_row(input: &str) -> IResult<&str, Vec<u64>> {
    many1(preceded(space0, number)).parse(input)
}

fn operation_row(input: &str) -> IResult<&str, Vec<Operation>> {
    many1(preceded(space0, operation)).parse(input)
}

fn number(input: &str) -> IResult<&str, u64> {
    complete::u64.parse(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    alt((tag("+"), tag("*")))
        .map(|operation| match operation {
            "+" => Add::add,
            "*" => Mul::mul,
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
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  \
";
        let expected = "4277556";
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
