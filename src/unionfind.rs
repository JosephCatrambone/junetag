use std::collections::HashMap;
use image::{GenericImageView, Luma};

/// Given the binarized image, the next step is to find edges
/// which might form the boundary of a tag. A straightforward
/// approach is to identify edge pixels which have an opposite
/// colored neighbor, then form connected groups of edge pixels.
/// However, this approach breaks down when the white space
/// between tag boundaries approaches only a single pixel wide,
/// which may happen for physically small or faraway tags. If
/// two tag boundaries are incorrectly merged, the tags will not
/// be detected. Our proposed solution is to segment the edges
/// based on the identities of the black and white components
/// from which they arise.
/// Connected components of light and dark pixels are segmented using the union-find algorithm [15],
/// which gives each component a unique ID. For every pair of
/// adjacent black and white components, we identify the pixels
/// on the boundaries of those two regions as a distinct cluster.
/// This clustering can be done efficiently by using a hash table,
/// indexing each cluster by the black and white components’
/// IDs, as described in Figure 4. In the aforementioned case of
/// a single pixel-wide white component separating two distinct
/// black components, we have solved the problem by allowing
/// the same white pixels to appear in both resulting clusters.
fn boundary_segmentation(threshold_image: &dyn GenericImageView<Pixel=Luma<u8>>) -> Vec<usize> {
	let num_pixels = (threshold_image.width() * threshold_image.height()) as usize;
	let mut boundary_point_group_assignment = Vec::with_capacity(num_pixels);
	// Fill boundary_point_group_assignment with '0'.
	for idx in 0..num_pixels {
		boundary_point_group_assignment.push(idx);
	}

	for y in 0..threshold_image.height() {
		for x in 0..threshold_image.width() {
			let idx = (x+y*threshold_image.width()) as usize;
			// We can't just check up and left because there may be a spiral.
			// Do a 4-way connectivity check and union all of the sets (if they match).
			let local_pixel = threshold_image.get_pixel(x, y)[0];
			for (x2, y2) in [
				(x.saturating_add(1), y), // Right
				(x, y.saturating_sub(1)), // Up
				(x.saturating_sub(1), y), // Left
				(x, y.saturating_add(1)), // Down
			] {
				if x == x2 && y == y2 { continue; } // Skip saturated underflow. (0u8 - 1 -> 0).
				if x2 >= threshold_image.width() || y2 >= threshold_image.height() { continue; }

				let neighbor_pixel_idx = (x2 + y2*threshold_image.width()) as usize;
				if threshold_image.get_pixel(x2, y2)[0] == local_pixel {
					union(idx, neighbor_pixel_idx, &mut boundary_point_group_assignment);
				}
			}
		}
	}

	boundary_point_group_assignment
}

/// Given a vector of assignments [0, 50, 42, 11, 10, 10, 2, 2, 2, ...]
/// make the smallest distinct assignment, [0, 1, 2, 3, 4, 4, 5, 5, 5, ...]
/// returns both the new assignment and the largest value in the new assignment.
/// Assumes that boundary_segmentation has been run, but won't freak out for an arbitrary list.
fn compact_boundary_segmentation(assignment: &Vec<usize>) -> (Vec<usize>, usize) {
	// TODO: HashMap is using a cryptographically robust one and we don't need that.
	let mut mapping = HashMap::with_capacity(assignment.len());  // This is way oversized.
	let mut remapped: Vec<usize> = Vec::with_capacity(assignment.len());
	let mut next_index = 0usize;
	for value in assignment {
		if let Some(new_assignment) = mapping.get(value) {
			remapped.push(*new_assignment);
		} else {
			mapping.insert(value, next_index);
			remapped.push(next_index);
			next_index += 1;
		}
	}

	return (remapped, next_index);
}

