use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn get_next_position(current: &Position, movement: &char) -> Position {
    Position { x: 1, y: 2 };

    match movement {
        '^' => Position {
            x: current.x,
            y: current.y + 1,
        },
        'v' => Position {
            x: current.x,
            y: current.y - 1,
        },
        '>' => Position {
            x: current.x + 1,
            y: current.y,
        },
        '<' => Position {
            x: current.x - 1,
            y: current.y,
        },
        _ => {
            panic!("Unexpected char: {}", _);
        }
    }
}

pub fn part1(directions: &str) -> u32 {
    let mut tracker: HashMap<Position, u32> = HashMap::new();
    let mut current = Position { x: 0, y: 0 };
    tracker.insert(Position { x: 0, y: 0 }, 1);

    for c in directions.trim().chars() {
        tracker.insert(get_next_position(&current, &c), 1);
        current = get_next_position(&current, &c);
    }

    tracker.keys().len() as u32
}

pub fn part2(directions: &str) -> u32 {
    let mut tracker: HashMap<Position, u32> = HashMap::new();
    let mut current = Position { x: 0, y: 0 };
    tracker.insert(Position { x: 0, y: 0 }, 1);

    for c in directions.trim().chars() {
        tracker.insert(get_next_position(&current, &c), 1);
        current = get_next_position(&current, &c);
    }

    tracker.keys().len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(">"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
