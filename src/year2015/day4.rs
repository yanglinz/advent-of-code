pub fn part1(secret: &str) -> u32 {
    for i in 0..10_000_000 {
        let key = format!("{}{}", secret, i);
        let hash = format!("{:?}", md5::compute(key));
        if hash.starts_with("00000") {
            return i
        }
    } 

    0
}

pub fn part2(secret: &str) -> u32 {
    123
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdef"), 609_043);
        assert_eq!(part1("pqrstuv"), 1_048_970);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2("^v"), 3);
        // assert_eq!(part2("^>v<"), 3);
        // assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