fn find_root(member: usize, roots: &mut Vec<usize>) -> usize {
	// Traverse up the roots twice, once to obtain the ultimate parent, again to set to the ultimate parent.
	// Find the greatest grandparent.
	let mut current_member = member;
	while roots[current_member] != current_member {
		current_member = roots[current_member];
	}
	let ultimate_parent = current_member;
	// Go back over the path and set them all.
	current_member = member;
	while roots[current_member] != current_member {
		let next_parent = roots[current_member];
		roots[current_member] = ultimate_parent;
		current_member = next_parent;
	}
	ultimate_parent
}

fn union(member_a: usize, member_b: usize, roots: &mut Vec<usize>) {
	let ultimate_root_a = find_root(member_a, roots);
	let ultimate_root_b = find_root(member_b, roots);
	if ultimate_root_a == ultimate_root_b {
		return;  //  Nothin to do.
	}

	// Bounds check? -- this is not part of the original algorithm and should never be necessary,
	// but if it happens to be the case that we're unioning two new undiscovered points...
	// If A happens to be outside the length of the roots, zero-fill it until it's inside.
	// while member_a > roots.len() || member_b > roots.len() { roots.push(NO_NEIGHBOR); }

	// Make the ultimate root of b a child of ultimte root a.
	if ultimate_root_a < ultimate_root_b {
		roots[ultimate_root_b as usize] = ultimate_root_a;
	} else {
		roots[ultimate_root_a as usize] = ultimate_root_b;
	}
}


#[cfg(test)]
mod tests {
	use image::DynamicImage::ImageLuma8;
	use image::GrayImage;
	use super::*;

	#[test]
	fn test_neighbor_segmentation() {
		let img = GrayImage::from_raw(5, 5, vec![
			0, 0, 0, 0, 0,
			0, 1, 1, 1, 0,
			0, 1, 0, 1, 1,
			0, 1, 1, 1, 0,
			0, 0, 1, 1, 0,
		]).unwrap();

		let expected_roots = vec![
			0, 0, 0, 0, 0,
			0, 1, 1, 1, 0,
			0, 1, 2, 1, 1,
			0, 1, 1, 1, 3,
			0, 0, 1, 1, 3,
		];

		let assigned_roots = boundary_segmentation(&img);
		let (compacted_roots, _) = compact_boundary_segmentation(&assigned_roots);
		assert_eq!(expected_roots, compacted_roots);
	}

	#[test]
	fn test_find_root() {
		// Tree:
		// 0 (root), 1 (root), 4 (root),
		// 1 -> 2, 3, 5
		// 4 -> 6
		// 6 -> 7
		let mut roots = vec![0, 1, 1, 1, 4, 1, 4, 6];
		let expected_roots = vec![0, 1, 1, 1, 4, 1, 4, 4];
		let mut calculated_roots = vec![];
		for idx in 0..=7usize {
			calculated_roots.push(find_root(idx, &mut roots));
		}
		assert_eq!(expected_roots, calculated_roots);
	}

	#[test]
	fn test_union() {
		let mut roots = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];  // Everything is its own node.

		union(1, 2, &mut roots);
		assert_eq!(find_root(2, &mut roots), 1); // Roots: _, 1, 1, 3, 4, 5, 6, 7, 8, 9, 10
		assert_eq!(find_root(3, &mut roots), 3);

		union(3, 4, &mut roots);
		assert_eq!(find_root(2, &mut roots), 1); // Roots: _, 1, 1, 3, 3, 5, 6, 7, 8, 9, 10
		assert_eq!(find_root(4, &mut roots), 3);

		union(2, 3, &mut roots);
		assert_eq!(find_root(1, &mut roots), 1); // Roots: _, 1, 1, 1, 1, 5, 6, 7, 8, 9, 10
		assert_eq!(find_root(2, &mut roots), 1);
		assert_eq!(find_root(3, &mut roots), 1);
		assert_eq!(find_root(4, &mut roots), 1);
		assert_eq!(find_root(5, &mut roots), 5);
	}
}

