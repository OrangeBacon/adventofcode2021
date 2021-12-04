use anyhow::Result;

pub fn day3(data: &str) -> Result<(String, String)> {
    let data: Vec<_> = data.lines().collect();

    let (zeros, ones) = bit_counts(&data);

    let mut num = 0;
    for (zero, one) in zeros.into_iter().zip(ones) {
        num <<= 1;
        if one > zero {
            num += 1;
        }
    }

    let part_1 = num * ((!num) & ((1 << data[0].len()) - 1));

    let oxy = filter(data.clone(), |a, b| a <= b)?;
    let co2 = filter(data, |a, b| a > b)?;

    let part_2 = oxy * co2;

    Ok((part_1.to_string(), part_2.to_string()))
}

fn bit_counts(data: &[&str]) -> (Vec<usize>, Vec<usize>) {
    let ones = data
        .iter()
        .fold(vec![0usize; data[0].len()], |mut acc, line| {
            for (idx, char) in line.char_indices() {
                if char == '1' {
                    acc[idx] += 1;
                }
            }

            acc
        });

    let zeros = ones.iter().map(|o| data.len() - *o).collect::<Vec<_>>();

    (zeros, ones)
}

fn filter(mut data: Vec<&str>, compare: impl Fn(usize, usize) -> bool) -> Result<u32> {
    let mut idx = 0;
    while data.len() > 1 {
        let (zeros, ones) = bit_counts(&data);

        let is_zero = compare(zeros[idx], ones[idx]);
        let check = if is_zero { '0' } else { '1' };

        data.drain_filter(|line| line.chars().nth(idx) == Some(check))
            .for_each(drop);

        idx += 1;
    }

    Ok(u32::from_str_radix(data[0], 2)?)
}
