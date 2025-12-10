use std::ops::Add;

use miette::{Context, miette};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
};
use z3::{Optimize, SatResult, ast::Int};

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<u64> {
    let (_, machines) = machines
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse machines")?;
    let total_presses = machines.iter().map(solve_machine).map(Option::unwrap).sum();
    Ok(total_presses)
}

fn solve_machine(machine: &Machine) -> Option<u64> {
    let optimizer = Optimize::new();
    let presses = (0..machine.buttons.len() as u32)
        .map(Int::new_const)
        .collect::<Vec<_>>();
    presses
        .iter()
        .for_each(|press| optimizer.assert(&press.ge(0)));
    machine
        .joltages
        .iter()
        .copied()
        .enumerate()
        .for_each(|(counter, joltage_requirement)| {
            let joltage = machine
                .buttons
                .iter()
                .zip(presses.iter())
                .filter(|(button, _)| button.contains(&counter))
                .map(|(_, press)| Int::clone(press))
                .reduce(Add::add)
                .unwrap_or(Int::from(0));
            optimizer.assert(&joltage.eq(joltage_requirement))
        });
    let total_presses = presses.into_iter().reduce(Add::add).unwrap_or(Int::from(0));
    optimizer.minimize(&total_presses);
    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer.get_model().expect("there to be a solution");
            model
                .eval(&total_presses, true)
                .map(|presses| presses.as_u64().unwrap())
        }
        SatResult::Unsat | SatResult::Unknown => None,
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

#[derive(Clone, Debug)]
struct Machine {
    #[allow(unused)]
    lights: Vec<Light>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

fn machines(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(line_ending, machine).parse(input)
}

fn machine(input: &str) -> IResult<&str, Machine> {
    separated_pair(
        indicator_lights,
        space1,
        separated_pair(button_wirings, space1, joltage_requirements),
    )
    .map(|(lights, (buttons, joltage))| Machine {
        lights,
        buttons,
        joltages: joltage,
    })
    .parse(input)
}

fn indicator_lights(input: &str) -> IResult<&str, Vec<Light>> {
    delimited(tag("["), many1(indicator_light), tag("]")).parse(input)
}

fn button_wirings(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(space1, button_wiring).parse(input)
}

fn joltage_requirements(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(tag("{"), separated_list1(tag(","), complete::u32), tag("}")).parse(input)
}

fn indicator_light(input: &str) -> IResult<&str, Light> {
    alt((tag("."), tag("#")))
        .map(|light| match light {
            "." => Light::Off,
            "#" => Light::On,
            _ => unreachable!(),
        })
        .parse(input)
}

fn button_wiring(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("("),
        separated_list1(tag(","), complete::usize),
        tag(")"),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}/
";
        let expected = 33;
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
