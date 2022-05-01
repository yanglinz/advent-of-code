fn has_three_vowels(s: &str) -> bool {
    let mut num_vowels = 0;
    for c in "aeiou".chars() {
        if s.contains(c) {
            num_vowels += 1;
        }

        if num_vowels >= 3 {
            return true;
        }
    }

    false
}

fn has_repeats(s: &str) -> bool {
    let s_bytes = s.as_bytes();
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            continue;
        }

        let prev = s_bytes[i - 1];
        if c == prev as char {
            return true;
        }
    }

    false
}

fn is_nice(s: &str) -> bool {
    if !(has_three_vowels(s)) {
        return false;
    }

    if !has_repeats(s) {
        return false;
    }

    let patterns = vec!["ab", "cd", "pq", "xy"];
    for p in patterns {
        if s.contains(p) {
            return false;
        }
    }

    true
}

pub fn part1(strings: &str) -> u32 {
    1
}

pub fn part2(strings: &str) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helpers() {
        assert_eq!(has_three_vowels("aio"), true);
        assert_eq!(has_three_vowels("abc"), false);
        assert_eq!(has_three_vowels("ai"), false);

        assert_eq!(has_repeats("abcdefg"), false);
        assert_eq!(has_repeats("aabcdefg"), true);
        assert_eq!(has_repeats("abcccccdefg"), true);
        assert_eq!(has_repeats("aa"), true);
        assert_eq!(has_repeats("a"), false);
        assert_eq!(has_repeats(""), false);

        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), false);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

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
