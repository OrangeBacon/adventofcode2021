use anyhow::Result;

pub fn day1(data: &str) -> Result<(String, String)> {
    let data: Vec<usize> = data.lines().map(|s| s.parse()).collect::<Result<_, _>>()?;

    let part_1 = count_greater(&data);

    let part_2: Vec<usize> = data.windows(3).map(|window| window.iter().sum()).collect();
    let part_2 = count_greater(&part_2);

    Ok((part_1.to_string(), part_2.to_string()))
}

fn count_greater(data: &[usize]) -> i32 {
    data.iter().enumerate().fold(0, |acc, (idx, &num)| {
        if num > data[idx.checked_sub(1).unwrap_or(0)] {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    #[test]
    fn day1() -> anyhow::Result<()> {
        let test = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

        assert_eq!(super::day1(test)?, (7.to_string(), 5.to_string()));

        Ok(())
    }
}
