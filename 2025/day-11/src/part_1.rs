use std::collections::HashMap;

use miette::{Context, miette};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
};

#[allow(unused_variables)]
#[tracing::instrument(skip(input), err)]
pub fn solve(input: &str) -> miette::Result<u32> {
    let (_, connections) = connections
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse connections")?;
    let paths = number_of_paths(&connections, &Device::You, &Device::Out);
    Ok(paths)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Device {
    You,
    Out,
    Other(Box<str>),
}

fn number_of_paths(
    connections: &HashMap<Device, Vec<Device>>,
    start: &Device,
    end: &Device,
) -> u32 {
    if start == end {
        1
    } else {
        connections
            .get(start)
            .map(|outputs| {
                outputs
                    .iter()
                    .map(|output| number_of_paths(connections, output, end))
                    .sum()
            })
            .unwrap_or(0)
    }
}

fn connections(input: &str) -> IResult<&str, HashMap<Device, Vec<Device>>> {
    separated_list1(
        line_ending,
        separated_pair(device, tag(": "), separated_list1(space1, device)),
    )
    .map(|vec| vec.into_iter().collect())
    .parse(input)
}

fn device(input: &str) -> IResult<&str, Device> {
    alpha1
        .map(|device| match device {
            "you" => Device::You,
            "out" => Device::Out,
            other => Device::Other(Box::from(other)),
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out\
";
        let expected = 5;
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
