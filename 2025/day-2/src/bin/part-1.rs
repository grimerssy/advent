use day_2::part_1::solve;
use miette::Context;
use tracing_subscriber::EnvFilter;

fn main() -> miette::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let file = include_str!("../../../../input.txt");
    let solution = solve(file).context("solve part 1")?;
    println!("{solution}");
    Ok(())
}
