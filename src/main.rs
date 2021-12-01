use anyhow::Result;

mod day1;

fn main() -> Result<()> {
    let result = day1::day1()?;
    println!("day1a: {}", result.0);
    println!("day1b: {}", result.1);

    Ok(())
}
