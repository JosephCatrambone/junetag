
use glam::Vec3;
use image;
use image::{ImageBuffer, imageops, Luma};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use nokhwa::{
	nokhwa_initialize,
	pixel_format::{RgbAFormat, RgbFormat},
	query,
	utils::{ApiBackend, RequestedFormat, RequestedFormatType},
	CallbackCamera,
};
use std::sync::mpsc;
use std::sync::mpsc::SendError;
use std::thread;
use std::time::Duration;

use junetag::*;


fn main() {
	let mut tile_size: u8 = 3;
	let mut min_difference: u8 = 5;

	//
	// Webcam Setup:
	//

	// Permissions (OSX)
	nokhwa_initialize(|granted| {
		println!("Granted: {}", granted);
	});
	let cameras = query(ApiBackend::Auto).unwrap();
	cameras.iter().for_each(|cam| println!("{:?}", cam));

	let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
	let webcam = cameras.first().unwrap();
	let (tx, rx) = mpsc::sync_channel(10);

	let mut webcam_thread = CallbackCamera::new(webcam.index().clone(), format, move |buffer| {
		loop {
			let image = buffer.decode_image::<RgbAFormat>().unwrap();
			println!("{}x{} {}", image.width(), image.height(), image.len());
			match tx.try_send(imageops::grayscale(&image)) {
				Ok(_) => {
					println!("Sending image");
					thread::sleep(Duration::from_millis(100));
				}
				Err(mpsc::TrySendError::Disconnected(_)) => {
					println!("Quitting camera thread.");
					return;
				}
				Err(mpsc::TrySendError::Full(_)) => {
					// Drop the frame and sleep.
					thread::sleep(Duration::from_millis(100));
				}
			}
		}
	}).unwrap();
	webcam_thread.open_stream().expect("Unable to open camera stream.");

	//
	// Screen Buffer Setup:
	//

	// Wait for the first frame...
	let first_image = rx.recv().unwrap();
	let mut frame: u64 = 0;
	let mut buffer: Vec<u32> = vec![0; (first_image.width() * first_image.height()) as usize];

	let mut window = Window::new(
		"Test - ESC to exit",
		first_image.width() as usize,
		first_image.height() as usize,
		WindowOptions::default(),
	)
		.unwrap_or_else(|e| {
			panic!("{}", e);
		});

	// Limit to max ~60 fps update rate
	window.limit_update_rate(Some(Duration::from_micros(16600)));

	//
	// Main Loop:
	//

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let img = rx.recv().unwrap();
		let thresh = threshold::adaptive_threshold_original(&img, 5, 10);
		for i in buffer.iter_mut() {
			*i = 0;
		}

		if window.is_key_pressed(Key::Q, KeyRepeat::No) {
			tile_size = tile_size.saturating_add(1);
		}
		if window.is_key_pressed(Key::A, KeyRepeat::No) {
			tile_size = tile_size.saturating_sub(1);
		}
		if window.is_key_pressed(Key::W, KeyRepeat::No) {
			min_difference = min_difference.saturating_add(1);
		}
		if window.is_key_pressed(Key::S, KeyRepeat::No) {
			min_difference = min_difference.saturating_sub(1);
		}

		if frame % 15 == 0 {
			println!("{} / {}", tile_size, min_difference);
		}

		// We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
		window
			.update_with_buffer(&buffer, first_image.width() as usize, first_image.height() as usize)
			.unwrap();
		frame += 1;
	}
}