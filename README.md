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
- NixOS (x86_64) (glibc) (coming version 3.0)
- Windows (x86_64)

If the regular Linux binary doesn't work, try the musl version

More platforms and architectures should be supported in the future such as MacOS, FreeBSD and aarch64. If your system or architecture is not supported, please create an issue where you write about it so I compile for your platform in the next version. You can also compile the code yourself if you have rust installed on your system.

# Compiling from source
Visual Oxide will work on any operative system and architecture that supports Rust's compiler

Install cargo, Rust's package manager, and open a terminal in the project's directory and run ``` cargo build -r ```
