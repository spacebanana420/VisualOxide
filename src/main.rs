use image::imageops;
use image::{GenericImageView, Pixel, GenericImage, ColorType, ImageBuffer, DynamicImage, Rgba};
use image::io::Reader;
use std::io;
use std::fs;

fn main() {
    println!("1. Resize     2. Crop     3. Image to ASCII");
    println!("Choose an operation");
    let operation = answer_to_u8();
    let currentdir = fs::read_dir("./").unwrap();
    for dir in currentdir {
        println!("{}", dir.unwrap().path().display());
    }
    println!("Input image file");
    let inputimg = answer();

    match operation {
        1=> resizeimg(&inputimg),
        2=> cropimg(&inputimg),
        3=> image_to_ascii(&inputimg),
        _=> {
            println!("Please choose one of the available operation");
        }
    }
}

fn answer() -> String {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("aaa");
    return userinput;
}

fn answer_to_u8() -> u8 {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("aaa");
    let userinput:u8 = userinput.trim().parse().expect("Needs to be a number!");
    return userinput;
}

fn answer_to_u32() -> u32 {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput).expect("aaa");
    let userinput:u32 = userinput.trim().parse().expect("Needs to be a number!");
    return userinput;
}

fn open_image(filename:&str) -> image::DynamicImage {
    let inputimage = image::open(filename.trim()).expect("Image file not found in specified directory");
    return inputimage;
}
//fn help
//fn batchprocess
//fn transcode
//fn getpixel
//fn generateimage
//fn checkalpha
//fn print
//imagetoascii
//duplicate horizontal ascii characters for compensat
//remove extension from file readline
fn image_to_ascii(imgname:&str) {
    println!("Note: ASCII art generation only works if the image is 8bits/channel   ");
    let mut ascii_output = String::new();
    println!("");
    let img = open_image(&imgname);
    let (width, height) = img.dimensions();
    println!("Choose the ASCII output width");
    let resizewidth = answer_to_u32();
    let img = image::DynamicImage::resize(&img, resizewidth, height*resizewidth/width, imageops::Nearest);
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as f32; let g = pixel[1] as f32; let b = pixel[2] as f32; let alpha = pixel[3] as f32;
            //println!("r={}", r.to_string());
            if alpha == 0.0 {ascii_output.push_str("  ");}//print!(" ");}
            else {
                let brightness:f32 = (r + g + b) / 3.0;
                match brightness {
                    brightness if brightness >= 240.0 => ascii_output.push_str("##"),//print!("."),
                    brightness if brightness >= 220.0 => ascii_output.push_str("WW"),//print!("."),
                    brightness if brightness >= 200.0 => ascii_output.push_str("00"),//print!("*"),
                    brightness if brightness >= 180.0 => ascii_output.push_str("&&"),//print!("*"),
                    brightness if brightness >= 160.0 => ascii_output.push_str("çç"),//print!("o"),
                    brightness if brightness >= 140.0 => ascii_output.push_str("$$"),//print!("o"),
                    brightness if brightness >= 120.0 => ascii_output.push_str("%%"),//print!("&"),
                    brightness if brightness >= 100.0 => ascii_output.push_str("11"),//print!("&"),
                    brightness if brightness >= 80.0 => ascii_output.push_str("ww"),//print!("$"),
                    brightness if brightness >= 60.0 => ascii_output.push_str("++"),//print!("$"),
                    brightness if brightness >= 40.0 => ascii_output.push_str("**"),//print!("+"),
                    brightness if brightness >= 20.0 => ascii_output.push_str("''"),//print!("+"),
                    _=> ascii_output.push_str("--"),//print!("#"),
                }
            }

        }
        ascii_output.push('\n');//println!("");
    }
    println!("{ascii_output}");
    fs::write("ascii_output.txt", ascii_output);
}

fn icogen(imgname:&str) {
   let img = open_image(&imgname);
   img.save("{imgname}_icon.ico");
}

