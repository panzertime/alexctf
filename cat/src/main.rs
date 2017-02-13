extern crate image;
extern crate rustc_serialize;

use image::{
	GenericImage,
	ImageBuffer,
	Pixel
};
use std::fs::File;
use std::path::Path;
use std::env;
use rustc_serialize::hex::FromHex;
use std::str;


/*
So, we want to open the image from the stream (is possible?  who knows?), then we want to decode from a PNG to 
an ImageBuffer, then iterate over the pixels and perform the same check as the python library
*/


fn main() {
	let args: Vec<_> = env::args().collect();
	let img = image::open(&Path::new(&args[1])).unwrap();

	let (width, height) = img.dimensions();
	
	let mut acc = 0;
	let mut result = String::new();

	for y in 0..height {
		for x in 0..width {
			if is_changed(& img.get_pixel(x, y)) {
				result.push(format!("{:X}", acc));
			}
			acc += 1;
			if acc == 16 {
				acc = 0;
			}
		}
	}


	println!("{}", str::from_utf8(result.from_hex().unwrap()).unwrap());		
		
Ok(())

}

fn is_changed<P: Pixel>(p: & P) -> bool {
	let r = p.to_rgb()[0] % 8;
	let g = p.to_rgb()[1] % 8;
	let b = p.to_rgb()[2] % 8;
	r == 1 && g == 1 && b == 1
}
