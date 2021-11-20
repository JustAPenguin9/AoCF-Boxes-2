use std::env;
use std::fs::create_dir;
use std::path::Path;

use ansi_term::Colour::{Blue, Green, Red};
use clap::{crate_version, App, Arg};
use csv::{ReaderBuilder, Trim};
use image::Rgba;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

mod create_image;
use create_image::create_image;

mod row_types;
use row_types::{BoxRow, HeaderRow, ImageRow};

fn main() {
	/* CLAP */
	let app = App::new("AoCF Boxes 2")
		.version(format!("v{}", crate_version!()).as_str())
		.author("JustAPenguin")
		.about("CLI app that draws the hit, hurt, and collision boxes for Touhou 15.5 Antinomy of Common Flowers")
		.arg(Arg::with_name("verbose")
			.help("Prints out the verbose output (-vv for extra verbosity)")
			.short("v")
			.long("verbose")
			.multiple(true))
		.arg(Arg::with_name("csv_file")
			.help("location to the csv file [REQUIRED]")
			.index(1)
			.required(true))
		.arg(Arg::with_name("output_dir")
			.help("Set the output directory. Default ./output/")
			.short("o")
			.long("outputDir")
			.takes_value(true))
		.get_matches();

	/* CREATE AN OUTPUT DIR IF NONE EXIST ALREADY */
	let output_dir = app.value_of("output_dir").unwrap_or("output");
	if !Path::new(output_dir).is_dir() {
		create_dir(output_dir).unwrap_or_else(|_| {
			println!("{}", Red.bold().paint("error:\tcould not create the output directory"));
			std::process::exit(101);
		});
		if app.is_present("verbose") {
			println!("output dir {} created", Blue.paint(output_dir));
		}
	}

	// create the reader
	let mut rdr = ReaderBuilder::new()
		.has_headers(false)
		.flexible(true)
		.comment(Some(b'#'))
		.trim(Trim::All)
		.from_path(app.value_of("csv_file").unwrap())
		.unwrap_or_else(|_| {
			println!(
				"{}",
				Red.bold().paint(
					"error:\tcould not create a reader based on the csv file \
					\n\tdid you misspell the file name?"
				)
			);
			std::process::exit(101);
		});

	let mut img_num = 0;
	let mut line_num = 2; // starts at 2 because of the header and first row

	// read the header
	let header: HeaderRow =
		rdr.records().next().unwrap().unwrap().deserialize(None).unwrap_or_else(|_| {
			println!(
				"{}",
				Red.bold().paint(
					"error:\trow 1 formatted incorrectly \
					\n\tshould be the path to the image folder followed by 4 positive whole numbers"
				)
			);
			std::process::exit(65);
		});
	match app.occurrences_of("verbose") {
		0 => (),
		1 => println!("succesfully read the header"),
		2 | _ => {
			println!("succesfully read the header");
			println!("{:#?}", &header);
		}
	}

	// read the first row / second line
	let mut image_row: ImageRow =
		rdr.records().next().unwrap().unwrap().deserialize(None).unwrap_or_else(|_| {
			println!(
				"{}",
				Red.bold().paint(
					"error:\trow 2 formatted incorrectly \
					\n\tshould be the full image name followed by 4 positive whole numbers"
				)
			);
			std::process::exit(65);
		});
	match app.occurrences_of("verbose") {
		0 => (),
		1 => println!("opening {}{}", &header.image_dir, Blue.paint(&image_row.sprite_name)),
		2 | _ => {
			println!("opening {}{}", &header.image_dir, Blue.paint(&image_row.sprite_name));
			println!("{:#?}", &image_row)
		}
	}
	let mut img = create_image(&header, &image_row).unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold().paint(format!(
				"error:\tcould not open the image {}{} on line {}",
				header.image_dir, image_row.sprite_name, line_num
			))
		);
		std::process::exit(65);
	});
	if app.is_present("verbose") {
		println!("opened {}{}", &header.image_dir, Blue.paint(&image_row.sprite_name))
	}

	while let Some(result) = rdr.records().next() {
		let record = result.unwrap();
		line_num += 1;

		match record[0].to_lowercase().as_str() {
			"coll" | "collision" => {
				let row: BoxRow = record.deserialize(None).unwrap_or_else(|_| {
					println!(
						"{}",
						Red.bold().paint(format!(
							"error:\trow {} formatted incorrectly \
							\n\tshould be the type of box followed by 4 positive whole numbers",
							line_num
						))
					);
					std::process::exit(65);
				});
				draw_hollow_rect_mut(
					&mut img,
					Rect::at(
						image_row.x_crop - image_row.matrix30 - row.width as i32
							+ row.xoffset + 1 + header.padding_left as i32,
						image_row.y_crop - image_row.matrix31 - row.height as i32
							+ row.yoffset + header.padding_top as i32,
					)
					.of_size((row.width * 2) as u32, (row.height * 2 + 1) as u32),
					Rgba([0u8, 0u8, 255u8, 255u8]), // blue
				);
				match app.occurrences_of("verbose") {
					0 => (),
					1 => println!("drew collision box on {}", Blue.paint(&image_row.sprite_name)),
					2 | _ => {
						println!("drew collision box on {}", Blue.paint(&image_row.sprite_name));
						println!("{:#?}", &row)
					}
				}
			}
			"hurt" => {
				let row: BoxRow = record.deserialize(None).unwrap_or_else(|_| {
					println!(
						"{}",
						Red.bold().paint(format!(
							"error:\trow {} formatted incorrectly \
							\n\tshould be the type of box followed by 4 positive whole numbers",
							line_num
						))
					);
					std::process::exit(65);
				});
				draw_hollow_rect_mut(
					&mut img,
					Rect::at(
						image_row.x_crop - image_row.matrix30 - row.width as i32
							+ row.xoffset + 1 + header.padding_left as i32,
						image_row.y_crop - image_row.matrix31 - row.height as i32
							+ row.yoffset + header.padding_top as i32,
					)
					.of_size((row.width * 2) as u32, (row.height * 2 + 1) as u32),
					Rgba([0u8, 255u8, 0u8, 255u8]), // green
				);
				match app.occurrences_of("verbose") {
					0 => (),
					1 => println!("drew hurt box on {}", Blue.paint(&image_row.sprite_name)),
					2 | _ => {
						println!("drew hurt box on {}", Blue.paint(&image_row.sprite_name));
						println!("{:#?}", &row)
					}
				}
			}
			"hit" => {
				let row: BoxRow = record.deserialize(None).unwrap_or_else(|_| {
					println!(
						"{}",
						Red.bold().paint(format!(
							"error:\trow {} formatted incorrectly \
							\n\tshould be the type of box followed by 4 positive whole numbers",
							line_num
						))
					);
					std::process::exit(65);
				});
				draw_hollow_rect_mut(
					&mut img,
					Rect::at(
						image_row.x_crop - image_row.matrix30 - row.width as i32
							+ row.xoffset + 1 + header.padding_left as i32,
						image_row.y_crop - image_row.matrix31 - row.height as i32
							+ row.yoffset + header.padding_top as i32,
					)
					.of_size((row.width * 2) as u32, (row.height * 2 + 1) as u32),
					Rgba([255u8, 0u8, 0u8, 255u8]), // red
				);
				match app.occurrences_of("verbose") {
					0 => (),
					1 => println!("drew hit box on {}", Blue.paint(&image_row.sprite_name)),
					2 | _ => {
						println!("drew hit box on {}", Blue.paint(&image_row.sprite_name));
						println!("{:#?}", &row)
					}
				}
			}
			"hit2" => {
				let row: BoxRow = record.deserialize(None).unwrap_or_else(|_| {
					println!(
						"{}",
						Red.bold().paint(format!(
							"error:\trow {} formatted incorrectly \
							\n\tshould be the type of box followed by 4 positive whole numbers",
							line_num
						))
					);
					std::process::exit(65);
				});
				draw_hollow_rect_mut(
					&mut img,
					Rect::at(
						image_row.x_crop - image_row.matrix30 - row.width as i32
							+ row.xoffset + 1 + header.padding_left as i32,
						image_row.y_crop - image_row.matrix31 - row.height as i32
							+ row.yoffset + header.padding_top as i32,
					)
					.of_size((row.width * 2) as u32, (row.height * 2 + 1) as u32),
					Rgba([255u8, 255u8, 0u8, 255u8]), // yellow
				);
				match app.occurrences_of("verbose") {
					0 => (),
					1 => println!("drew hit box on {}", Blue.paint(&image_row.sprite_name)),
					2 | _ => {
						println!("drew hit box on {}", Blue.paint(&image_row.sprite_name));
						println!("{:#?}", &row)
					}
				}
			}
			"gif" | "frame" => {
				// TODO:
				println!("gifs are not supported in this release")
			}
			_ => {
				// save the image
				img_num += 1;
				img.save(format!(
					"{}/{}-boxes{:02}.png",
					output_dir,
					&image_row.sprite_name[..&image_row.sprite_name.len() - 4],
					img_num
				))
				.unwrap_or_else(|_| {
					println!(
						"{}",
						Red.bold().paint(format!(
							"error:\tcould not save the image {}",
							&image_row.sprite_name
						))
					);
					std::process::exit(101);
				});
				if app.is_present("verbose") {
					println!(
						"saved image {} to the output dir",
						Blue.paint(&image_row.sprite_name)
					);
				}

				image_row = record.deserialize(None).unwrap_or_else(|_| {
					println!(
						"{}",
						Red.bold().paint(format!(
							"error:\trow {} formatted incorrectly \
							\n\tshould be the image name followed by 4 positive whole numbers",
							line_num
						))
					);
					std::process::exit(65);
				});
				match app.occurrences_of("verbose") {
					0 => (),
					1 => println!(
						"opening {}{}",
						&header.image_dir,
						Blue.paint(&image_row.sprite_name)
					),
					2 | _ => {
						println!(
							"opening {}{}",
							&header.image_dir,
							Blue.paint(&image_row.sprite_name)
						);
						println!("{:#?}", &image_row)
					}
				}
				img = create_image(&header, &image_row).unwrap_or_else(|_| {
					println!(
						"{}",
						Red.bold().paint(format!(
							"error:\tcould not open the image {}{} on line {}",
							header.image_dir, image_row.sprite_name, line_num
						))
					);
					std::process::exit(65);
				});
				if app.is_present("verbose") {
					println!("opened {}{}", &header.image_dir, Blue.paint(&image_row.sprite_name))
				}
			}
		};
	}

	// save the final image
	img_num += 1;
	img.save(format!(
		"{}/{}-boxes{:02}.png",
		output_dir,
		&image_row.sprite_name[..&image_row.sprite_name.len() - 4],
		img_num
	))
	.unwrap_or_else(|_| {
		println!(
			"{}",
			Red.bold()
				.paint(format!("error:\tcould not save the image {}", &image_row.sprite_name))
		);
		std::process::exit(101);
	});
	if app.is_present("verbose") {
		println!("saved image {} to the output dir", Blue.paint(&image_row.sprite_name));
	}

	println!("{}", Green.bold().paint("done!"))
}