fn testfun(imgname:&str) {
    //let img = image::open(imgname.trim()).expect("ag");
    //let pixeltest = img.get_pixel(40, 40);
    let mut switch:u8 = 1;
    let mut square = DynamicImage::new_rgb8(400, 400);
    //DynamicImage::put_pixel(&mut square, 350, 350, image::Rgba([255, 255, 255, 255]));
    //let mut square = ImageBuffer::new(400, 400);
    for x in 0..square.width() {
        for y in 0..square.height() {
            if switch <= 4 {
                DynamicImage::put_pixel(&mut square, x, y, image::Rgba([255, 255, 255, 255]));
            }
            else {
                DynamicImage::put_pixel(&mut square, x, y, image::Rgba([0, 0, 0, 255]));
            }
            if switch != 8 {switch+=1;} else {switch=1;}
        }
    }
    for (x, y, mut pixel) in square.pixels() {
        //square.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
        //DynamicImage::put_pixel(&mut square, x, y, image::Rgba([255, 255, 255, 255]));
        //pixel = image::Rgba([255, 255, 255, 255]);
        //square.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
    }
    square.save("square.png").unwrap();
    //println!("{}", pixeltest as u8);
}

/*fn imageinfo(imgname:&str) {
    let img = open_image(&imgname);
    let (w, h) = img.dimensions();
}*/

fn resizeimg(imgname:&str) {
    let img = open_image(&imgname);
    let (w, h) = img.dimensions();

    println!("1. factor  2. manual  3. auto aspect");
    println!("Choose a mode");
    let mode = answer_to_u8();

    let mut rw:u32 = 0; let mut rh:u32 = 0;
    match mode {
        1=> {
            println!("Input the image scaling factor");
            let scalefactor = answer_to_u32();
            rw = w * scalefactor; rh = h * scalefactor;
        },
        2=> {
            println!("Input width");
            rw = answer_to_u32();
            println!("Input height");
            rh = answer_to_u32();
        },
        3=> {
            println!("1. width   2. height");
            println!("Choose one to scale, the other will be automatic");
            let mode = answer_to_u8();
            match mode {
                1=> {
                    println!("Input width");
                    let tempdimension = answer_to_u32();
                    rw = tempdimension;
                    rh = h * (rw / w);
                },
                2=> {
                    println!("Input height");
                    let tempdimension = answer_to_u32();
                    rh = tempdimension;
                    rw = w * (rh / h);
                },
                _=> {
                    println!("You need to choose which to scale!");
                }
            }
        },
        _=> {
            println!("You need to select a scaling mode!");
        },
    }

    let img_resized = image::DynamicImage::resize(&img, rw, rh, imageops::CatmullRom);
    img_resized.save("{imgname}_scaled.png").unwrap();
    //let imgenc = PngEncoder::write_image(imgenc, &img_resized, w, h, ColorType::Rgb16);
    //image::save_buffer("test.png", &img_resized, w/2, w/2, ExtendedColorType::Bgr16).unwrap();
}

fn cropimg(imgname:&str) {
    let mut img = open_image(&imgname);
    let (w, h) = img.dimensions();
    println!("1. Manual     2. Center (preset)"); println!("Choose the cropping mode");
    let cropmode = answer_to_u8();
    match cropmode {
        1=> {
            println!("Choose the starting point (from left)");
            let startx = answer_to_u32();
            println!("Choose the starting point (from top)");
            let starty = answer_to_u32();
            println!("Choose the crop width");
            let cropwidth = answer_to_u32();
            println!("Choose the crop height");
            let cropheight = answer_to_u32();
            let img = image::DynamicImage::crop(&mut img, startx, starty, cropwidth, cropheight);
        },
        2=> { //unfinished
            let img = image::DynamicImage::crop(&mut img, w/4, h/4, w/2, h/2);
        },
        _=> {
            println!("Choose a cropping mode!");
        }
    }
    img.save("{imgname}_cropped.png").unwrap();
}
