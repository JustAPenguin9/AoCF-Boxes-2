use std::env;
use std::fs::create_dir;
use std::path::Path;

use ansi_term::Colour::{Blue, Green, Red};
use clap::{crate_version, App, Arg};
use csv::{ReaderBuilder, Trim};
use image::{open, GenericImageView, Rgba};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

type Row = (String, i32, i32, i32, i32);

fn main() {
	/* CLAP */
	let matches = App::new("AoCF Boxes 2")
		.version(format!("v{}", crate_version!()).as_str())
		.author("JustAPenguin")
		.about("CLI app that draws the hit, hurt, and collision boxes for Touhou 15.5 Antinomy of Common Flowers")
		.arg(Arg::with_name("verbose")
			.short("v")
			.long("verbose")
			.help("prints out more to aid in debugging"))
		.arg(Arg::with_name("csv_file")
			.help("location to the csv file [REQUIRED]")
			.index(1)
		.required(true))
		.get_matches();

	/* CREATE AN OUTPUT DIR IF NONE EXIST ALREADY */
	if !Path::new("output").is_dir() {
		create_dir("output").unwrap_or_else(|_| {
			println!("{}", Red.bold().paint("error: could not create an output dir"));
			std::process::exit(101);
		});
		if matches.is_present("verbose") {
			println!("output dir created")
		}
	}

	/* CREATE THE READER */
	let file_path = matches.value_of("csv_file").unwrap();
	let mut rdr = ReaderBuilder::new()
		.has_headers(true)
		.flexible(true)
		.comment(Some(b'#'))
		.trim(Trim::All)
		.from_path(file_path)
		.unwrap_or_else(|_| {
			println!(
				"{}",
				Red.bold().paint("error: could not create a reader based on the csv file")
			);
			std::process::exit(101);
		});
	let records = rdr.deserialize().collect::<Result<Vec<Row>, csv::Error>>()
		.unwrap_or_else( |_| {
			println!("{}", Red.bold().paint("error: error deserializing the csv file
       maybe the data wasn't included correctly? the header should have 4 columbs and each row shold have 6 columbs"));
			std::process::exit(65);
		});

	/* GET THE DATA FROM THE HEADER */
	let header = rdr.headers().unwrap();
	let path_to_image: String = header[0].parse().unwrap();
	// FIXME: errors with a panic if data is missing
	let padding_top: u32 = header[1].parse().unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold()
				.paint("error: the top padding in the header of the csv file is not a number")
		);
		std::process::exit(65);
	});
	let padding_left: u32 = header[2].parse().unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold()
				.paint("error: the left padding in the header of the csv file is not a number")
		);
		std::process::exit(65);
	});
	let padding_bottom: u32 = header[3].parse().unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold()
				.paint("error: the bottom padding in the header of the csv file is not a number")
		);
		std::process::exit(65);
	});
	let padding_right: u32 = header[4].parse().unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold()
				.paint("error: the right padding in the header of the csv file is not a number")
		);
		std::process::exit(65);
	});

	/* FIRST IMAGE */
	let mut sprite_name: &String = &records[0].0;
	let mut x_crop: i32 = records[0].1;
	let mut y_crop: i32 = records[0].2;
	let mut matrix30: i32 = records[0].3;
	let mut matrix31: i32 = records[0].4;

	if matches.is_present("verbose") {
		println!("opening {}{}", path_to_image, Blue.paint(sprite_name));
	}
	let mut sprite: image::DynamicImage = open(format!("{}{}", path_to_image, sprite_name))
		.unwrap_or_else(|_| {
			println!(
				"{}",
				Red.bold().paint("error: could not open the first image in the csv file")
			);
			std::process::exit(65);
		});
	let img_width = sprite.width() + padding_left + padding_right;
	let img_height = sprite.height() + padding_top + padding_bottom;
	let mut img = image::DynamicImage::new_rgba8(img_width, img_height);
	image::imageops::overlay(&mut img, &sprite, padding_left, padding_top);
	if matches.is_present("verbose") {
		println!("opened {}{}", path_to_image, Blue.paint(sprite_name));
	}

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
				if matches.is_present("verbose") {
					println!("drew collision box on {}", Blue.paint(sprite_name));
				}
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
				if matches.is_present("verbose") {
					println!("drew hurt box on {}", Blue.paint(sprite_name));
				}
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
				if matches.is_present("verbose") {
					println!("drew hit box on {}", Blue.paint(sprite_name));
				}
			}
			_ => {
				/* SPRITE / IMAGE */
				if i != 0 {
					img.save(format!("output/{}", sprite_name)).unwrap_or_else(|_| {
						println!(
							"{}",
							Red.bold()
								.paint(format!("error: could not save the image {}", sprite_name))
						);
						std::process::exit(101);
					});
					if matches.is_present("verbose") {
						println!("saved image {} to output dir", Blue.paint(sprite_name));
					}

					sprite_name = &row.0;
					x_crop = row.1;
					y_crop = row.2;
					matrix30 = row.3;
					matrix31 = row.4;

					if matches.is_present("verbose") {
						println!("opening {}{}", path_to_image, Blue.paint(sprite_name));
					}
					sprite =
						open(format!("{}{}", path_to_image, sprite_name)).unwrap_or_else(|_| {
							println!(
								"{}",
								Red.bold().paint(format!(
									"error: could not open the image on line {}",
									i + 2
								))
							);
							std::process::exit(65);
						});
					img = image::DynamicImage::new_rgba8(img_width, img_height);
					image::imageops::overlay(&mut img, &sprite, padding_left, padding_top);
					if matches.is_present("verbose") {
						println!("opened {}{}", path_to_image, Blue.paint(sprite_name));
					}
				}
			}
		};
	}
	img.save(format!("output/{}", sprite_name)).unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold().paint(format!("error: could not save the image {}", sprite_name))
		);
		std::process::exit(101);
	});
	if matches.is_present("verbose") {
		println!("saved image {} to output dir", Blue.paint(sprite_name));
	}

	println!("{}", Green.bold().paint("done!"))
}
