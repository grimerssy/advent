use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    rc::Rc,
};

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
    let (_, boxes) = junction_boxes
        .parse(input)
        .map_err(|err| miette!("{err}"))
        .context("parse junction boxes")?;
    let circuits = boxes
        .iter()
        .map(|b| (*b, Rc::new(HashSet::from([*b]))))
        .collect::<HashMap<_, _>>();
    let mut pairs = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, from)| boxes.iter().take(i).map(move |to| (from, to)))
        .collect::<Vec<_>>();
    pairs.sort_unstable_by(|left, right| {
        JunctionBox::distance(left.0, left.1)
            .partial_cmp(&JunctionBox::distance(right.0, right.1))
            .unwrap_or(Ordering::Equal)
    });
    let last_connection = pairs
        .into_iter()
        .scan(circuits, |circuits, (from, to)| {
            let any_circuit_len = circuits
                .values()
                .next()
                .map(|circuit| circuit.len())
                .unwrap_or(0);
            if circuits.len() == any_circuit_len {
                None
            } else {
                let circuit_union = circuits
                    .get(from)
                    .unwrap()
                    .union(circuits.get(to).unwrap())
                    .copied();
                let new_circuit = Rc::new(circuit_union.collect::<HashSet<_>>());
                new_circuit.iter().for_each(|junction_box| {
                    *circuits.get_mut(junction_box).unwrap() = new_circuit.clone()
                });
                Some((from, to))
            }
        })
        .last();
    Ok(last_connection
        .map(|(from, to)| u64::from(from.x) * u64::from(to.x))
        .unwrap_or(0))
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}

impl JunctionBox {
    fn distance(&self, other: &Self) -> f64 {
        let x = (f64::from(self.x) - f64::from(other.x)).powi(2);
        let y = (f64::from(self.y) - f64::from(other.y)).powi(2);
        let z = (f64::from(self.z) - f64::from(other.z)).powi(2);
        (x + y + z).sqrt()
    }
}

fn junction_boxes(input: &str) -> IResult<&str, Vec<JunctionBox>> {
    separated_list1(line_ending, junction_box).parse(input)
}

fn junction_box(input: &str) -> IResult<&str, JunctionBox> {
    separated_pair(
        complete::u32,
        tag(","),
        separated_pair(complete::u32, tag(","), complete::u32),
    )
    .map(|(x, (y, z))| JunctionBox { x, y, z })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn example_works() -> miette::Result<()> {
        let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689\
";
        let expected = 25272;
        let solution = solve(input)?;
        assert_eq!(solution, expected);
        Ok(())
    }
}
