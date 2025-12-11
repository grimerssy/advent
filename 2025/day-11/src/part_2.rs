use std::{collections::HashMap, fmt};

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
pub fn solve(input: &str) -> miette::Result<u64> {
    let (_, connections) = connections
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse connections")?;
    let paths = number_of_paths(
        &connections,
        &Device::Srv,
        &[Device::Fft, Device::Dac],
        &Device::Out,
    );
    Ok(paths)
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum Device {
    Srv,
    Out,
    Fft,
    Dac,
    Other(Box<str>),
}

fn number_of_paths(
    connections: &HashMap<Device, Vec<Device>>,
    start: &Device,
    must_pass: &[Device],
    end: &Device,
) -> u64 {
    let mut dp = HashMap::new();
    number_of_paths_dp(&mut dp, connections, start, must_pass, end)
}

fn number_of_paths_dp(
    dp: &mut HashMap<(Device, String), u64>,
    connections: &HashMap<Device, Vec<Device>>,
    start: &Device,
    must_pass: &[Device],
    end: &Device,
) -> u64 {
    let pass_key = must_pass
        .iter()
        .map(|node| format!("{node:?}"))
        .collect::<String>();
    let cache_key = (start.clone(), pass_key);
    if let Some(cached) = dp.get(&cache_key).copied() {
        return cached;
    }
    let computed = if start == end && must_pass.is_empty() {
        1
    } else {
        let modified_must_pass = if must_pass.contains(start) {
            Some(
                must_pass
                    .iter()
                    .filter(|&node| node != start)
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        };
        let must_pass = modified_must_pass.as_deref().unwrap_or(must_pass);
        connections
            .get(start)
            .map(|outputs| {
                outputs
                    .iter()
                    .map(|output| number_of_paths_dp(dp, connections, output, must_pass, end))
                    .sum()
            })
            .unwrap_or(0)
    };
    dp.insert(cache_key, computed);
    computed
}

fn number_of_paths_rec(
    connections: &HashMap<Device, Vec<Device>>,
    start: &Device,
    must_pass: &[Device],
    end: &Device,
) -> u64 {
    if start == end && must_pass.is_empty() {
        1
    } else {
        let modified_must_pass = if must_pass.contains(start) {
            Some(
                must_pass
                    .iter()
                    .filter(|&node| node != start)
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        };
        let must_pass = modified_must_pass.as_deref().unwrap_or(must_pass);
        connections
            .get(start)
            .map(|outputs| {
                outputs
                    .iter()
                    .map(|output| number_of_paths_rec(connections, output, must_pass, end))
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
            "svr" => Device::Srv,
            "out" => Device::Out,
            "fft" => Device::Fft,
            "dac" => Device::Dac,
            other => Device::Other(Box::from(other)),
        })
        .parse(input)
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Srv => "svr",
            Self::Out => "out",
            Self::Fft => "fft",
            Self::Dac => "dac",
            Self::Other(other) => other,
        }
        .fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out\
";
        let expected = 2;
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
