use core::fmt;
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
    let (_, red_tiles) = red_tiles(input)
        .map_err(|err| miette!("{err}"))
        .context("parse red tiles")?;
    let polygon_edges = red_tiles
        .iter()
        .copied()
        .cycle()
        .skip(1)
        .zip(red_tiles.iter().copied())
        .map(|(to, from)| Edge { from, to })
        .inspect(|edge| debug_assert!((edge.from.x == edge.to.x) != (edge.from.y == edge.to.y)));
    let rectangles = red_tiles
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, from)| red_tiles.iter().copied().take(i).map(move |to| (from, to)))
        .map(|(from, to)| Rectangle::new(from, to));
    let max_area = rectangles
        .filter(|rectangle| {
            rectangle.edges().all(|edge| {
                polygon_edges
                    .clone()
                    .all(|poly_edge| !edge.intersects(&poly_edge) || poly_edge.contains(&edge))
            })
        })
        .map(|rectangle| rectangle.area())
        .max()
        .unwrap_or(0);
    Ok(max_area)
}

#[derive(Clone, Debug)]
struct Rectangle {
    forming_corners: [TileCoordinate; 2],
}

#[derive(Clone, Debug)]
struct Edge {
    from: TileCoordinate,
    to: TileCoordinate,
}

#[derive(Clone, Copy)]
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

impl Rectangle {
    fn new(a: TileCoordinate, b: TileCoordinate) -> Self {
        Self {
            forming_corners: [a, b],
        }
    }

    fn area(&self) -> u64 {
        let [a, b] = self.forming_corners;
        a.x.abs_diff(b.x).add(1) * a.y.abs_diff(b.y).add(1)
    }

    fn corners(&self) -> impl Iterator<Item = TileCoordinate> {
        self.forming_corners
            .iter()
            .copied()
            .flat_map(|TileCoordinate { x, y: _ }| {
                self.forming_corners
                    .iter()
                    .copied()
                    .map(move |TileCoordinate { x: _, y }| TileCoordinate { x, y })
            })
    }

    fn edges(&self) -> impl Iterator<Item = Edge> {
        self.corners()
            .enumerate()
            .flat_map(|(i, from)| self.corners().take(i).map(move |to| (from, to)))
            .filter(|(from, to)| (from.x == to.x) != (from.y == to.y))
            .map(|(from, to)| Edge { from, to })
    }
}

impl Edge {
    fn intersects(&self, other: &Self) -> bool {
        let self_min_x = self.from.x.min(self.to.x);
        let self_max_x = self.from.x.max(self.to.x);
        let self_min_y = self.from.y.min(self.to.y);
        let self_max_y = self.from.y.max(self.to.y);
        let other_min_x = other.from.x.min(other.to.x);
        let other_max_x = other.from.x.max(other.to.x);
        let other_min_y = other.from.y.min(other.to.y);
        let other_max_y = other.from.y.max(other.to.y);
        match (self.is_vertical(), other.is_vertical()) {
            (true, true) => {
                self_min_x == other_min_x
                    && ((self_min_y + 1..self_max_y).contains(&other_min_y)
                        || (self_min_y + 1..self_max_y).contains(&other_max_y)
                        || (other_min_y + 1..other_max_y).contains(&self_min_y)
                        || (other_min_y + 1..other_max_y).contains(&self_max_y))
            }
            (false, false) => {
                self_min_y == other_min_y
                    && ((self_min_x + 1..self_max_x).contains(&other_min_x)
                        || (self_min_x + 1..self_max_x).contains(&other_max_x)
                        || (other_min_x + 1..other_max_x).contains(&self_min_x)
                        || (other_min_x + 1..other_max_x).contains(&self_max_x))
            }
            (true, false) => {
                let self_x = self.from.x;
                let self_ys = self.from.y.min(self.to.y) + 1..self.from.y.max(self.to.y);
                let other_y = other.from.y;
                let other_xs = other.from.x.min(other.to.x) + 1..other.from.x.max(other.to.x);
                self_ys.contains(&other_y) && other_xs.contains(&self_x)
            }
            (false, true) => {
                let self_y = self.from.y;
                let self_xs = self.from.x.min(self.to.x) + 1..self.from.x.max(self.to.x);
                let other_x = other.from.x;
                let other_ys = other.from.y.min(other.to.y) + 1..other.from.y.max(other.to.y);
                self_xs.contains(&other_x) && other_ys.contains(&self_y)
            }
        }
    }

    fn contains(&self, other: &Self) -> bool {
        if self.is_vertical() != other.is_vertical() {
            return false;
        }
        if self.is_vertical() {
            (self.from.y.min(self.to.y)..=self.from.y.max(self.to.y)).contains(&other.from.y)
        } else {
            (self.from.x.min(self.to.x)..=self.from.x.max(self.to.x)).contains(&other.from.x)
        }
    }

    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x && self.from.y != self.to.y
    }
}

impl fmt::Debug for TileCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
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
