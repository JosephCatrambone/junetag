use image::{GenericImageView, Luma};

const NO_NEIGHBOR: u32 = 0;

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
fn boundary_segmentation(threshold_image: &dyn GenericImageView<Pixel=Luma<u8>>) -> Vec<u32> {
	let num_pixels = (threshold_image.width() * threshold_image.height()) as usize;
	let mut boundary_point_group_assignment = Vec::with_capacity(num_pixels);
	// Fill boundary_point_group_assignment with '0'.
	for _ in 0..num_pixels {
		boundary_point_group_assignment.push(NO_NEIGHBOR);
	}

	let mut next_group_id = 1;
	for y in 0..threshold_image.height() {
		for x in 0..threshold_image.width() {

		}
	}

	boundary_point_group_assignment
}
