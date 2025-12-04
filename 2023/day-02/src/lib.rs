mod parsers;

use std::str::FromStr;

use anyhow::Context;

#[derive(Clone, Debug, Default)]
struct Set {
    r: usize,
    g: usize,
    b: usize,
}

impl FromStr for Set {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, set) = parsers::parse_set(s)?;
        Ok(set)
    }
}
