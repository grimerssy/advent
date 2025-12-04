fn main() {
    let input = include_str!("../../input.txt");
    let result = input
        .lines()
        .map(|record| record.split_once(':').unwrap().1)
        .map(|sets| sets.trim().split("; "))
        .map(|sets| sets.map(Set::from))
        .map(Set::minimal_required)
        .map(|Set { r, g, b }| r * g * b)
        .sum::<usize>();
    println!("{result}");
}

#[derive(Debug, Default, Clone)]
struct Set {
    r: usize,
    g: usize,
    b: usize,
}

impl Set {
    fn new() -> Self {
        Self::default()
    }

    fn minimal_required(sets: impl Iterator<Item = Self>) -> Self {
        sets.fold(Set::new(), |Set { r, g, b }, set| Set {
            r: r.max(set.r),
            g: g.max(set.g),
            b: b.max(set.b),
        })
    }
}

impl From<&str> for Set {
    fn from(s: &str) -> Self {
        s.split(", ")
            .map(|s| s.split_once(' ').unwrap())
            .map(|(count, color)| (count.parse::<usize>().unwrap(), color))
            .fold(Set::new(), |Set { r, g, b }, (count, color)| match color {
                "red" => Set { r: r + count, g, b },
                "green" => Set { r, g: g + count, b },
                "blue" => Set { r, g, b: b + count },
                _ => unreachable!(),
            })
    }
}
