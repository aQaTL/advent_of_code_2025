use itertools::Itertools;
use nom::{
    IResult, Parser,
    branch::alt,
    character::{
        complete::{space0, space1},
        one_of,
    },
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
};

fn main() {
    let input = std::fs::read_to_string("input/day_6.txt").unwrap();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

fn p1(input: &str) -> i64 {
    let mut lines = Vec::new();
    for l in input.lines() {
        let (_, line) = parse_line(l).unwrap();
        match line {
            LineType::Numbers(nums) => {
                for (n_idx, n) in nums.into_iter().enumerate() {
                    let ll = match lines.get_mut(n_idx) {
                        Some(v) => v,
                        None => {
                            lines.push(Line::default());
                            &mut lines[n_idx]
                        }
                    };
                    ll.numbers.push(n);
                }
            }
            LineType::Ops(ops) => {
                for (o_idx, o) in ops.into_iter().enumerate() {
                    lines[o_idx].op = o;
                }
            }
        }
    }
    let mut sum = 0_i64;
    for l in lines {
        match l.op {
            b'+' => sum += l.numbers.into_iter().sum::<i64>(),
            b'*' => sum += l.numbers.into_iter().product::<i64>(),
            _ => panic!(),
        }
    }

    sum
}

fn parse_line(l: &str) -> IResult<&str, LineType> {
    alt((
        map(
            preceded(
                space0,
                separated_list1(space1, map(nom::character::complete::u64, |n| n as i64)),
            ),
            LineType::Numbers,
        ),
        map(
            preceded(
                space0,
                separated_list1(space1, map(one_of("+*"), |n| n as u8)),
            ),
            LineType::Ops,
        ),
    ))
    .parse_complete(l)
}

fn p2(input: &str) -> i64 {
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let mut grid: Vec<Line2> = Vec::new();
    for j in 0..lines.len() {
        for i in 0..lines[j].len() {
            let l = match grid.get_mut(i) {
                Some(v) => v,
                None => {
                    grid.push(Default::default());
                    &mut grid[i]
                }
            };
            let c = lines[j][i];
            if matches!(c, '+' | '*') {
                l.op = c;
            } else {
                l.numbers.push(c);
            }
        }
    }

    let mut sum = 0;
    let mut current_res = 0;
    let mut current_op = ' ';
    for l in grid {
        match l.op {
            '+' => {
                current_op = l.op;
            }
            '*' => {
                current_op = l.op;
                if current_res == 0 {
                    current_res = 1;
                }
            }
            _ => (),
        }
        let Ok(n) = l
            .numbers
            .into_iter()
            .filter(|x| !x.is_ascii_whitespace())
            .join("")
            .parse::<i64>()
        else {
            sum += current_res;
            current_res = 0;
            continue;
        };
        match current_op {
            '+' => {
                current_res += n;
            }
            '*' => {
                current_res *= n;
            }
            _ => panic!(),
        }
    }

    sum + current_res
}

enum LineType {
    Numbers(Vec<i64>),
    Ops(Vec<u8>),
}

#[derive(Debug, Default)]
struct Line {
    numbers: Vec<i64>,
    op: u8,
}

#[derive(Debug, Default)]
struct Line2 {
    numbers: Vec<char>,
    op: char,
}

#[cfg(test)]
mod tests {
    #[test]
    fn ex1() {
        let input = "\
123 328  51 64
    45 64  387 23
    6 98  215 314
*   +   *   +  \
";
        let expected = 4277556;
        assert_eq!(super::p1(input), expected);
    }

    #[test]
    fn ex2() {
        let input = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  \
";
        let expected = 3263827;
        assert_eq!(super::p2(input), expected);
    }
}
