use std::{collections::HashMap, convert::identity, iter};

use miette::{Context, miette};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
};
use tap::prelude::*;

#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<u64> {
    let (_, machines) = machines
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse machines")?;
    let total_presses = machines.iter().map(solve_machine).sum();
    Ok(total_presses)
}

fn solve_machine(machine: &Machine) -> u64 {
    let press_requirements = HashMap::new().tap_mut(|reqs| {
        reqs.insert(vec![Light::Off; machine.lights.len()], 0);
    });
    iter::repeat(&machine.buttons)
        .scan(press_requirements, |reqs, buttons| {
            reqs.iter()
                .flat_map(|(lights, presses)| {
                    buttons.iter().map(move |button| (lights, presses, button))
                })
                .map(|(lights, presses, button)| {
                    let lights_after_press = lights
                        .iter()
                        .copied()
                        .enumerate()
                        .map(|(i, light)| {
                            if button.contains(&i) {
                                light.toggle()
                            } else {
                                light
                            }
                        })
                        .collect::<Vec<_>>();
                    (lights_after_press, presses + 1)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|(lights, presses)| {
                    reqs.entry(lights).or_insert(presses);
                });
            Some(reqs.get(&machine.lights).copied())
        })
        .find_map(identity)
        .expect("infinite iter to have enough iterations")
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

#[derive(Clone, Debug)]
struct Machine {
    lights: Vec<Light>,
    buttons: Vec<Vec<usize>>,
    #[allow(unused)]
    joltage: Vec<u64>,
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
        joltage,
    })
    .parse(input)
}

fn indicator_lights(input: &str) -> IResult<&str, Vec<Light>> {
    delimited(tag("["), many1(indicator_light), tag("]")).parse(input)
}

fn button_wirings(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(space1, button_wiring).parse(input)
}

fn joltage_requirements(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(tag("{"), separated_list1(tag(","), complete::u64), tag("}")).parse(input)
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

impl Light {
    fn toggle(self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
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
        let expected = 7;
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
