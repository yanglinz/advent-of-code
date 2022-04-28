pub fn part1(input: &str) -> u32 {
    let mut res = 0;
    for c in input.chars() {
        if c == '(' {
            res = res + 1;
        } else if c == ')' {
            res = res - 1;
        }
    }

    res
}
