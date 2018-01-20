use std::ops::Rem;

use itertools::Itertools;

use super::util::stdin_as_string;

fn fill_disk(initial: &str, size: usize) -> String {
    let mut a = initial.to_string();

    while a.len() < size {
        let b = a.chars()
            .rev()
            .map(|c| match c {
                '1' => '0',
                '0' => '1',
                c => c,
            })
            .collect::<String>();
        a.push_str("0");
        a.push_str(&b);
    }
    a
}

fn calculate_checksum(data: &str, size: usize) -> String {
    let mut checksum = data.chars().take(size).collect::<String>();

    while checksum.len().rem(2) == 0 {
        checksum = checksum.chars()
            .chunks(2)
            .into_iter()
            .map(|mut c| if c.next() == c.next() { '1' } else { '0' })
            .collect();
    }
    checksum
}

pub fn solve() {
    let input = stdin_as_string();

    let size1 = 272;
    let size2 = 35_651_584;

    let data1 = fill_disk(&input, size1);
    let checksum1 = calculate_checksum(&data1, size1);
    println!("Part 1: {}", checksum1);

    let data2 = fill_disk(&input, size2);
    let checksum2 = calculate_checksum(&data2, size2);
    println!("Part 2: {}", checksum2);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "10000";
        let size = 20;

        let data = fill_disk(input, size);
        let checksum = calculate_checksum(&data, size);
        assert_eq!(&checksum, "01100");
    }
}
