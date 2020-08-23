# dither
This is a program to dither an image. I needed a tool that would let me supply a
custom grayscale palette, and apply dithering accordingly, and I needed practice
with Rust, so... ta-da. *(**NOTE**: This program works on RGB colors, not just grayscale)*. The dithering code is far from elegant, but it works
well enough.

For now, this code implements the Floyd-Steinberg algorithm.

# "License"
Feel free to use this at your own risk and do whatever you want with the code.

# Building
This is written in Rust (and uses Cargo), so you'll need to install them. See
[this](https://doc.rust-lang.org/cargo/getting-started/installation.html).
You'll also need to clone this project :) Once cloned, navigate to the project
folder and run:
```
cargo build --release
```

The binary will be found at `target/release/dither`.

# Executing
Supply an image and a palette:

        ./dither path/to/image "#000" "#AAA" "#DDD" "#FFF"

Colors are #RGB or #RRGGBB format. Output is in `output.png`

# Problems/Ideas
* It's slow; come up with a better method of traversing the image and matching palette colors.
* Needs to use science/magic to better compare colors.
* It might be cool to add a feature to find the best palette of size N ?
