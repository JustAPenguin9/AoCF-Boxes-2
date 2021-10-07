# AoCF-Boxes-2

AoCF boxes 2 draws the hit, hurt and collision boxes for *Touhou Hyouibana ~ Antinomy of Common Flowers* through the use of a .csv file. AoCF boxes 2 is meant to be a replacement to the original [AoCF boxes](https://github.com/JustAPenguin9/AoCF-Boxes). Boxes 2 does everything that boxes 1 does in a quicker and cleaner fashion.

### Downloading and running
To use AoCF boxes 2 download the most most recient [release](https://github.com/JustAPenguin9/AoCF-Boxes-2/releases) on to your computer.
To run AoCF boxes 2 type into your terminal:
```shell
aocf-boxes-2 [path to your .csv file]
```

### Current format for the .csv file
```
path to image folder (including the "/" at the end), padding top, padding left, padding bottom, padding right 
image, x crop, y crop, matrix30, matrix31
[coll OR hurt OR hit], width,  height, x offset, y offset
image, x crop, y crop, matrix30, matrix31
[coll OR hurt OR hit], width,  height, x offset, y offset
cont...
```

#### Available flags
The available flags when running are...<br>
-h, --help: Prints help information<br>
-V, --version: Prints version information<br>
-v, --verbose: Prints out more to aid in debugging<br>

###### i swear its actually good this time
