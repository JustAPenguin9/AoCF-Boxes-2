use image::Rgba;
use imageproc::rect::Rect;

mod matches;
mod types;

use matches::get_matches;
use types::{Colour, Move};

macro_rules! error_out {
    ($($arg:tt)*) => {{
        print!("error: ");
        println!($($arg)*);
        std::process::exit(1)
    }}
}

fn main() {
	let matches = get_matches();

	// mostly stolen from the println macro in the standard library because macros are witchcraft
	// definied in main to use matches
	macro_rules! print_verbose {
        ($($arg:tt)*) => {
            if matches.get_flag("verbose") {
                // println!("verbose: {}", std::format_args_nl!($($arg)*))
                // print!("verbose: ");
                println!($($arg)*);
            }
        }
    }

	// will never error because its a required argument
	let path = matches.get_one::<std::path::PathBuf>("input_file").unwrap();
	print_verbose!("Got path to file {}", path.to_string_lossy());

	let f = std::fs::read_to_string(path)
		.unwrap_or_else(|_| error_out!("Could not open {}", path.to_string_lossy()));
	print_verbose!("Opened file");

	let file: Move = match matches.get_one::<String>("format").unwrap().as_str() {
		"json" => serde_json::from_str(&f)
			.unwrap_or_else(|e| error_out!("Failed to parse file {path:?} {e}")),
		"yaml" => serde_yaml::from_str(&f)
			.unwrap_or_else(|e| error_out!("Failed to parse file {path:?} {e}")),
		"toml" => {
			toml::from_str(&f).unwrap_or_else(|e| error_out!("Failed to parse file {path:?} {e}"))
		}
		// defaults to ron
		_ => ron::de::from_str(&f)
			.unwrap_or_else(|e| error_out!("Failed to parse file {path:?} {e}")),
	};
	print_verbose!("Deserialized file");

	// will never error because there is a default value
	let output_dir = matches.get_one::<String>("output_dir").unwrap();
	if !std::path::Path::new(output_dir).is_dir() {
		std::fs::create_dir(output_dir)
			.unwrap_or_else(|_| error_out!("Could not create the output directory"));
		print_verbose!("Created output folder");
	}

	let mut frames: Vec<image::Frame> = vec![];
	let mut can_make_gif = true;

	let mut i = 0;
	for image in file.images {
		let sprite = image::open(format!("{}{}", file.directory, image.file))
			.unwrap_or_else(|_| error_out!("Could not find the image {}", image.file));
		print_verbose!("Opened image {}", image.file);
		let mut base = image::DynamicImage::new_rgba8(
			sprite.width() + file.padding_tlbr.1 + file.padding_tlbr.3,
			sprite.height() + file.padding_tlbr.0 + file.padding_tlbr.2,
		);
		print_verbose!("└── Created base image");

		if !matches.get_flag("no_sprites") {
			image::imageops::overlay(
				&mut base,
				&sprite,
				file.padding_tlbr.1.into(),
				file.padding_tlbr.0.into(),
			);
			print_verbose!("└── Applied sprite to base");
		}

        print_verbose!("└── Drawing box(es)");
		for b in image.boxes {
			let colour = match b.colour {
				Colour::Hit => Rgba([255u8, 0u8, 0u8, 255u8]),
				Colour::Hit2 => Rgba([255u8, 255u8, 0u8, 255u8]),
				Colour::Hurt => Rgba([0u8, 255u8, 0u8, 255u8]),
				Colour::Collision => Rgba([0u8, 0u8, 255u8, 255u8]),
				// left shift the value 8 bits because the input is only 24 bits for RGB missing the 8 alpha bits
				// then OR the number with 0x000000FF so that its always completly visable
				Colour::Hex(val) => Rgba((val << 8 | 0x000000FF).to_be_bytes()),
			};

			let pos_x = image.crop_xy.0 - image.matrix_3031.0 - b.size_wh.0 as i32
				+ b.offset_xy.0 + file.padding_tlbr.1 as i32
				+ 1;
			let pos_y = image.crop_xy.1 - image.matrix_3031.1 - b.size_wh.1 as i32
				+ b.offset_xy.1 + file.padding_tlbr.0 as i32;

			let width = b.size_wh.0 * 2;
			let height = b.size_wh.1 * 2 + 1;

			if width <= 0 || height <= 0 {
				print_verbose!("│   └── Skipping {:?} box due to a width or height of 0", b.colour);
				continue;
			}

			imageproc::drawing::draw_hollow_rect_mut(
				&mut base,
				Rect::at(pos_x, pos_y).of_size(width, height),
				colour,
			);
			print_verbose!(
				"│   └── Drew {:?} box (x: {}, y: {}, width: {}, height: {})",
				b.colour,
				&pos_x,
				&pos_y,
				&width,
				&height
			);
		}

		let name = format!("{}/{}-boxes{i:02}.png", output_dir, file.name);
		base.save(&name).unwrap_or_else(|_| error_out!("Could not save the image {name}"));
		print_verbose!("└── Saved {}", name);

		if image.exposure.is_none() {
			can_make_gif = false
		} else {
			// will never error because the None case is checked above
			for _ in 0..image.exposure.unwrap() {
				frames.push(image::Frame::new(base.clone().into_rgba8()));
			}
            if matches.get_flag("gif") {
                print_verbose!("└── Pushed the image onto gif stack {} time(s)", image.exposure.unwrap());
            }
		}

		i += 1;
	}

	// save the gif if needed
	match (matches.get_flag("gif"), can_make_gif) {
		(true, true) => {
			let path = format!("{}/{}.gif", output_dir, file.name);
			let gif = std::fs::File::create(&path)
				.unwrap_or_else(|_| error_out!("Could not encode the gif"));
			let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(
				&gif,
				file.speed.unwrap_or_else(|| 1),
			);
			encoder
				.set_repeat(image::codecs::gif::Repeat::Infinite)
				.unwrap_or_else(|_| error_out!("Could not encode the gif"));

			encoder
				.encode_frames(frames)
				.unwrap_or_else(|_| error_out!("Could not encode the gif"));
			print_verbose!("Saved gif {}", path);
		}
		(true, false) => {
			println!("Could not make a gif because not all images had a valid exposure")
		}
		_ => (),
	}

	println!("Done! Saved to {}", output_dir);
}
