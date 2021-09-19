extern crate image;
extern crate imageproc;
extern crate csv;

use std::fs::create_dir;
use std::path::Path;
use std::env;

use csv::{ReaderBuilder, Trim};
use image::{open, Rgba};
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
		.from_path(file_path).expect("could not retrieve csv file");
	let records = rdr.deserialize().collect::<Result<Vec<Row>, csv::Error>>().expect(
		"error deserializing the csv file, maybe you didnt include the data correctly?
		the header should have 4 columbs and the row shold have 6 columbs"
		);

	/* GET THE DATA FROM THE HEADER */
	let header = rdr.headers().unwrap();
	let path_to_image :String = header[0].parse().unwrap();

	/* IMAGE */
	let mut sprite_name :&String = &records[0].0;
	let mut x_crop :i32 = records[0].1;
	let mut y_crop :i32 = records[0].2;
	let mut matrix30 :i32 = records[0].3;
	let mut matrix31 :i32 = records[0].4;

	println!("opening: {}/{}", path_to_image, sprite_name);
	let mut img :image::DynamicImage = open(format!("{}/{}", path_to_image, sprite_name)).expect(&format!("error getting the first image"));

	for (i, row) in records.iter().enumerate() {		
		match row.0.as_str() {
			"coll" | "collision" => { 
				draw_hollow_rect_mut(&mut img, Rect::at(
					x_crop - matrix30 - row.1 as i32 + row.3 + 1, // + extra space left
					y_crop - matrix31 - row.2 as i32 + row.4 // + extra space top
				).of_size(
					(row.1 * 2) as u32,
					(row.2 * 2 + 1) as u32
				), Rgba([0u8, 0u8, 255u8, 255u8]));
				println!("drew sprite {}", sprite_name);
			}
			"hurt"               => {
				draw_hollow_rect_mut(&mut img, Rect::at(
					x_crop - matrix30 - row.1 as i32 + row.3 + 1, // + extra space left
					y_crop - matrix31 - row.2 as i32 + row.4 // + extra space top
				).of_size(
					(row.1 * 2) as u32,
					(row.2 * 2 + 1) as u32
				), Rgba([0u8, 255u8, 0u8, 255u8]));
				println!("drew sprite {}", sprite_name);
			}
			"hit"                => {
				draw_hollow_rect_mut(&mut img, Rect::at(
					x_crop - matrix30 - row.1 as i32 + row.3 + 1, // + extra space left
					y_crop - matrix31 - row.2 as i32 + row.4 // + extra space top
				).of_size(
					(row.1 * 2) as u32,
					(row.2 * 2 + 1) as u32
				), Rgba([255u8, 0u8, 0u8, 255u8]));
				println!("drew sprite {}", sprite_name);
			}
			_                    => {
				/* SPRITE / IMAGE */
				if i != 0 {
					img.save(format!("output/{}", sprite_name)).expect("could not save image");

					sprite_name = &row.0;
					x_crop = row.1;
					y_crop = row.2;
					matrix30 = row.3;
					matrix31 = row.4;
					println!("opening: {}/{}", path_to_image, sprite_name);
					img = open(format!("{}/{}", path_to_image, sprite_name)).expect(&format!("error on line {}", i));
				}
			}
		};
	}
	img.save(format!("output/{}", sprite_name)).expect("could not save image");
	println!("saved sprite {}", sprite_name);
}

// fn draw_box(img :image::DynamicImage, row :&Row, colour :Rgba<u8>) {
// 	draw_hollow_rect_mut(&mut img, Rect::at(
// 		x_crop - matrix30 - row.2 as i32 + row.4 + 1, // + extra space left
// 		y_crop - matrix31 - row.3 as i32 + row.5  // + extra space top
// 	).of_size(
// 		(row.2 * 2) as u32,
// 		(row.3 * 2 + 1) as u32
// 	), colour);
// 	println!("drew sprite {}", sprite_name);
// }
/* 
	/* SPRITE / IMAGE */
	let mut sprite_name :&String = &records[0].0;
	let mut img :image::DynamicImage = open(sprite_name).expect("could not read the first image from csv file");

	/* COLOURS */
	let mut colour :Rgba<u8>;
	let opacity :u8 = 255;

	for (i, row) in records.iter().enumerate() {		
		println!("row: {:?}, itteration: {}", row, i);

		if row.0 != "".to_string() && i != 0 {
			img.save(format!("output/{}", sprite_name)).expect("could not save image");
			sprite_name = &row.0;
			println!("changed sprite_name to {}", sprite_name);
			img.save(format!("output/{}", sprite_name)).expect("could not save image");
			img = open(row.0.clone()).expect(&(format!("could not read image from csv file\nline {}(?)", i + 2)));
		};

		match row.1.as_str() {
			"coll" | "collision" => { colour = Rgba([0u8, 0u8, 255u8, opacity]) }
			"hurt"               => { colour = Rgba([0u8, 255u8, 0u8, opacity]) }
			"hit"                => { colour = Rgba([255u8, 0u8, 0u8, opacity]) }
			_                    => { panic!("missing box colur") }
		};

		draw_hollow_rect_mut(&mut img, Rect::at(
			x_crop - matrix30 - row.2 as i32 + row.4 + 1, // + extra space left
			y_crop - matrix31 - row.3 as i32 + row.5  // + extra space top
		).of_size(
			row.2 * 2,
			row.3 * 2 + 1
		), colour);
		println!("drew sprite {}", sprite_name);
	}
	img.save(format!("output/{}", sprite_name)).expect("could not save image");
	println!("saved sprite {}", sprite_name);
*/