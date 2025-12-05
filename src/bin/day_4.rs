fn main() {
    let input = std::fs::read_to_string("input/day_4.txt").unwrap();
    println!("Part 1: {}", p1(&input));
    println!("Part 2: {}", p2(&input));
}

fn p1(input: &str) -> i64 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect();

    let mut sum = 0;
    for j in 0..grid.len() {
        for i in 0..(grid[0].len()) {
            if grid[j][i] != b'@' {
                continue;
            }
            if adjacent(&grid, j, i).filter(|b| *b == b'@').count() < 4 {
                sum += 1;
            }
        }
    }

    sum
}

fn p2(input: &str) -> i64 {
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect();

    let mut sum = 0;

    loop {
        let mut grid_copy = grid.clone();

        let mut removed = 0;
        for j in 0..grid.len() {
            for i in 0..(grid[0].len()) {
                if grid[j][i] != b'@' {
                    continue;
                }
                if adjacent(&grid, j, i).filter(|b| *b == b'@').count() < 4 {
                    grid_copy[j][i] = b'.';
                    removed += 1;
                }
            }
        }
        if removed == 0 {
            break;
        }
        sum += removed;
        grid = grid_copy;
    }

    sum
}

fn adjacent(grid: &Vec<Vec<u8>>, j: usize, i: usize) -> impl Iterator<Item = u8> {
    let width = grid[0].len();
    let height = grid.len();
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .into_iter()
    .filter_map(move |(delta_i, delta_j)| {
        let new_j = j.checked_add_signed(delta_j).filter(|j| *j < height);
        let new_i = i.checked_add_signed(delta_i).filter(|i| *i < width);
        new_i.zip(new_j)
    })
    .map(|(i, j)| grid[j][i])
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.\
    ";

    #[test]
    fn p1() {
        let expected = 13;
        assert_eq!(super::p1(INPUT), expected);
    }

    #[test]
    fn p2() {
        let expected = 43;
        assert_eq!(super::p2(INPUT), expected);
    }
}
