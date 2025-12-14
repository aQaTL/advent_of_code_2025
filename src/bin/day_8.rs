use std::{collections::HashSet, i64};

use itertools::Itertools;

fn main() {
	let input = std::fs::read_to_string("input/day_8.txt").unwrap();
	println!("Part 1: {}", p1(&input));
	println!("Part 2: {}", p2(&input));
}

fn p1(input: &str) -> i64 {
	p1_sort_approach(input)
}

#[allow(dead_code)]
fn p1_orig(input: &str) -> i64 {
	let boxes: Vec<(i64, i64, i64)> = input
		.lines()
		.map(|l| {
			l.split(",")
				.map(|n| n.parse().unwrap())
				.collect_tuple()
				.unwrap()
		})
		.collect();

	let mut found_idx_pairs = HashSet::new();
	let mut circuits = Vec::<Vec<usize>>::new();

	let mut closest_point_a = (0, 0, 0);
	let mut closest_point_b = (0, 0, 0);
	for iter_idx in 0..1000 {
		let mut closest_point_a_idx = 0;
		let mut closest_point_b_idx = 0;
		let mut closest_dist = i64::MAX;

		for idx in 0..(boxes.len() - 1) {
			let curr_box = boxes[idx];
			let closest_idx = (idx..boxes.len())
				.filter(|idx_b| idx != *idx_b && !found_idx_pairs.contains(&(idx, *idx_b)))
				.min_by_key(|idx_b| euclid_distance(curr_box, boxes[*idx_b]))
				.unwrap();
			let closest = boxes[closest_idx];
			let dist = euclid_distance(curr_box, closest);
			if dist < closest_dist {
				closest_dist = dist;
				closest_point_a = curr_box;
				closest_point_b = closest;
				closest_point_a_idx = idx;
				closest_point_b_idx = closest_idx;
			}
		}
		// found_idx_pairs.insert((
		// 	closest_point_a_idx.min(closest_point_b_idx),
		// 	closest_point_a_idx.max(closest_point_a_idx),
		// ));
		found_idx_pairs.insert((closest_point_a_idx, closest_point_b_idx));
		let mut found_a = false;
		let mut a_c_idx = 0;
		let mut found_b = false;
		let mut b_c_idx = 0;
		for (idx, c) in circuits.iter().enumerate() {
			for c_idx in 0..c.len() {
				if c[c_idx] == closest_point_a_idx {
					found_a = true;
					a_c_idx = idx;
				} else if c[c_idx] == closest_point_b_idx {
					found_b = true;
					b_c_idx = idx;
				}
			}
		}

		match (found_a, found_b) {
			(true, true) if a_c_idx == b_c_idx => (),
			(true, true) => {
				let b_c = std::mem::take(&mut circuits[b_c_idx]);
				circuits[a_c_idx].extend(b_c);
			}
			(false, true) => {
				circuits[b_c_idx].push(closest_point_a_idx);
			}
			(true, false) => {
				circuits[a_c_idx].push(closest_point_b_idx);
			}
			(false, false) => {
				circuits.push(vec![closest_point_a_idx, closest_point_b_idx]);
			}
		}

		println!(
			"{iter_idx}: closest point ({closest_dist}): {closest_point_a:?} = {closest_point_b:?} | {}",
			circuits.len()
		);
	}
	println!("{circuits:#?}");

	let result = circuits
		.into_iter()
		.map(|c| c.len() as i64)
		.sorted()
		.rev()
		.take(3)
		.product();

	result
}

fn p1_sort_approach(input: &str) -> i64 {
	let boxes: Vec<(i64, i64, i64)> = input
		.lines()
		.map(|l| {
			l.split(",")
				.map(|n| n.parse().unwrap())
				.collect_tuple()
				.unwrap()
		})
		.collect();

	let edge_idxs: Vec<(usize, usize)> = (0..boxes.len())
		.map(|idx| ((idx + 1)..boxes.len()).map(move |idx_2| (idx, idx_2)))
		.flatten()
		.sorted_unstable_by_key(|(idx_a, idx_b)| euclid_distance(boxes[*idx_a], boxes[*idx_b]))
		.collect_vec();

	let mut uf = UnionFind::new(boxes.len());
	for (edge_idx_i, edge_idx_j) in &edge_idxs[..1000] {
		uf.unite(*edge_idx_i, *edge_idx_j);
	}

	uf.size
		.into_iter()
		.sorted_unstable()
		.rev()
		.take(3)
		.product::<usize>() as i64
}

struct UnionFind {
	// usize is going to be the index of `boxes`
	parent: Vec<usize>,
	size: Vec<usize>,
}

impl UnionFind {
	fn new(size: usize) -> Self {
		let mut uf = UnionFind {
			parent: Vec::with_capacity(size),
			size: vec![1; size],
		};
		for i in 0..size {
			uf.parent.push(i);
		}
		uf
	}

	fn find(&mut self, i: usize) -> usize {
		let root = self.parent[i];

		if self.parent[root] != root {
			self.parent[i] = self.find(root);
			return self.parent[i];
		}

		root
	}

	fn unite(&mut self, i: usize, j: usize) -> bool {
		let irep = self.find(i);
		let jrep = self.find(j);

		if irep == jrep {
			return false;
		}

		let i_size = self.size[irep];
		let j_size = self.size[jrep];

		if i_size < j_size {
			self.parent[irep] = jrep;
			self.size[jrep] += self.size[irep];
		} else {
			self.parent[jrep] = irep;
			self.size[irep] += self.size[jrep];
		}
		true
	}
}

fn p2(input: &str) -> i64 {
	let boxes: Vec<(i64, i64, i64)> = input
		.lines()
		.map(|l| {
			l.split(",")
				.map(|n| n.parse().unwrap())
				.collect_tuple()
				.unwrap()
		})
		.collect();

	let edge_idxs: Vec<(usize, usize)> = (0..boxes.len())
		.map(|idx| ((idx + 1)..boxes.len()).map(move |idx_2| (idx, idx_2)))
		.flatten()
		.sorted_unstable_by_key(|(idx_a, idx_b)| euclid_distance(boxes[*idx_a], boxes[*idx_b]))
		.collect_vec();

	let mut uf = UnionFind::new(boxes.len());
	let mut circuits = boxes.len();
	for (edge_idx_i, edge_idx_j) in edge_idxs.iter() {
		if uf.unite(*edge_idx_i, *edge_idx_j) {
			circuits -= 1;
		}
		if circuits == 1 {
			return boxes[*edge_idx_i].0 * boxes[*edge_idx_j].0;
		}
	}
	panic!()
}

fn euclid_distance(p: (i64, i64, i64), q: (i64, i64, i64)) -> i64 {
	((p.0 - q.0).pow(2) + (p.1 - q.1).pow(2) + (p.2 - q.2).pow(2)).isqrt()
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689\
    ";

	#[test]
	fn p2() {
		let expected = 25272;
		assert_eq!(super::p2(INPUT), expected);
	}

	#[test]
	fn union_find() {
		let size = 7;
		let mut uf = super::UnionFind::new(size);

		uf.unite(1, 2);
		uf.unite(3, 4);
		uf.unite(2, 4);
		uf.unite(5, 6);
		uf.unite(5, 1);

		println!("sizes: {:?}", uf.size);

		let (c1, c2) = (6, 3);
		let r = uf.find(c1) == uf.find(c2);
		println!("Are {c1} and {c2} in the same set? {r}");
		assert!(r);
	}
}
