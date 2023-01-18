use image::imageops;
use image::{GenericImageView, GenericImage, DynamicImage};
use std::fs;
use crate::userinput;

//fn help
//fn batchprocess
//fn generateimage
//fn checkalpha

fn open_image(filename:&str) -> image::DynamicImage {
    let inputimage = image::open(filename.trim()).expect("Image file not found in specified directory");
    return inputimage;
}

pub fn image_to_ascii(imgname:&str) {
    println!("Note: ASCII art generation only works if the image is 8bits/channel");
    let mut ascii_output = String::new();
    println!("");
    let img = open_image(&imgname);
    let (width, height) = img.dimensions();
    println!("Choose the ASCII output width");
    let resizewidth = userinput::answer_to_u32();
    let img = image::DynamicImage::resize(&img, resizewidth, height*resizewidth/width, imageops::Nearest);
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as f32; let g = pixel[1] as f32; let b = pixel[2] as f32; let alpha = pixel[3] as f32;
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
                    _=> ascii_output.push_str("--"),
                }
            }

        }
        ascii_output.push('\n');//println!("");
    }
    println!("{ascii_output}");
    let mut exportname = userinput::remove_extension(&imgname);
    String::push_str(&mut exportname, "_ascii.txt");
    fs::write(exportname, ascii_output).expect("Could not write the ASCII output");
}

pub fn icogen(imgname:&str) {
   let img = open_image(&imgname);
   let mut exportname = userinput::remove_extension(&imgname);
   String::push_str(&mut exportname, ".ico");
   img.save(exportname).expect("Could not save the ico image");
}

/*
fn testfunction(imgname:&str) {
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
}*/

/*fn imageinfo(imgname:&str) {
    let img = open_image(&imgname);
    let (w, h) = img.dimensions();
}*/

pub fn resizeimg(imgname:&str) {
    let img = open_image(&imgname);
    let (w, h) = img.dimensions();

    println!("1. factor  2. manual  3. auto aspect");
    println!("Choose a mode");
    let mode = userinput::answer_to_u8();

    let mut rw:u32 = 0; let mut rh:u32 = 0;
    match mode {
        1=> {
            println!("Input the image scaling factor");
            let scalefactor = userinput::answer_to_u32();
            rw = w * scalefactor; rh = h * scalefactor;
        },
        2=> {
            println!("Input width");
            rw = userinput::answer_to_u32();
            println!("Input height");
            rh = userinput::answer_to_u32();
        },
        3=> {
            println!("1. width    2. height");
            println!("Choose one to scale, the other will be automatic");
            let mode = userinput::answer_to_u8();
            match mode {
                1=> {
                    println!("Input width");
                    let tempdimension = userinput::answer_to_u32();
                    rw = tempdimension;
                    rh = h * (rw / w);
                },
                2=> {
                    println!("Input height");
                    let tempdimension = userinput::answer_to_u32();
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
    println!("1. Linear    2. Cubic    3. Nearest");
    println!("Choose a pixel interpolation filter");
    let filterans = userinput::answer_to_u8();
    let mut pixelfilter = imageops::CatmullRom;
    match filterans {
        1=> {
            pixelfilter = imageops::Triangle;
        },
        2=> {
            pixelfilter = imageops::CatmullRom;
        },
        3=> {
            pixelfilter = imageops::Nearest;
        },
        _=> println!("You need to choose a pixel filter!")
    }
    let img_resized = image::DynamicImage::resize(&img, rw, rh, pixelfilter);

    let mut exportname = userinput::remove_extension(&imgname);
    String::push_str(&mut exportname, "_scaled.png");
    img_resized.save(exportname).unwrap();
    //let imgenc = PngEncoder::write_image(imgenc, &img_resized, w, h, ColorType::Rgb16);
    //image::save_buffer("test.png", &img_resized, w/2, w/2, ExtendedColorType::Bgr16).unwrap();
}

pub fn cropimg(imgname:&str) { //check if center is actually centered
    let mut img = open_image(&imgname);
    let (w, h) = img.dimensions();
    println!("1. Manual     2. Center");
    println!("Choose the cropping mode");
    let cropmode = userinput::answer_to_u8();
    match cropmode {
        1=> {
            println!("Choose the starting point (from left)");
            let startx = userinput::answer_to_u32();
            println!("Choose the starting point (from top)");
            let starty = userinput::answer_to_u32();
            println!("Choose the crop width");
            let cropwidth = userinput::answer_to_u32();
            println!("Choose the crop height");
            let cropheight = userinput::answer_to_u32();
            img = image::DynamicImage::crop(&mut img, startx, starty, cropwidth, cropheight);
        },
        2=> {
            println!("Choose the crop width");
            let cropwidth = userinput::answer_to_u32();
            println!("Choose the crop height");
            let cropheight = userinput::answer_to_u32();
            //fix center calculation
            img = image::DynamicImage::crop(&mut img, (w-cropwidth)/2, (h-cropheight)/2, cropwidth, cropheight);
        },
        _=> {
            println!("Choose a cropping mode!");
        }
    }
    let mut exportname = userinput::remove_extension(&imgname);
    String::push_str(&mut exportname, "_cropped.png");
    img.save(exportname).unwrap();
}
