pub fn puzzle_one(input: &str) -> u32 {
    let mut checksum = 0_u32;
    for line in input.trim().lines() {
        let vec: Vec<u32> = line
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();
        let min = vec.iter().min().unwrap();
        let max = vec.iter().max().unwrap();
        let diff = max - min;
        checksum += diff;
    }
    checksum
}

pub fn puzzle_two(input: &str) -> u32 {
    let mut checksum = 0_u32;
    for line in input.trim().lines() {
        let vec: Vec<u32> = line
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();
        for &num in vec.iter() {
            match vec.iter().find(|&&x| (x % num == 0 && x / num != 1)) {
                Some(x) => {
                    checksum += x / num;
                    break;
                }
                None => continue,
            }
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle_one_example() {
        assert_eq!(18, puzzle_one("5 1 9 5\n7 5 3\n2 4 6 8"));
    }

    #[test]
    fn puzzle_two_example() {
        assert_eq!(9, puzzle_two("5 9 2 8\n9 4 7 3\n3 8 6 5"));
    }
}
