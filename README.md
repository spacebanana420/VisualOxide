# Visual Oxide
A CLI image processor made in Rust that will be capable of processing images (color, size, crop, effects, etc) and encoding them

Visual Oxide uses the [image crate](https://crates.io/crates/image) library for most of its capabilities 

The program is currently on a very early stage

# Features (as of now)
- Image resizing and cropping
- Image generation and creation of pixel patterns
- ASCII art generation from an image with custom width
- Decoding support for all formats supported by the image library
- Preservation of original bit depth
- PNG encoding
- Image to ico conversion

# Compiling from source
Visual Oxide will work on any operative system that supports Rust's compiler and package manager

Install cargo, Rust's package manager, and open a terminal in the project's directory and run ``` cargo build -r ```
