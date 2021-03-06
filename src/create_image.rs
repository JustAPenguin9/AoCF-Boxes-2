use clap::ArgMatches;
use image::{open, DynamicImage, GenericImageView};

use crate::row_types::{HeaderRow, ImageRow};

pub fn create_image(
	header: &HeaderRow,
	image_row: &ImageRow,
	app: &ArgMatches,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
	let sprite: image::DynamicImage =
		open(format!("{}{}", header.image_dir, image_row.sprite_name))?;
	let mut img = image::DynamicImage::new_rgba8(
		sprite.width() + header.padding_left + header.padding_right,
		sprite.height() + header.padding_top + header.padding_bottom,
	);

	// skip drawing sprite if noSprites flag is used
	if !app.is_present("no_sprites") {
		image::imageops::overlay(&mut img, &sprite, header.padding_left, header.padding_top);
	}

	return Ok(img);
}
