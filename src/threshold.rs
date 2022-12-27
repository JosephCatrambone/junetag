
use image::{DynamicImage, GenericImageView};
use image::GrayImage;
use image::io::Reader as ImageReader;
use image::Luma;
use rayon::prelude::*;

const BLACK_PX: u8 = 0;
const WHITE_PX: u8 = 255;
const GREY_PX: u8 = 127;

/// Instead of computing the exact extrema (max and min
/// values) around every pixel, we divide the image into tiles
/// of 4x4 pixels and compute the extrema within each tile. To
/// prevent artifacts from arising between tile boundaries with
/// large differences in extreme values, we find the extrema in a
/// neighborhood of 3x3 surrounding tiles, ensuring a minimum
/// of one tile overlap when computing extrema for adjacent
/// pixels. Each pixel is then assigned a value of white or
/// black, using the mean value (max+min)/2 as the threshold
/// (Figure 3b).
///
/// See https://github.com/AprilRobotics/apriltag/blob/master/apriltag_quad_thresh.c
/// image_u8_t *threshold(apriltag_detector_t *td, image_u8_t *im)
///
/// JC Note: I think this was terribly explained in the paper.  It seems like it's just a conv with
/// a 3x3 grid, and it's not clear how that's different from just selecting the max over a 12x12
/// pixel box.  We'll duplicate the structure of the original code, where they first iterate over
/// superpixels (tiles) of size 4x4, then go over these again and use a full convolution of +/- 1
/// superpixel for thresholding.
pub fn adaptive_threshold_original(image_view: &dyn GenericImageView<Pixel=Luma<u8>>, tile_size: u32, min_contrast: u8) -> GrayImage {
	let mut result = GrayImage::new(image_view.width(), image_view.height());

	let width = image_view.width();
	let height = image_view.height();
	let width_in_tiles = (width / (tile_size as u32))+1;
	let height_in_tiles = (height / (tile_size as u32))+1;
	//let mut tile_min = DynamicImage::new_luma8(width_in_tiles, height_in_tiles).into_luma8();
	let mut tile_min = GrayImage::new(width_in_tiles, height_in_tiles);
	let mut tile_max = GrayImage::new(width_in_tiles, height_in_tiles);

	// Pre-fill tile mins.
	for tile_y in 0..height_in_tiles {
		for tile_x in 0..width_in_tiles {
			*tile_min.get_pixel_mut(tile_x, tile_y) = Luma([WHITE_PX]);
			*tile_max.get_pixel_mut(tile_x, tile_y) = Luma([BLACK_PX]);
		}
	}

	// Compute the per-tile extrema.
	//let mut par_iter = (0..width_in_tiles*height_in_tiles).into_par_iter().map(|idx| {
	for tile_y in 0..height_in_tiles {
		for tile_x in 0..width_in_tiles {
			let mut local_tile_max = BLACK_PX;
			let mut local_tile_min = WHITE_PX;
			for pixel_offset_y in 0..tile_size {
				for pixel_offset_x in 0..tile_size {
					let pixel_x = tile_x*tile_size + pixel_offset_x;
					let pixel_y = tile_y*tile_size + pixel_offset_y;
					if pixel_x >= width || pixel_y >= height { continue; }
					let pixel = image_view.get_pixel(pixel_x, pixel_y)[0];
					local_tile_max = pixel.max(local_tile_max);
					local_tile_min = pixel.min(local_tile_min);
				}
			}
			*tile_min.get_pixel_mut(tile_x, tile_y) = Luma([local_tile_min]);
			*tile_max.get_pixel_mut(tile_x, tile_y) = Luma([local_tile_max]);
		}
	}

	// Perform a convolution over neighborhoods.
	let mut blurred_tile_min = GrayImage::new(width_in_tiles, height_in_tiles);
	let mut blurred_tile_max = GrayImage::new(width_in_tiles, height_in_tiles);
	for tile_y in 0..height_in_tiles {
		for tile_x in 0..width_in_tiles {
			let mut conv_min = WHITE_PX;
			let mut conv_max = BLACK_PX;
			for dy in &[-1i32, 0, 1] {
				for dx in &[-1i32, 0, 1] {
					let x = tile_x as i32 + dx;
					let y = tile_y as i32 + dy;
					if x < 0 || x >= width_in_tiles as i32 || y < 0 || y >= height_in_tiles as i32 {
						continue;
					}
					conv_min = conv_min.min(tile_min.get_pixel(x as u32, y as u32)[0]);
					conv_max = conv_max.max(tile_max.get_pixel(x as u32, y as u32)[0]);
				}
			}
			*blurred_tile_min.get_pixel_mut(tile_x, tile_y) = Luma([conv_min]);
			*blurred_tile_max.get_pixel_mut(tile_x, tile_y) = Luma([conv_max]);
		}
	}

	// Finally, go over the original image and try to threshold.
	for y in 0..height {
		for x in 0..width {
			let adaptive_min = blurred_tile_min.get_pixel(x/tile_size, y/tile_size)[0];
			let adaptive_max = blurred_tile_max.get_pixel(x/tile_size, y/tile_size)[0];
			let mut p = result.get_pixel_mut(x, y);
			if adaptive_max.abs_diff(adaptive_min) < min_contrast {
				*p = Luma([GREY_PX]);
				continue;
			}

			//uint8_t thresh = min + (max - min) / 2;
			let threshold = (adaptive_min as u16 + ((adaptive_max as u16 + adaptive_min as u16)/2)) as u8;

			if image_view.get_pixel(x, y)[0] <= threshold {
				*p = Luma([BLACK_PX]);
			} else {
				*p = Luma([WHITE_PX]);
			}
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use image::DynamicImage::ImageLuma8;
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;

	#[test]
	fn test_threshold() {
		let img = ImageReader::open("apriltag_input.png").unwrap().decode().unwrap().to_luma8();
		//assert_eq!(add(1, 2), 3);
		let thresh = adaptive_threshold_original(&img, 4, 30);
		thresh.save("apriltag_threshold_OURS_OUTPUT.png");
	}
}