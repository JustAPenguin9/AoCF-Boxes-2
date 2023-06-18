use std::path::PathBuf;

use clap::{crate_version, value_parser, Arg, ArgAction, ArgMatches, Command};

pub fn get_matches() -> ArgMatches {
	Command::new("AoCF Boxes 2")
		.version(crate_version!())
		.author("JustAPenguin")
		.about("CLI app that draws the hit, hurt, and collision boxes for Touhou 15.5 Antinomy of Common Flowers")
		.arg(Arg::new("input_file")
			.help("Path to the .ron file")
			.value_parser(value_parser!(PathBuf))
			.index(1)
			.required(true))
		.arg(Arg::new("gif")
			.help("Export a gif")
			.short('g')
			.long("gif")
			.action(ArgAction::SetTrue))
		.arg(Arg::new("no_sprites")
			.help("Draw the boxes without the sprites included")
			.short('n')
			.long("noSprites")
			.action(ArgAction::SetTrue))
		.arg(Arg::new("output_dir")
			.help("Set the output directory")
			.short('o')
			.long("outputDir")
			.default_value("output")
			.num_args(1))
		.get_matches()
}