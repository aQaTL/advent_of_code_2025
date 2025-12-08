use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day_7.txt").unwrap();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

fn p1(input: &str) -> i64 {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let start = (0_usize, grid[0].iter().position(|c| *c == b'S').unwrap());

    let mut streams = VecDeque::<(usize, usize)>::new();
    streams.push_back(start);
    let mut visited = HashSet::new();
    let mut splits = 0;

    while let Some((mut stream_j, stream_i)) = streams.pop_front() {
        stream_j += 1;
        visited.insert((stream_j, stream_i));
        match grid.get(stream_j).and_then(|row| row.get(stream_i)) {
            Some(b'.') => {
                streams.push_back((stream_j, stream_i));
                continue;
            }
            Some(b'^') => {
                let new_stream_1 = (stream_j, stream_i - 1);
                let new_stream_2 = (stream_j, stream_i + 1);
                let visited_new_stream_1 = visited.contains(&new_stream_1);
                let visited_new_stream_2 = visited.contains(&new_stream_2);
                if !visited_new_stream_1 {
                    streams.push_back(new_stream_1);
                    visited.insert(new_stream_1);
                }
                if !visited_new_stream_2 {
                    streams.push_back(new_stream_2);
                    visited.insert(new_stream_2);
                }
                if !visited_new_stream_1 || !visited_new_stream_2 {
                    splits += 1;
                }
            }
            None => {
                continue;
            }

            _ => panic!(),
        }
    }

    splits
}

fn p2(input: &str) -> i64 {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect_vec();
    let start = (0_usize, grid[0].iter().position(|c| *c == b'S').unwrap(), 1);

    let mut streams = VecDeque::<(usize, usize, i64)>::new();
    streams.push_back(start);
    let mut realities = 0;

    while let Some((mut stream_j, stream_i, particle_count)) = streams.pop_front() {
        stream_j += 1;

        match grid.get(stream_j).and_then(|row| row.get(stream_i)) {
            Some(b'.') => {
                streams.push_back((stream_j, stream_i, particle_count));
                continue;
            }
            Some(b'^') => {
                let new_stream_1_i = stream_i - 1;
                let new_stream_2_i = stream_i + 1;

                if let Some((_, _, pc)) = streams
                    .iter_mut()
                    .find(|(j, i, _)| (*j, *i) == (stream_j, new_stream_1_i))
                {
                    *pc += particle_count;
                } else {
                    streams.push_back((stream_j, new_stream_1_i, particle_count));
                }

                if let Some((_, _, pc)) = streams
                    .iter_mut()
                    .find(|(j, i, _)| (*j, *i) == (stream_j, new_stream_2_i))
                {
                    *pc += particle_count;
                } else {
                    streams.push_back((stream_j, new_stream_2_i, particle_count));
                }
            }
            None => {
                realities += particle_count;
                continue;
            }
            _ => panic!(),
        }
    }

    realities
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............\
    ";

    #[test]
    fn p1() {
        let expected = 21;
        assert_eq!(super::p1(INPUT), expected);
    }

    #[test]
    fn p2() {
        let expected = 40;
        assert_eq!(super::p2(INPUT), expected);
    }
}
