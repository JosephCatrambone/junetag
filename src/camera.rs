
use glam::f32::{Mat3, Mat4, Quat, Vec3};

#[derive(Debug, Default)]
pub struct CameraParameters {
	// Radial Distortion
	k1: f32, // Larger than zero for barrel/fisheye distortion.  Less than zero for pincushion.
	k2: f32,
	k3: f32,
	k4: f32,
	k5: f32,
	k6: f32,
	// Tangential distortion:
	p1: f32,
	p2: f32,

	// Intrinsics:
	// Center of the image.  Principal point.  These are floats because of subpixel values.
	c1: f32,
	c2: f32,
	// Focal length in pixel units.
	f1: f32,
	f2: f32,

	// Extrinsic parameters:
	translation: Vec3,
	rotation: Quat,
}

impl CameraParameters {
	pub fn new(x_resolution: u32, y_resolution: u32) -> Self {
		let mut cam = CameraParameters::default();
		cam.c1 = x_resolution as f32 * 0.5;
		cam.c2 = y_resolution as f32 * 0.5;

		cam.f1 = 1.0;
		cam.f2 = 1.0;

		cam
	}

	pub fn project_point(&self, x: f32, y: f32, z: f32) -> (f32, f32) {
		let preprojection = self.rotation.mul_vec3(Vec3::new(x, y, z)) + self.translation;

		let x_prime = preprojection.x / preprojection.z;
		let y_prime = preprojection.y / preprojection.z;

		// Idealized undistorted UV:
		//let u = (self.f1 * x_prime) + self.c1;
		//let v = (self.f2 * y_prime) + self.c2;

		let r2 = x_prime*x_prime + y_prime*y_prime;

		let radial_distortion =
			(1.0 + (self.k1*r2) + (self.k2*r2*r2) + (self.k3*r2*r2*r2)) /
			(1.0 + (self.k4*r2) + (self.k5*r2*r2) + (self.k6*r2*r2*r2));

		let xpp = x_prime * radial_distortion + 2.0*self.p1*x_prime*y_prime + self.p2;
		let ypp = y_prime * radial_distortion + self.p1*(r2+2.0*y_prime*y_prime) + 2.0*self.p2*x_prime*y_prime;

		let u = self.f1 * xpp + self.c1;
		let v = self.f2 * ypp + self.c2;

		(u, v)
	}

	pub fn set_fov(&mut self, fov: f32) {
		// fov = 2*arctan( film diagonal / 2*f )
		// fov / 2 = arctan( film diagonal / 2*f )
		// tan(fov / 2) = film diagonal / 2*f
		// 2*tan(fov / 2) = film diagonal / f
		// 2 * tan(fov / 2) / film_diagonal = 1 / f
		let film_width:f32 = 2f32*self.c1;
		let film_height:f32 = 2f32*self.c2;
		let film_diagonal = ((film_width*film_width) + (film_height*film_height)).sqrt();
		self.f1 = film_diagonal / (2f32 * (fov * 0.5).tan());
		self.f2 = self.f1;
	}

	pub fn look_at_from(&mut self, origin: Vec3, target: Vec3, up: Vec3) {
		// Right-hand system:
		// Rx Ry Rz 0
		// Ux Uy Uz 0
		// Fx Fy Fz 0
		// Tx Ty Tz 1

		// We're using a right-handed z-up.
		let forward = (target - origin).normalize();
		let right = forward.cross(up);
		let mat = Mat3::from_cols(
			right,
			up,
			forward,
		);

		self.rotation = Quat::from_mat3(&mat);
	}

	pub fn translate_to(&mut self, x: f32, y: f32, z: f32) {
		self.translation.x = x;
		self.translation.y = y;
		self.translation.z = z;
	}

	pub fn translate_by(&mut self, x: f32, y: f32, z: f32) {
		self.translation.x += x;
		self.translation.y += y;
		self.translation.z += z;
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_project_point() {
		let p10 = na::Point3::new(8.0, 9.0, 10.0);
		//assert_eq!(result, 4);
		let camera = CameraParameters { f1: 1.0f32, f2: 1.0f32, c1: 320f32, c2: 240f32, ..CameraParameters::default() };
		println!("{}", camera.project_point(&p10));
	}
}
