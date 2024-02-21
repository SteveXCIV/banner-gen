![banner image](./banner.webp)

# banner-gen

A super simple tool to generate banner images.

This repository is currently in a **very** rough state.
It's what my Software Dev. professor would have referred to as a "garage program",
i.e. something I cooked up in my garage.

I plan on writing a blog post detailing how I made this, and revisiting it to
turn it into a proper piece of software at a later date.
Please consider watching this space for major updates.

## Build Instructions

1. Clone the repo `git clone https://github.com/SteveXCIV/banner-gen.git`
2. `cd banner-gen`
3. `cargo build`

## Running the Program

`cargo run` to run the program.
A GUI window will open and immediately generate a banner.
There are a couple of keyboard controls:

- `space` will generate a new banner
- `S` will save the current banner with a unique-ish name in the current directory

The file name is unique-ish because it's just `banner_` plus the current UNIX
time in milliseconds, as a PNG file.
