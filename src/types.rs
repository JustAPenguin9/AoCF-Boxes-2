use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Move {
	pub directory: String,
	pub name: String,
	pub speed: Option<i32>,
	pub padding_tlbr: (u32, u32, u32, u32),
	pub images: Vec<Image>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Image {
	pub file: String,
	pub exposure: Option<u32>,
	pub crop_xy: (i32, i32),
	pub matrix_3031: (i32, i32),
	// TODO: overlays
	// overlays: Option<Vec<Overlay>>,
	pub boxes: Vec<Box>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Overlay {
	file: String,
	offset_xy: (i32, i32),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Box {
	pub colour: Colour,
	pub size_wh: (u32, u32),
	pub offset_xy: (i32, i32),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Colour {
	Hit,
	Hit2,
	Hurt,
	Collision,
	Hex(u32),
}
