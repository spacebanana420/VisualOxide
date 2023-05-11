# Visual Oxide
A CLI image processor made in Rust that will be capable of processing images (color, size, crop, effects, etc) and encoding them

Visual Oxide uses the [image crate](https://crates.io/crates/image) library for most of its capabilities 

The program is currently on a very early stage

# Features (as of now)
- ASCII art generation from images
- Image resizing and cropping
- Image generation and creation of pixel patterns (not implemented yet)
- Contrast adjustment
- Preservation of original bit depth
- Multiple format support
- Image to ico conversion

# Download
You can download the latest versions of Visual Oxide [here](https://github.com/spacebanana420/VisualOxide/releases)

Visual Oxide is compiled for the following systems:
- Linux (x86_64) (both glibc and musl)
- Windows (x86_64)

If the regular Linux binary doesn't work, for example on NixOS, try the musl version

Cross-compiling isn't my strong suit, so for now I only provide binaries for x86 Linux and Windows, but this doesn't mean that my program doesn't work on other systems, because it does. Unfortunately, you will have to compile from source.

# Compiling from source
Visual Oxide will work on any operative system and architecture that supports Rust's compiler

Install cargo, Rust's package manager, and open a terminal in the project's directory and run ``` cargo build -r ```
