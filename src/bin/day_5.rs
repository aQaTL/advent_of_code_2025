use std::ops::RangeInclusive;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day_5.txt").unwrap();
    // println!("Part 1: {}", p1(&input));
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
    let ranges: Vec<RangeInclusive<i64>> = ranges
        .lines()
        .map(|l| {
            l.split('-')
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(start, end)| start..=end)
        .collect();

    println!("ranges to go through: {}", ranges.len());
    let mut consolidated_ranges = Vec::<RangeInclusive<i64>>::new();
    consolidated_ranges.push(ranges[0].clone());
    for (idx, mut range) in ranges.into_iter().skip(1).enumerate() {
        println!("{idx}. To check: {}", consolidated_ranges.len());
        for con_range in &mut consolidated_ranges {
            // check if start is within
            // let contains_start = con_range.contains(range.start());
            // let contains_end = con_range.contains(range.end());
            let contains_start = contains(&con_range, *range.start());
            let contains_end = contains(&con_range, *range.end());

            println!("{contains_start} {contains_end}");
            // println!(
            //     "std: {contains_start}..={contains_end} my: {contains_start_x}..={contains_end_x}"
            // );

            // if contains_start != contains_start_x {
            //     println!("bad");
            // }
            // if contains_end != contains_end_x {
            //     println!("bad");
            // }

            match (contains_start, contains_end) {
                (true, true) => break,
                (true, false) => {
                    *con_range = *con_range.start()..=*range.end();
                    break;
                }
                (false, true) => {
                    *con_range = *range.start()..=*con_range.end();
                    break;
                }
                (false, false) => {
                    consolidated_ranges.push(range);
                    break;
                }
            }
        }
    }

    consolidated_ranges
        .into_iter()
        .map(|range| *range.end() - *range.start())
        .sum()
}

fn contains(range: &RangeInclusive<i64>, n: i64) -> bool {
    n <= *range.end() && n >= *range.start()
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
}
