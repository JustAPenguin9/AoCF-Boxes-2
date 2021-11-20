use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HeaderRow {
	pub image_dir: String,
	pub padding_top: u32,
	pub padding_left: u32,
	pub padding_bottom: u32,
	pub padding_right: u32,
}

#[derive(Debug, Deserialize)]
pub struct ImageRow {
	pub sprite_name: String,
	pub x_crop: i32,
	pub y_crop: i32,
	pub matrix30: i32,
	pub matrix31: i32,
}

#[derive(Debug, Deserialize)]
pub struct GifRow {
	// TODO: add gif support
	pub image_name: String,
	pub exposure: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct BoxRow {
	pub box_type: String,
	pub width: i32,
	pub height: i32,
	pub xoffset: i32,
	pub yoffset: i32,
}
