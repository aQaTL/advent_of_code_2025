fn main() {
	let input = std::fs::read_to_string("input/day_3.txt").unwrap();
	println!("Part 1: {}", p1(&input));
	println!("Part 2: {}", p2(&input));
}

fn p1(input: &str) -> i64 {
	let banks = input.lines().map(|l| {
		l.bytes()
			.map(|c| (c - '0' as u8) as i64)
			.collect::<Vec<_>>()
	});
	let mut sum = 0;
	for bank in banks {
		let mut largest_idx = 0;
		for idx in 0..(bank.len() - 1) {
			if bank[idx] > bank[largest_idx] {
				largest_idx = idx;
			}
		}
		let largest = bank[largest_idx];
		let second_largest = bank[(largest_idx + 1)..].iter().max().unwrap();
		sum += largest * 10 + second_largest;
	}
	sum
}

fn p2(input: &str) -> i64 {
	let banks = input.lines().map(|l| {
		l.bytes()
			.map(|c| (c - '0' as u8) as i64)
			.collect::<Vec<_>>()
	});
	let mut sum = 0;
	for bank in banks {
		let mut largest_idxs = [0; 12];

		for battery_idx in 0..12_usize {
			let start_idx = if battery_idx == 0 {
				0
			} else {
				largest_idxs[battery_idx - 1] + 1
			};
			let end_idx = bank.len() - (12 - battery_idx - 1);
			largest_idxs[battery_idx] = start_idx;
			for searched_for_battery_idx in start_idx..end_idx {
				if bank[searched_for_battery_idx] > bank[largest_idxs[battery_idx]] {
					largest_idxs[battery_idx] = searched_for_battery_idx;
				}
			}
		}
		sum += (0..12)
			.map(|e| bank[largest_idxs[e]] * 10_i64.pow(11 - e as u32))
			.sum::<i64>();
	}

	sum
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111\
        ";

	#[test]
	fn p1() {
		let expected = 357;
		assert_eq!(super::p1(INPUT), expected);
	}

	#[test]
	fn my_test_1() {
		let input = "2128289";
		let expected = 89;
		assert_eq!(super::p1(input), expected);
	}

	#[test]
	fn my_test_2() {
		let input = "212827857";
		let expected = 88;
		assert_eq!(super::p1(input), expected);
	}

	#[test]
	fn my_test_3() {
		let input = "212782387";
		let expected = 88;
		assert_eq!(super::p1(input), expected);
	}

	#[test]
	fn p2() {
		let expected = 3121910778619;
		assert_eq!(super::p2(INPUT), expected);
	}
}
