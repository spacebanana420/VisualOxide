use image::imageops;
//use image::io::Reader;
//use image::io::Reader as ImageReader;
use image::GenericImageView; //use image::GenericImage;
use image::ColorType;
use image::io::Reader;
use std::io;
//use image::Rgb;
use std::fs;

fn main() {
    println!("1. Resize     2. Crop");
    let mut operation = String::new();
    println!("Choose an operation"); io::stdin().read_line(&mut operation);
    let currentdir = fs::read_dir("./").unwrap();
    for dir in currentdir {
        println!("{}", dir.unwrap().path().display());
    }
    let mut inputimg = String::new();
    println!("Input image file"); io::stdin().read_line(&mut inputimg);
    resizeimg(&inputimg);
    /*match operation {
        String::from("1")|String::from("resize")|String::from("Resize")=> resizeimg(&inputimg);
        _=> println!("Chosen operation is incorrect"); println!("Please choose an operation by typing its name or number"),
    }*/
}

//fn help
//fn batchprocess
//fn transcode
//fn getpixel
//fn generateimage
//fn checkalpha
//fn extractinfo
//fn print
//create square of specific pixel values, test loop, test getpixel and putpixel

/*fn to_ico(doremy:String) {
   lut mut img = Reader::open("{doremy}").decode();
        img.save_buffer("test.ico");
}*/

fn imageinfo(doremy:&str) {
    let img = image::open(doremy.trim()).expect("ag");
    let (w, h) = img.dimensions();
}

fn resizeimg(doremy:&str) {
    println!("AAAAAAAAAAA {doremy}");
    //dimensions
    //let mut img = ImageReader::open("{doremy}").expect("doremy").decode();
    let img = image::open(doremy.trim()).expect("ag");
    let (w, h) = img.dimensions();
    let img_resized = image::DynamicImage::resize(&img, w/2, h/2, imageops::CatmullRom);
    img_resized.save("doremytest.png").unwrap();
    //let imgenc = PngEncoder::write_image(imgenc, &img_resized, w, h, ColorType::Rgb16);
    //image::save_buffer("test.png", &img_resized, w/2, w/2, ExtendedColorType::Bgr16).unwrap();
}

fn cropimg(doremy:&str) {
    let mut img = image::open(doremy.trim()).expect("ag");
    let (w, h) = img.dimensions();
    //let img_cropped = image::DynamicImage::crop(&img, w/4, h/4, w/2, h/2);
    img.crop(w/4, h/4, w/2, h/2);
    img.save("doremytest.png").unwrap();
}
