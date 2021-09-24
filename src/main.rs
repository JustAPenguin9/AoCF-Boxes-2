use std::env;
use std::fs::create_dir;
use std::path::Path;

use csv::{ReaderBuilder, Trim};
use image::{open, GenericImageView, Rgba};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

type Row = (String, i32, i32, i32, i32);

fn main() {
	/* CREATE AN OUTPUT DIR IF NONE EXIST ALREADY */
	if !Path::new("output").is_dir() {
		create_dir("output").expect("could not create output dir");
	}

	/* CREATE THE READER */
	let file_path = env::args()
		.nth(1)
		.expect("missing csv file as the second argument");
	let mut rdr = ReaderBuilder::new()
		.has_headers(true)
		.flexible(true)
		.comment(Some(b'#'))
		.trim(Trim::All)
		.from_path(file_path)
		.expect("could not retrieve csv file");
	let records = rdr.deserialize().collect::<Result<Vec<Row>, csv::Error>>().expect(
	"error deserializing the csv file, maybe the data wasn't included correctly? the header should have 4 columbs and each row shold have 6 columbs"
	);

	/* GET THE DATA FROM THE HEADER */
	let header = rdr.headers().unwrap();
	let path_to_image: String = header[0].parse().unwrap();
	let padding_top: u32 = header[1]
		.parse()
		.expect("missing the top padding in the csv");
	let padding_left: u32 = header[2]
		.parse()
		.expect("missing the left padding in the csv");
	let padding_bottom: u32 = header[3]
		.parse()
		.expect("missing the bottom padding in the csv");
	let padding_right: u32 = header[4]
		.parse()
		.expect("missing the right padding in the csv");

	/* FIRST IMAGE */
	let mut sprite_name: &String = &records[0].0;
	let mut x_crop: i32 = records[0].1;
	let mut y_crop: i32 = records[0].2;
	let mut matrix30: i32 = records[0].3;
	let mut matrix31: i32 = records[0].4;

	println!("opening {}{}", path_to_image, sprite_name);
	let mut sprite: image::DynamicImage = open(format!("{}{}", path_to_image, sprite_name))
		.expect(&format!("error getting the first image"));
	let img_width = sprite.width() + padding_left + padding_right;
	let img_height = sprite.height() + padding_top + padding_bottom;
	let mut img = image::DynamicImage::new_rgba8(img_width, img_height);
	image::imageops::overlay(&mut img, &sprite, padding_left, padding_top);
	println!("opened {}{}", path_to_image, sprite_name);

	for (i, row) in records.iter().enumerate() {
		match row.0.as_str() {
			"coll" | "collision" => {
				draw_hollow_rect_mut(
					&mut img,
					Rect::at(
						x_crop - matrix30 - row.1 as i32 + row.3 + 1 + padding_left as i32,
						y_crop - matrix31 - row.2 as i32 + row.4 + padding_top as i32,
					)
					.of_size((row.1 * 2) as u32, (row.2 * 2 + 1) as u32),
					Rgba([0u8, 0u8, 255u8, 255u8]),
				);
				println!("drew collision box on {}", sprite_name);
			}
			"hurt" => {
				draw_hollow_rect_mut(
					&mut img,
					Rect::at(
						x_crop - matrix30 - row.1 as i32 + row.3 + 1 + padding_left as i32,
						y_crop - matrix31 - row.2 as i32 + row.4 + padding_top as i32,
					)
					.of_size((row.1 * 2) as u32, (row.2 * 2 + 1) as u32),
					Rgba([0u8, 255u8, 0u8, 255u8]),
				);
				println!("drew hurt box on {}", sprite_name);
			}
			"hit" => {
				draw_hollow_rect_mut(
					&mut img,
					Rect::at(
						x_crop - matrix30 - row.1 as i32 + row.3 + 1 + padding_left as i32,
						y_crop - matrix31 - row.2 as i32 + row.4 + padding_top as i32,
					)
					.of_size((row.1 * 2) as u32, (row.2 * 2 + 1) as u32),
					Rgba([255u8, 0u8, 0u8, 255u8]),
				);
				println!("drew hit box on {}", sprite_name);
			}
			_ => {
				/* SPRITE / IMAGE */
				if i != 0 {
					img.save(format!("output/{}", sprite_name))
						.expect("could not save image");
					println!("saved image {} to output dir", sprite_name);

					sprite_name = &row.0;
					x_crop = row.1;
					y_crop = row.2;
					matrix30 = row.3;
					matrix31 = row.4;

					println!("opening {}{}", path_to_image, sprite_name);
					sprite = open(format!("{}{}", path_to_image, sprite_name))
						.expect(&format!("might be an error on line {}", i + 1));
					img = image::DynamicImage::new_rgba8(img_width, img_height);
					image::imageops::overlay(&mut img, &sprite, padding_left, padding_top);
					println!("opened {}{}", path_to_image, sprite_name);
				}
			}
		};
	}
	img.save(format!("output/{}", sprite_name))
		.expect("could not save image");
	println!("saved image {} to output dir", sprite_name);
}
