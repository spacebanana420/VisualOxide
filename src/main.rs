use image::imageops;
use image::{GenericImageView, GenericImage, ColorType, ImageBuffer, DynamicImage, Rgba};
use image::io::Reader;
use std::io;
use std::fs;

fn main() {
    println!("1. Resize     2. Crop");
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
        _=> {
            println!("Chosen operation is incorrect");
            println!("Please choose an operation by typing its name or number");
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

//fn help
//fn batchprocess
//fn transcode
//fn getpixel
//fn generateimage
//fn checkalpha
//fn print

fn icogen(doremy:&str) {
   let img = image::open(doremy.trim()).expect("ag");
   img.save("test.ico");
}

fn testfun(doremy:&str) {
    //let img = image::open(doremy.trim()).expect("ag");
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

fn imageinfo(doremy:&str) {
    let img = image::open(doremy.trim()).expect("ag");
    let (w, h) = img.dimensions();
}

fn resizeimg(doremy:&str) {
    let img = image::open(doremy.trim()).expect("Image file not found in specified directory");
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
            }
        },
        _=> {
            println!("You need to select a scaling mode!");
        },
    }

    let img_resized = image::DynamicImage::resize(&img, rw, rh, imageops::CatmullRom);
    img_resized.save("doremytest.png").unwrap();
    //let imgenc = PngEncoder::write_image(imgenc, &img_resized, w, h, ColorType::Rgb16);
    //image::save_buffer("test.png", &img_resized, w/2, w/2, ExtendedColorType::Bgr16).unwrap();
}

fn cropimg(doremy:&str) {
    let mut img = image::open(doremy.trim()).expect("ag");
    let (w, h) = img.dimensions();
    let img = image::DynamicImage::crop(&mut img, w/4, h/4, w/2, h/2);
    //img.crop(w/4, h/4, w/2, h/2);
    //let img_cropped = img.crop(200, 200, 400, 400);
    img.save("doremytest.png").unwrap();
}
