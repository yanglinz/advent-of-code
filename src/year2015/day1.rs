pub fn part1(parens: &str) -> i32 {
    let mut res = 0;
    for c in parens.chars() {
        if c == '(' {
            res = res + 1;
        } else if c == ')' {
            res = res - 1;
        }
    }

    res
}

pub fn part2(parens: &str) -> u32 {
    let mut res = 0;
    for (i, c) in parens.chars().enumerate() {
        if c == '(' {
            res = res + 1;
        } else if c == ')' {
            res = res - 1;
        }

        if res == -1 {
            let index = i as u32;
            return index + 1;
        }
    }

    let index = parens.len() as u32;
    index + 1
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

    #[test]
    fn test_part2() {
        assert_eq!(part2("()())"), 5);
        assert_eq!(part2(")"), 1);
    }
}
