use itertools::Itertools;

fn main() {
	let input = std::fs::read_to_string("input/day_2.txt").unwrap();
	println!("Part 1: {}", p1(&input));
	println!("Part 2: {}", p2(&input));
	println!("Part 2 (iterator): {}", p2_iterator(&input));
}

fn p1(input: &str) -> i64 {
	let mut sum = 0;
	for id_range in input.trim().split(',') {
		let (start, end) = id_range
			.split('-')
			.map(|id| id.parse::<i64>().unwrap())
			.collect_tuple()
			.unwrap();

		for n in start..=end {
			let s = n.to_string();
			if s.len() % 2 != 0 {
				continue;
			}

			let first_half = &s[..(s.len() / 2)];
			let second_half = &s[(s.len() / 2)..];
			if first_half == second_half {
				sum += n;
			}
		}
	}
	sum
}

fn p2(input: &str) -> i64 {
	let mut sum = 0;
	for id_range in input.trim().split(',') {
		let (start, end) = id_range
			.split('-')
			.map(|id| id.parse::<i64>().unwrap())
			.collect_tuple()
			.unwrap();

		for n in start..=end {
			let s = n.to_string();

			'l: for substr_len in 1..=(s.len() / 2) {
				if s.len() % substr_len != 0 {
					continue;
				}
				let first_chunk = &s[0..substr_len];

				for start_idx in (substr_len..s.len()).step_by(substr_len) {
					let current_chunk = &s[start_idx..(start_idx + substr_len)];
					if first_chunk != current_chunk {
						continue 'l;
					}
				}
				sum += n;
				break;
			}
		}
	}
	sum
}

fn p2_iterator(input: &str) -> i64 {
	input
		.trim()
		.split(',')
		.map(|id_range| {
			id_range
				.split('-')
				.map(|id| id.parse::<i64>().unwrap())
				.collect_tuple()
				.unwrap()
		})
		.flat_map(|(start, end)| {
			(start..=end).map(|n| (n, n.to_string())).filter(|(_, s)| {
				(1..=(s.len() / 2))
					.filter(|substr_len| s.len() % substr_len == 0)
					.any(|substr_len| {
						(substr_len..s.len())
							.step_by(substr_len)
							.map(|start_idx| &s[start_idx..(start_idx + substr_len)])
							.all(|chunk| chunk == &s[0..substr_len])
					})
			})
		})
		.map(|(n, _)| n)
		.sum()
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124\
    ";

	#[test]
	fn p1() {
		let input = INPUT.replace('\n', "");
		let expected = 1227775554;
		assert_eq!(super::p1(&input), expected);
	}

	#[test]
	fn p2() {
		let input = INPUT.replace('\n', "");
		let expected = 4174379265;
		assert_eq!(super::p2(&input), expected);
	}

	#[test]
	fn p2_signular_example_1() {
		let input = "565653-565659";
		let expected = 565656;
		assert_eq!(super::p2(&input), expected);
	}

	#[test]
	fn p2_signular_example_2() {
		let input = "2121212118-2121212124";
		let expected = 2121212121;
		assert_eq!(super::p2(&input), expected);
	}
}
