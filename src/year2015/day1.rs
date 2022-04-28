pub fn part1(parents: &str) -> i32 {
    let mut res = 0;
    for c in parents.chars() {
        if c == '(' {
            res = res + 1;
        } else if c == ')' {
            res = res - 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }
}
