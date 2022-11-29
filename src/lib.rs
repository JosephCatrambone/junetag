
mod threshold;
mod unionfind;

/*
#[repr(C)]
#[derive(Default)]
pub struct Detection {
}
*/

/*
extern "C" {
	pub fn detect(image_data: Vec<u8>, width: usize) -> Option<Detection>;
}
*/


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
