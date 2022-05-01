pub fn part1(dimensions: &str) -> u32 {
    fn get_area(l: u32, w: u32, h: u32) -> u32 {
        let mut smallest_side = [l, w, h];
        smallest_side.sort_unstable();
        let slack = smallest_side[0] * smallest_side[1];

        2 * l * w + 2 * w * h + 2 * h * l + slack
    }

    let mut res = 0;
    for d in dimensions.split_whitespace() {
        let dimensions: Vec<&str> = d.split('x').collect();
        let l = dimensions[0].parse().unwrap();
        let w = dimensions[1].parse().unwrap();
        let h = dimensions[2].parse().unwrap();
        res += get_area(l, w, h);
    }

    res
}

pub fn part2(dimensions: &str) -> u32 {
    fn get_ribbon_length(l: u32, w: u32, h: u32) -> u32 {
        let mut smallest_side = [l, w, h];
        smallest_side.sort_unstable();

        2 * (smallest_side[0] + smallest_side[1]) + l * w * h
    }

    let mut res = 0;
    for d in dimensions.split_whitespace() {
        let dimensions: Vec<&str> = d.split('x').collect();
        let l = dimensions[0].parse().unwrap();
        let w = dimensions[1].parse().unwrap();
        let h = dimensions[2].parse().unwrap();
        res += get_ribbon_length(l, w, h);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
        assert_eq!(part1("2x3x4\n1x1x10"), 58 + 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2x3x4"), 34);
        assert_eq!(part2("1x1x10"), 14);
        assert_eq!(part2("2x3x4\n1x1x10"), 34 + 14);
    }
}
