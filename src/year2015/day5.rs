use fancy_regex::Regex;

fn has_three_vowels(s: &str) -> bool {
    let mut num_vowels = 0;
    let vowels = "aeiou";
    for c in s.chars() {
        if vowels.contains(c) {
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
    let mut count = 0;
    for s in strings.trim().split('\n') {
        if is_nice(s) {
            count += 1
        }
    }

    count
}

fn has_pairs(s: &str) -> bool {
    let re = Regex::new(r"(\w\w)\w*\1").unwrap();
    let m = re.is_match(s);
    if let Ok(is_match) = m {
        is_match
    } else {
        false
    }
}

fn has_sandwich(s: &str) -> bool {
    let re = Regex::new(r"(\w)\w\1").unwrap();
    let m = re.is_match(s);
    if let Ok(is_match) = m {
        is_match
    } else {
        false
    }
}

fn is_nice_redux(s: &str) -> bool {
    has_pairs(s) && has_sandwich(s)
}

pub fn part2(strings: &str) -> u32 {
    let mut count = 0;
    for s in strings.trim().split('\n') {
        if is_nice_redux(s) {
            count += 1
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(has_three_vowels("aio"), true);
        assert_eq!(has_three_vowels("aaa"), true);
        assert_eq!(has_three_vowels("abc"), false);
        assert_eq!(has_three_vowels("ai"), false);

        assert_eq!(has_repeats("abcdefg"), false);
        assert_eq!(has_repeats("aabcdefg"), true);
        assert_eq!(has_repeats("abcccccdefg"), true);
        assert_eq!(has_repeats("aa"), true);
        assert_eq!(has_repeats("a"), false);
        assert_eq!(has_repeats(""), false);

        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("ugknbfddgicrmopnab"), false);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(has_pairs("xyxy"), true);
        assert_eq!(has_pairs("xyx"), false);
        assert_eq!(has_pairs("aabcdefgaa"), true);
        assert_eq!(has_pairs("abcdefga"), false);

        assert_eq!(has_sandwich("xyx"), true);
        assert_eq!(has_sandwich("abcdefeghi"), true);
        assert_eq!(has_sandwich("efe"), true);
        assert_eq!(has_sandwich("abc"), false);

        assert_eq!(is_nice_redux("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_redux("xxyxx"), true);
        assert_eq!(is_nice_redux("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_redux("ieodomkazucvgmuy"), false);
    }
}
