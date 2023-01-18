use image::{GenericImageView, GenericImage, DynamicImage};
use crate::userinput;

fn open_image(filename:&str) -> image::DynamicImage {
    let inputimage = image::open(filename.trim()).expect("Image file not found in specified directory");
    return inputimage;
}

fn brightest_pixel (r:u32, g:u32, b:u32) -> u8 { //needs testing or improvement
    //for the returned value, red is 0, green is 1, blue is 2
    //let (pos_r, pos_g, pos_b):u8 = (1, 2, 3);
    if r < g {
        if r < b {
            if b < g {return 1;} else {return 2;}
        }
        else {return 1;}
    }
    else if r < b {return 2;}
    else {return 0;}
}

pub fn contrast (imgname:&str) { //function is unfinished and will only work for 8 bit for now
    let mut img = open_image(&imgname);
    let mut (width, height) = img.dimensions();
    let mut contrastaddition:u32;
    //let mut newpixel: [u32; 4] = [0; 4];
    let mut newpixel = image::Rgba([0, 0, 0, 0]);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0] as f32; let g = pixel[1] as f32; let b = pixel[2] as f32; let alpha = pixel[3] as f32;
            let brightness:f32 = (r + g + b) / 3.0;
            let brightest = brightest_pixel();
            if brightness > 127 {
                newpixel[0] = r + 20.0;
                newpixel[1] = g + 20.0;
                newpixel[2] = b + 20.0;
                newpixel[3] = alpha;
                DynamicImage::put_pixel(&mut img, x, y, newpixel);
            }
            else if brightness < 127 {

            }
        }
    }
}
