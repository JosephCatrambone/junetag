
use glam::Vec3;
use minifb::{Key, Window, WindowOptions};
use minifb::Key::H;

use junetag::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
	let mut frame: u64 = 0;
	let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

	let mut camera_params = camera::CameraParameters::new(WIDTH as u32, HEIGHT.try_into().unwrap());
	camera_params.set_fov(45.0f32.to_radians());
	camera_params.translate_by(0.0, -5.0, 0.0);

	let mut green_unit = vec![];
	// Add a box.
	for i in 0..100 {
		green_unit.push(Vec3::new(0.0, 0.0, i as f32 / 100.0));
		green_unit.push(Vec3::new(0.0, 1.0, i as f32 / 100.0));
		green_unit.push(Vec3::new(1.0, 0.0, i as f32 / 100.0));
		green_unit.push(Vec3::new(1.0, 1.0, i as f32 / 100.0));
		green_unit.push(Vec3::new(0.0, i as f32 / 100.0, 0.0));
		green_unit.push(Vec3::new(0.0, i as f32 / 100.0, 1.0));
		green_unit.push(Vec3::new(1.0, i as f32 / 100.0, 0.0));
		green_unit.push(Vec3::new(1.0, i as f32 / 100.0, 1.0));
		green_unit.push(Vec3::new(i as f32 / 100.0, 0.0, 0.0));
		green_unit.push(Vec3::new(i as f32 / 100.0, 0.0, 1.0));
		green_unit.push(Vec3::new(i as f32 / 100.0, 1.0, 0.0));
		green_unit.push(Vec3::new(i as f32 / 100.0, 1.0, 1.0));
	}

	let mut window = Window::new(
		"Test - ESC to exit",
		WIDTH,
		HEIGHT,
		WindowOptions::default(),
	)
		.unwrap_or_else(|e| {
			panic!("{}", e);
		});

	// Limit to max ~60 fps update rate
	window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

	while window.is_open() && !window.is_key_down(Key::Escape) {
		// Clear.
		for i in buffer.iter_mut() {
			*i = 0;
		}

		// Project all points based on the camera.
		let new_pts: Vec<(f32, f32)> = green_unit.iter().map(|p| { (&camera_params).project_point(p.x, p.y, p.z).clone() }).collect();

		for p in new_pts {
			if p.0 >= 0.0 && p.0 < WIDTH as f32 && p.1 >= 0.0 && p.1 < HEIGHT as f32 {
				let x = p.0 as usize;
				let y = p.1 as usize;
				buffer[x + y*WIDTH] = 0x00FF00;
			}
		}

		// Move camera.
		camera_params.translate_by(
			if window.is_key_down(Key::A) { -0.1f32 } else if window.is_key_down(Key::Q) { 0.1f32 } else { 0.0 },
			if window.is_key_down(Key::S) { -0.1f32 } else if window.is_key_down(Key::W) { 0.1f32 } else { 0.0 },
			if window.is_key_down(Key::D) { -0.1f32 } else if window.is_key_down(Key::E) { 0.1f32 } else { 0.0 },
		);
		if frame % 15 == 0 {
			dbg!(&camera_params);
		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window
			.update_with_buffer(&buffer, WIDTH, HEIGHT)
			.unwrap();
		frame += 1;
	}
}