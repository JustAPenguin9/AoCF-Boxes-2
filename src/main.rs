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

type Row = (String, String, u32, u32, i32, i32);
/////////   image   colour  w    h    x    y /////////


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
	let records = rdr.deserialize().collect::<Result<Vec<Row>, csv::Error>>().unwrap();

	/* GET THE DATA FROM THE HEADER */
	let header = rdr.headers().unwrap();
	let x_crop :i32 = header[0].parse().unwrap();
	let y_crop :i32 = header[0].parse().unwrap();
	let matrix30 :i32 = header[2].parse().unwrap();
	let matrix31 :i32 = header[3].parse().unwrap();

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
			_                    => { panic!("missing propper colour") }
		};

		draw_hollow_rect_mut(&mut img, Rect::at(
			x_crop - matrix30 - row.2 as i32 + row.4 + 1, // + extra space left
			y_crop - matrix31 - row.3 as i32 + row.5 - 1  // + extra space top
		).of_size(
			row.2 * 2,
			row.3 * 2 + 1
		), colour);
		println!("drew sprite {}", sprite_name);
	}
	img.save(format!("output/{}", sprite_name)).expect("could not save image");
	println!("saved sprite {}", sprite_name);
}