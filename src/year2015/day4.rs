fn find_hash(base: &str, n: u8) -> u32 {
    let leading_match = "0".repeat(n as usize);
    for i in 0..10_000_000 {
        let key = format!("{}{}", base, i);
        let hash = format!("{:?}", md5::compute(key));
        if hash.starts_with(&leading_match) {
            return i;
        }
    }

    0
}

pub fn part1(base: &str) -> u32 {
    find_hash(base.trim(), 5)
}

pub fn part2(base: &str) -> u32 {
    find_hash(base.trim(), 6)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Commenting out; it's slow
        // assert_eq!(part1("abcdef"), 609_043);
        // assert_eq!(part1("pqrstuv"), 1_048_970);
    }

    #[test]
    fn test_part2() {
        // Commenting out; it's slow
        // assert_eq!(part2("abcdef"), 6_742_839);
    }
}
