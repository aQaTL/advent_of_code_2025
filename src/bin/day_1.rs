fn main() {
    let input = std::fs::read_to_string("input/day_1.txt").unwrap();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2_brute_force(&input));
}

fn p1(input: &str) -> i64 {
    let mut zero_count = 0;
    let mut dial = 50;
    let rotations = parse_input(input);

    for rotation in rotations {
        dial = (dial + rotation) % 100;
        if dial < 0 {
            dial = 100 + dial;
        }

        if dial == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn p2_brute_force(input: &str) -> i64 {
    let mut zero_count = 0;
    let mut dial = 50;
    let rotations = parse_input(input);

    for rotation in rotations {
        let delta = if rotation.is_negative() { -1 } else { 1 };
        for _ in 0..(rotation.abs()) {
            dial += delta;
            if dial < 0 {
                dial = 99;
            } else if dial == 100 {
                dial = 0;
            }
            if dial == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            let rotation_direction = (line.as_bytes()[0] == b'L').then(|| -1).unwrap_or(1);
            line[1..].parse::<i64>().unwrap() * rotation_direction
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82\
    ";

    #[test]
    fn p1() {
        let expected = 3;
        assert_eq!(super::p1(INPUT), expected);
    }

    #[test]
    fn my_test_1() {
        const INPUT: &str = "L350";
        let expected = 1;
        assert_eq!(super::p1(INPUT), expected);
    }

    #[test]
    fn p2() {
        let expected = 6;
        assert_eq!(super::p2_brute_force(INPUT), expected);
    }

    #[test]
    fn my_test_2() {
        const INPUT: &str = "R1000";
        let expected = 10;
        assert_eq!(super::p2_brute_force(INPUT), expected);
    }

    #[test]
    fn my_test_3() {
        const INPUT: &str = "L1000";
        let expected = 10;
        assert_eq!(super::p2_brute_force(INPUT), expected);
    }
}
