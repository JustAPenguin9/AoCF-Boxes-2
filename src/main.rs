use std::env;
use std::fs::create_dir;
use std::path::Path;

use ansi_term::Colour::{Blue, Green, Red};
use clap::{crate_version, App, Arg};
use csv::{ReaderBuilder, Trim};
use image::{open, DynamicImage, GenericImageView, Rgba};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

type Header = (String, u32, u32, u32, u32);
type Row = (String, i32, i32, i32, i32);

fn main() {
	/* CLAP */
	let matches = App::new("AoCF Boxes 2")
		.version(format!("v{}", crate_version!()).as_str())
		.author("JustAPenguin")
		.about("CLI app that draws the hit, hurt, and collision boxes for Touhou 15.5 Antinomy of Common Flowers")
		.arg(Arg::with_name("verbose")
			.help("Prints out what the program is doing")
			.short("v")
			.long("verbose"))
		.arg(Arg::with_name("csv_file")
			.help("location to the csv file [REQUIRED]")
			.index(1)
			.required(true))
		.arg(Arg::with_name("gif")
			.help("will create a gif out of images *soon*")
			.short("g")
			.long("gif"))
		.get_matches();

	/* CREATE AN OUTPUT DIR IF NONE EXIST ALREADY */
	if !Path::new("output").is_dir() {
		create_dir("output").unwrap_or_else(|_| {
			println!("{}", Red.bold().paint("error:\tcould not create an output dir"));
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
				Red.bold().paint(
					"error:\tcould not create a reader based on the csv file
\tdid you misspell the file name?"
				)
			);
			std::process::exit(101);
		});

	// header stuff
	let image_dir: String;
	let padding_top: u32;
	let padding_left: u32;
	let padding_bottom: u32;
	let padding_right: u32;

	{
		// headers is put in it own scope because rdr is used agin later on
		let headers = rdr.headers().unwrap();
		// deserialise the header, first line
		let header: Header = headers.deserialize(None).unwrap_or_else(|_| {
			println!(
				"{}",
				Red.bold().paint(
					"error:\trow 1 formatted incorrectly
\tshould be the path to the image folder followed by 4 positive whole numbers"
				)
			);
			std::process::exit(65);
		});
		image_dir = header.0;
		padding_top = header.1;
		padding_left = header.2;
		padding_bottom = header.3;
		padding_right = header.4;
		if matches.is_present("verbose") {
			println!("succesfully read the header");
		}
	} // headers is dropped allowing the use of rdr later on

	// image stuff
	let mut sprite_name: String;
	let mut x_crop: i32;
	let mut y_crop: i32;
	let mut matrix30: i32;
	let mut matrix31: i32;
	let img_width: u32;
	let img_height: u32;
	let mut img: DynamicImage;

	// deserialise first row, second line
	let row: Row = rdr.records().next().unwrap().unwrap().deserialize(None).unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold().paint(
				"error:\trow 2 formatted incorrectly
\tshould be the full image name followed by 4 positive whole numbers"
			)
		);
		std::process::exit(65);
	});
	sprite_name = row.0;
	x_crop = row.1;
	y_crop = row.2;
	matrix30 = row.3;
	matrix31 = row.4;

	if matches.is_present("verbose") {
		println!("opening {}{}", image_dir, Blue.paint(&sprite_name));
	}
	let sprite: image::DynamicImage =
		open(format!("{}{}", image_dir, sprite_name)).unwrap_or_else(|_| {
			println!(
				"{}",
				Red.bold().paint("error:\tcould not open the first image in the csv file")
			);
			std::process::exit(65);
		});
	img_width = sprite.width() + padding_left + padding_right;
	img_height = sprite.height() + padding_top + padding_bottom;
	img = image::DynamicImage::new_rgba8(img_width, img_height);
	image::imageops::overlay(&mut img, &sprite, padding_left, padding_top);
	if matches.is_present("verbose") {
		println!("opened {}{}", image_dir, Blue.paint(&sprite_name));
	}

	let mut line_num = 2; // the first 2 lines where already done (header and first image)
	while let Some(result) = rdr.records().next() {
		let record = result.unwrap();
		line_num += 1; // increment line number by 1 for the error messages

		if &record[0] == "GIF" && matches.is_present("gif") {
			println!("creating gifs is not yet available")
		// TODO: add gifs
		} else {
			let row: Row = record.deserialize(None).unwrap_or_else(|_| {
				println!(
					"{}",
					Red.bold().paint(format!(
						"error:\trow {} formatted incorrectly
\tshould be either the image name or type of box followed by 4 positive numbers",
						line_num
					))
				);
				std::process::exit(65);
			});
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
						println!("drew collision box on {}", Blue.paint(&sprite_name));
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
						println!("drew hurt box on {}", Blue.paint(&sprite_name));
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
						println!("drew hit box on {}", Blue.paint(&sprite_name));
					}
				}
				_ => {
					/* SPRITE / IMAGE */
					img.save(format!("output/{}", sprite_name)).unwrap_or_else(|_| {
						println!(
							"{}",
							Red.bold()
								.paint(format!("error:\tcould not save the image {}", sprite_name))
						);
						std::process::exit(101);
					});
					if matches.is_present("verbose") {
						println!("saved image {} to output dir", Blue.paint(&sprite_name));
					}
					sprite_name = row.0;
					x_crop = row.1;
					y_crop = row.2;
					matrix30 = row.3;
					matrix31 = row.4;
					if matches.is_present("verbose") {
						println!("opening {}{}", image_dir, Blue.paint(&sprite_name));
					}
					let sprite =
						open(format!("{}{}", image_dir, sprite_name)).unwrap_or_else(|_| {
							println!(
								"{}",
								Red.bold().paint(format!(
									"error:\tcould not open the image on line {}",
									line_num
								))
							);
							std::process::exit(65);
						});
					img = image::DynamicImage::new_rgba8(img_width, img_height);
					image::imageops::overlay(&mut img, &sprite, padding_left, padding_top);
					if matches.is_present("verbose") {
						println!("opened {}{}", image_dir, Blue.paint(&sprite_name));
					}
				}
			}
		}
	}
	img.save(format!("output/{}", sprite_name)).unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold().paint(format!("error:\tcould not save the image {}", sprite_name))
		);
		std::process::exit(101);
	});
	if matches.is_present("verbose") {
		println!("saved image {} to output dir", Blue.paint(&sprite_name));
	}

	println!("{}", Green.bold().paint("done!"))
}
