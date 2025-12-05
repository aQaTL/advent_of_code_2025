use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day_5.txt").unwrap();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

fn p1(input: &str) -> usize {
    let (ranges, ids) = input.split("\n\n").collect_tuple().unwrap();
    let ranges: Vec<(i64, i64)> = ranges
        .lines()
        .map(|l| {
            l.split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let ids: Vec<i64> = ids.lines().map(|n| n.parse().unwrap()).collect();

    ids.into_iter()
        .filter(|id| {
            ranges
                .iter()
                .any(|(start, end)| (start..=end).contains(&id))
        })
        .count()
}

fn p2(input: &str) -> i64 {
    let (ranges, _) = input.split("\n\n").collect_tuple().unwrap();
    let mut ranges: Vec<(i64, i64)> = ranges
        .lines()
        .map(|l| {
            l.split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    loop {
        let mut consolidated_ranges = Vec::<(i64, i64)>::new();
        consolidated_ranges.push(ranges[0].clone());

        let mut changed_anything = false;
        'l: for range in ranges.into_iter().skip(1) {
            for con_range in consolidated_ranges.iter_mut() {
                if overlaps(range, *con_range) {
                    changed_anything = true;
                    *con_range = (range.0.min(con_range.0), con_range.1.max(range.1));
                    continue 'l;
                }
            }
            consolidated_ranges.push(range);
        }
        ranges = consolidated_ranges;

        if !changed_anything {
            break;
        }
    }

    ranges.into_iter().map(|(start, end)| end - start + 1).sum()
}

fn overlaps(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 <= b.1 && a.0 >= b.0 || b.0 <= a.1 && b.0 >= a.0
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32\
    ";

    #[test]
    fn p1() {
        let expected = 3;
        assert_eq!(super::p1(INPUT), expected);
    }

    #[test]
    fn p2() {
        let expected = 14;
        assert_eq!(super::p2(INPUT), expected);
    }

    #[test]
    fn p2_test_1() {
        const INPUT: &str = "\
3-5
10-14
16-20
12-18
13-14
13-13

1
5
8
11
17
32\
        ";
        let expected = 14;
        assert_eq!(super::p2(INPUT), expected);
    }

    #[test]
    fn p2_test_2() {
        const INPUT: &str = "\
1-10
15-20
17-25

1
2\
        ";
        let expected = 21;
        assert_eq!(super::p2(INPUT), expected);
    }

    #[test]
    fn p2_test_3() {
        const INPUT: &str = "\
1-10
1-10

1
2\
        ";
        let expected = 10;
        assert_eq!(super::p2(INPUT), expected);
    }
    #[test]
    fn p2_test_4() {
        const INPUT: &str = "\
1-10
2-5

1
2\
        ";
        let expected = 10;
        assert_eq!(super::p2(INPUT), expected);
    }

    #[test]
    fn p2_test_5() {
        const INPUT: &str = "\
200-300
100-101
1-1
2-2
3-3
1-3
1-3
2-2
50-70
10-10
98-99
99-99
99-99
99-100
1-1
100-100
100-100
100-101
200-300
201-300
202-300
250-251
98-99
100-100
100-101
1-101

2\
        ";
        let expected = 202;
        assert_eq!(super::p2(INPUT), expected);
    }
}
