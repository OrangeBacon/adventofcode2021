use anyhow::Result;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn day2(data: &str) -> Result<(String, String)> {
    let commands = data
        .lines()
        .map(|l| l.split(' '))
        .flat_map(|mut l| Some((l.next()?, l.next()?)))
        .map(|(a, b)| Ok((a, b.parse::<i32>()?)))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flat_map(|(command, size)| {
            Some(match command {
                "forward" => Command::Forward(size),
                "down" => Command::Down(size),
                "up" => Command::Up(size),
                _ => return None,
            })
        })
        .collect::<Vec<_>>();

    let part_1 = commands.iter().fold((0, 0), |acc, cmd| match cmd {
        Command::Forward(x) => (acc.0, acc.1 + x),
        Command::Down(x) => (acc.0 + x, acc.1),
        Command::Up(x) => (acc.0 - x, acc.1),
    });
    let part_1 = part_1.0 * part_1.1;

    let part_2 = commands
        .iter()
        .fold((0, 0, 0), |(x, y, aim), cmd| match cmd {
            Command::Forward(v) => (x + v, y + aim * v, aim),
            Command::Down(v) => (x, y, aim + v),
            Command::Up(v) => (x, y, aim - v),
        });
    let part_2 = part_2.0 * part_2.1;

    Ok((part_1.to_string(), part_2.to_string()))
}
