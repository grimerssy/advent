use std::ops::{Add, Mul};

use miette::{Context, miette};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{complete::tag, take},
    character::complete::{self, digit1, line_ending, space0, space1},
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
    separated_pair(numbers, line_ending, operation_row).parse(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(line_ending, digit_row)
        .map(|rows| {
            let mut rows = rows.into_iter();
            let digit_lists = rows
                .next()
                .map(|row| row.into_iter().map(|n| vec![n]).collect::<Vec<_>>())
                .unwrap_or_default();
            let digit_lists = rows.fold(digit_lists, |mut lists, row| {
                lists
                    .iter_mut()
                    .zip(row)
                    .for_each(|(digits, n)| digits.push(n));
                lists
            });
            let maybe_numbers = digit_lists.into_iter().map(|digits| {
                digits
                    .into_iter()
                    .flatten()
                    .rev()
                    .enumerate()
                    .map(|(significance, digit)| digit as u64 * u64::pow(10, significance as u32))
                    .reduce(Add::add)
            });
            maybe_numbers
                .scan(None, |prev, maybe_number| {
                    let create_new = prev.is_none();
                    *prev = maybe_number;
                    Some(maybe_number.map(|number| (create_new, number)))
                })
                .flatten()
                .fold(Vec::new(), |mut number_lists, (create_new, number)| {
                    if create_new {
                        number_lists.push(vec![number])
                    } else {
                        number_lists.last_mut().unwrap().push(number)
                    }
                    number_lists
                })
        })
        .parse(input)
}

fn digit_row(input: &str) -> IResult<&str, Vec<Option<u8>>> {
    many1(maybe_digit).parse(input)
}

fn operation_row(input: &str) -> IResult<&str, Vec<Operation>> {
    many1(preceded(space0, operation)).parse(input)
}

fn maybe_digit(input: &str) -> IResult<&str, Option<u8>> {
    take(1_usize)
        .and_then(alt((space1, digit1)))
        .map(|maybe_digit| digit.parse(maybe_digit).map(|(_, digit)| digit).ok())
        .parse(input)
}

fn digit(input: &str) -> IResult<&str, u8> {
    complete::u8.parse(input)
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
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n");
        let expected = "3263827";
        let solution = solve(&input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
