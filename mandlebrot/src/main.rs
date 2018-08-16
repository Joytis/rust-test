extern crate num;
extern crate image;
extern crate crossbeam;
extern crate num_cpus;
extern crate lerp;

use num::Complex;
use std::str::FromStr;  
use std::io::Write;
use image::{RgbImage, Rgb};
use lerp::Lerp;


/// Try to determine if 'c' is in the mandelbrot set, using at most 'limit' iterations to decide. 
///
/// If 'c' is not a member, return 'Some(i)', where 'i' is the number of iterations it took 
/// for 'c' to leave the circle of radius two centered on the origin. If 'c' seems to be a member
/// (more precisely, if we reached the iteration limit without being able to prove that 'c' is 
///	not a member), return 'None'
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
	let mut z = Complex { re: 0.0, im: 0.0 };
	for i in 0..limit {
		z = z*z + c;
		if z.norm_sqr() > 4.0 {
			return Some(i);
		}
	}
	None
}

/// Parse the string 's' as a coordinate pair, like '"400x600"' or "1.0,0.5"'.
///
/// Specifically, 's' should have the form <left><sep><right> where <sep> is the
/// character given by the seperator argument, and <left> and <right> are both. Strings
/// that can be parsed by 'T::from_str'.
///
/// If 's' has the proper form, return 'Some<(x, y)>'. If it doesn't parse correctly, return 'None'
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
	match s.find(separator) {
		None => None,
		Some(index) => {
			match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
				(Ok(l), Ok(r)) => Some((l, r)),
				_ => None
			}
		}
	}
}

#[test]
fn test_parse_pair() {
	assert_eq!(parse_pair::<i32>("", 		','), None);
	assert_eq!(parse_pair::<i32>("10", 		','), None);
	assert_eq!(parse_pair::<i32>(",10", 	','), None);
	assert_eq!(parse_pair::<i32>("10,20", 	','), Some((10, 20)));
	assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
	assert_eq!(parse_pair::<f64>("0.5x", 	'x'), None);
	assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}


fn parse_triad<T: FromStr>(s: &str, separator: char) -> Option<(T, T, T)> {
	let split: Vec<&str> =  s.split(separator).collect();
	if split.len() != 3 {
		return None;
	}
	match (T::from_str(split[0]), T::from_str(split[1]), T::from_str(split[2])) {
		(Ok(v1), Ok(v2), Ok(v3)) => Some((v1, v2, v3)),
		_ => None
	}
}

fn parse_rgb(s: &str) -> Option<Rgb<u8>> {
	match parse_triad(s, ',') {
		Some(col) => Some(Rgb{ data: [col.0, col.1, col.2] }),
		None => None
	}
}


// Parse a pair of floatin-point numbers separated by a comma as a complex number. 
fn parse_complex(s: &str) -> Option<Complex<f64>> {
	match parse_pair(s, ',') {
		Some((re, im)) => Some( Complex{ re, im }),
		None => None
	}
}

#[test]
fn test_parse_complex() {
	assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625}));
	assert_eq!(parse_complex(",-0.0625"), None);
}

/// Given the row and column of a pixel in the output image, return the cooresponding
///	point on the complex plane. 
///
/// 'bounds' is a pair giving the width and height o fth eimage in pixels
/// 'pixel' is a (column, row) pair indicating a particular pixel in that image.
/// The 'upper_left' and 'lower_right' parameters are points on the complex plane designating the area
/// our image covers. 
fn pixel_to_point(bounds: (usize, usize),
				  pixel: (usize, usize),
				  upper_left: Complex<f64>,
				  lower_right: Complex<f64>)
	-> Complex<f64>
{
	let (width, height) = (lower_right.re - upper_left.re,
						   upper_left.im - lower_right.im);
	Complex {
		re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
		im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
	}
}

#[test]
fn test_pixel_to_point() {
	assert_eq!(pixel_to_point((100, 100), (25, 75),
							  Complex{ re: -1.0, im:  1.0},
							  Complex{ re:  1.0, im: -1.0}),
			   Complex{ re: -0.5, im: -0.5 });
}

/// Render a rectabgle of the Mandelbrot set into a buffer of pixels
///
/// The 'bounds' argument gives the width and height of the buffer 'pixels',
/// which holds one garyscale pixel per byte. Th 'upper_left' and 'lower_right' 
/// arguments specity points on the complex plane corresponding to the upper-left
/// and lower-right corners of the pixel buffer. 
fn render(pixels: &mut [Rgb<u8>],
		  bounds: (usize, usize),
		  upper_left: Complex<f64>,
		  lower_right: Complex<f64>,
		  lower_color: Rgb<u8>,
		  upper_color: Rgb<u8>)
{
	assert!(pixels.len() == bounds.0 * bounds.1);

	for row in 0..bounds.1 {
		for column in 0..bounds.0 {
			let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

			let scalar: f32 = match escape_time(point, 10000) {
				None => 0.0,
				Some(count) => (10000.0 - count as f32) / 10000.0
			};

			for i in 0..3 {
				pixels[row * bounds.0 + column][i] = (lower_color[i] as f32).lerp((upper_color[i] as f32), scalar) as u8;
			}
		}
	}
}

// /// Write the buffer 'pixels', whose dimensions are given by 'bounds', to the file named 'filename'.
// fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
// 	let output = File::create(filename)?;

// 	let encoder = PNGEncoder::new(output);
// 	encoder.encode(&pixels,
// 				   bounds.0 as u32, bounds.1 as u32,
// 				   ColorType::Gray(8))?;

// 	Ok(())
// }

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() != 7 {
		writeln!(std::io::stderr(), "Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT LOWCOL HIGHCOL")
			.unwrap();
		writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.20,0.25 -1,0.20", args[0])
			.unwrap();
		std::process::exit(1);
	}

	// fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> 
	let bounds = parse_pair(&args[2], 'x').
		expect("error parsing image dimensions");
	let upper_left = parse_complex(&args[3])
		.expect("error parsing upper left corner point");
	let lower_right = parse_complex(&args[4])
		.expect("error parsing lower-right corner point");
	let lower_col = parse_rgb(&args[5])
		.expect("error parsing lower color");
	let upper_col = parse_rgb(&args[6])
		.expect("Error parsing upper col");

	// let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(bounds.0 as u32, bounds.1 as u32);

	let mut pixels: Vec<Rgb<u8>> = vec![Rgb{ data: [0, 0, 0] }; bounds.0 * bounds.1]; 

	let threads = num_cpus::get();
	let rows_per_band = bounds.1 / threads + 1;
	{
		let bands = pixels.chunks_mut(rows_per_band * bounds.0);
		crossbeam::scope(|spawner| {
			for(i, band) in bands.into_iter().enumerate() {
				let top = rows_per_band * i;
				let height = band.len() / bounds.0;
				let band_bounds = (bounds.0, height);
				let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
				let band_lower_right = 
					pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
				spawner.spawn(move || {
					render(band, band_bounds, band_upper_left, band_lower_right, lower_col, upper_col);
				});
			}
		});
	}

	assert!(bounds.0 * bounds.1 == pixels.len());
	let img = RgbImage::from_fn(bounds.0 as u32, bounds.1 as u32, |x, y| {
		*pixels.get((y * bounds.0 as u32 + x) as usize).expect("Index out of range")
	});
	img.save("mandelbrot.png").expect("Error: Could not save PNG image");

	// write_image(&args[1], &pixels, bounds)
	// 	.expect("error writing PNG file");
}
