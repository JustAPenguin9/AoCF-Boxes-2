use image::Rgba;
use imageproc::rect::Rect;

mod matches;
mod types;

use matches::get_matches;
use types::{Colour, Move};

#[inline]
fn error_out<T: AsRef<str>>(msg: T) -> ! {
	println!("{}", ansi_term::Colour::Red.bold().paint(format!("error {}", msg.as_ref())));

	std::process::exit(1)
}

fn main() {
	let matches = get_matches();

	// will never error because its a required argument
	let path = matches.get_one::<std::path::PathBuf>("input_file").unwrap();

	let f = std::fs::File::open(path).expect("no file found");

	let file: Move = match ron::de::from_reader(f) {
		Ok(x) => x,
		Err(e) => {
			println!("Failed to load {path:?} {e}");

			std::process::exit(1);
		}
	};

	// will never error because there is a default value
	let output_dir = matches.get_one::<String>("output_dir").unwrap();
	if !std::path::Path::new(output_dir).is_dir() {
		std::fs::create_dir(output_dir)
			.unwrap_or_else(|_| error_out("Could not create the output directory"))
	}

	let mut frames: Vec<image::Frame> = vec![];
	let mut can_make_gif = true;

	let mut i = 0;
	for image in file.images {
		let sprite = image::open(format!("{}{}", file.directory, image.file))
			.unwrap_or_else(|_| error_out(format!("Can not find the image {}", image.file)));
		let mut base = image::DynamicImage::new_rgba8(
			sprite.width() + file.padding_tlbr.1 + file.padding_tlbr.3,
			sprite.height() + file.padding_tlbr.0 + file.padding_tlbr.2,
		);

		if !matches.get_flag("no_sprites") {
			image::imageops::overlay(
				&mut base,
				&sprite,
				file.padding_tlbr.1.into(),
				file.padding_tlbr.0.into(),
			);
		}

		for b in image.boxes {
			let colour = match b.colour {
				Colour::Hit => Rgba([255u8, 0u8, 0u8, 255u8]),
				Colour::Hit2 => Rgba([255u8, 255u8, 0u8, 255u8]),
				Colour::Hurt => Rgba([0u8, 255u8, 0u8, 255u8]),
				Colour::Collision => Rgba([0u8, 0u8, 255u8, 255u8]),
				Colour::Hex(mut val) => {
					// left shift the value 8 bits because the input is only 24 bits for RGB missing the 8 alpha bits
					// then OR the number with 0x000000FF so that its always completly visable
					val = val << 8 | 0x000000FF;
					let arr = val.to_be_bytes();
					Rgba(arr)
				}
			};

			imageproc::drawing::draw_hollow_rect_mut(
				&mut base,
				Rect::at(
					image.crop_xy.0 - image.matrix30 - b.size_wh.0 as i32
						+ b.offset_xy.0 + file.padding_tlbr.1 as i32
						+ 1,
					image.crop_xy.1 - image.matrix31 - b.size_wh.1 as i32
						+ b.offset_xy.1 + file.padding_tlbr.0 as i32,
				)
				.of_size(b.size_wh.0 * 2, b.size_wh.1 * 2 + 1),
				colour,
			)
		}

		let name = format!("{}/{}-boxes{i:02}.png", output_dir, file.name);
		base.save(&name).unwrap_or_else(|_| error_out(format!("Could not save the image {name}")));

		if image.exposure.is_none() {
			can_make_gif = false
		} else {
			// will never error because the None case is checked above
			for _ in 0..image.exposure.unwrap() {
				frames.push(image::Frame::new(base.clone().into_rgba8()));
			}
		}

		i += 1;
	}

	// save the gif if needed
	match (matches.get_flag("gif"), can_make_gif) {
		(true, true) => {
			let path = std::fs::File::create(format!("{}/{}.gif", output_dir, file.name))
				.unwrap_or_else(|_| error_out("Could not encode the gif"));
			let mut encoder = image::codecs::gif::GifEncoder::new(path);
			encoder
				.set_repeat(image::codecs::gif::Repeat::Infinite)
				.unwrap_or_else(|_| error_out("Could not encode the gif"));

			encoder.encode_frames(frames).unwrap_or_else(|_| error_out("Could not encode the gif"));
		}
		(true, false) => {
			println!("Could not make a gif because not all images had a valid exposure")
		}
		_ => (),
	}
}
