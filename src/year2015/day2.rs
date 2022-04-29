pub fn part1(dimensions: &str) -> u32 {
    fn get_area(l: u32, w: u32, h: u32) -> u32 {
        let slack = l * w;
        2 * l * w + 2 * w * h + 2 * h * l + slack
    }

    let mut res = 0;
    for d in dimensions.split_whitespace() {
        let dimensions: Vec<&str> = d.split("x").collect();
        let l = dimensions[0].parse().unwrap();
        let w = dimensions[1].parse().unwrap();
        let h = dimensions[2].parse().unwrap();
        res += get_area(l, w, h);
    }

    res
}

pub fn part2(parens: &str) -> u32 {
    1234
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(")"), 1);
    }
}
