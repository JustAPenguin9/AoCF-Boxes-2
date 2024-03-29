# AoCF-Boxes-2

AoCF boxes 2 draws the hit, hurt and collision boxes for *Touhou Hyouibana ~ Antinomy of Common Flowers* through the use of multiple file formats. AoCF boxes 2 is meant to be a replacement to the original [AoCF boxes](https://github.com/JustAPenguin9/AoCF-Boxes). Boxes 2 does everything that boxes 1 does in a quicker and cleaner fashion... so please don't use boxes 1. Boxes is largely built off of the work done by @fearnagae on discord and [MathyFurret](https://github.com/MathyFurret/th155-decomp).

### Downloading and running
To use AoCF boxes 2 download the most most recent [release](https://github.com/JustAPenguin9/AoCF-Boxes-2/releases) on to your computer.
To run AoCF boxes 2 type into your terminal:
```shell
aocf-boxes-2 [path to your .ron file]
```

### Current format for the .ron file
**Other file formats such as JSON can be used and follow a similar structure** 

You can view an example [here](examples/miko.ron).

- Trailing commas are allowed
- Comments can be done with `//` and `/*` `*/`


(These aren't the real numbers for Reimu j5a btw)
```rs
// this is only needed if you don't want to wrap every optional field in "Some(#)"
#![enable(implicit_some)]

(
	// directory where all the sprites are stored
	directory: "reimu/",
	// name given to the images outputted
	name: "reimu-j5a",
	// (top, left, bottom, right)
	padding_tlbr: (10, 0, 0, 20),
	// list of the frames
	images: [
		(
			// sprite in the directory named above
			file: "frame1.png",
			// the exposure of an image when encoded as a gif (optional)
			exposure: 3,
			// (x, y)
			crop_xy: (0, 10),
			// (matrix30, matrix31)
			matrix_3031: (10, 20),
			// list of the boxes to draw on the sprite
			boxes: [
				(
					// the colour can be "Hit", "Hit2", "Hurt", "Collision", or a custom value with "Hex(0xFF6A10)"
					colour: Collision,
					// (width, height)
					size_wh: (15, 15),
					// (x, y)
					offset_xy: (30, 30),
				),
				(
					colour: Hurt,
					size_wh: (30, 30),
					offset_xy: (50, 50),
				),
			]
		),
		(
			file: "frame2.png",
			exposure: 9,
			crop_xy: (0, 10),
			matrix_3031: (20, 15),
			boxes: [
				(
					colour: Collision,
					size_wh: (15, 15),
					offset_xy: (40, 30),
				),
				(
					colour: Hurt,
					size_wh: (30, 30),
					offset_xy: (60, 50),
				),
				(
					colour: Hit2,
					size_wh: (50, 60),
					offset_xy: (55, 20)
				)
			]
		)
	]
)
```
