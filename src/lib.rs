use glam::{Quat, UVec2, UVec4, Vec3};

pub mod camera;
pub mod threshold;
pub mod unionfind;
pub mod quad_detect;

#[repr(C)]
#[derive(Default)]
pub struct CDetection {
	// Keep the types basic to make it easier to interop with C.
	// This is _NOT_ for the sake of a C binding, but for other programs that might want to use a simple rust type.

	// Define a rectangle to hold the whole detection in-image.
	pixel_x: u32,
	pixel_y: u32,
	pixel_w: u32,
	pixel_h: u32,

	// Quad points:
	a_x: u32,
	a_y: u32,
	b_x: u32,
	b_y: u32,
	c_x: u32,
	c_y: u32,
	d_x: u32,
	d_y: u32,

	// Pose data:
	x: f32,
	y: f32,
	z: f32,
	normal_x: f32,
	normal_y: f32,
	normal_z: f32,

	// Encoding:
	marker_id: u32,
	//code_family_bits: u8,  // The '36' in 36h11.
	//code_family_hamming: u8, // The '12' in 48h12.
}

impl From<Detection> for CDetection {
	fn from(det: Detection) -> Self {
		CDetection {
			pixel_x: det.image_rect.x,
			pixel_y: det.image_rect.y,
			pixel_w: det.image_rect.z,
			pixel_h: det.image_rect.w,

			a_x: det.quad_corners[0].x,
			a_y: det.quad_corners[0].y,
			b_x: det.quad_corners[1].x,
			b_y: det.quad_corners[1].y,
			c_x: det.quad_corners[2].x,
			c_y: det.quad_corners[2].y,
			d_x: det.quad_corners[3].x,
			d_y: det.quad_corners[3].y,

			x: det.translation.x,
			y: det.translation.y,
			z: det.translation.z,
			normal_x: 0.0,
			normal_y: 0.0,
			normal_z: 0.0,

			marker_id: det.marker_id,
		}
	}
}

// Our internal structure, more rusty.
#[derive(Default)]
struct Detection {
	image_rect: UVec4,
	quad_corners: [UVec2; 4], // Four points in the quad.
	translation: Vec3,
	rotation: Quat,
	marker_id: u32,
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
